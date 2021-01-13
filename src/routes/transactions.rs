use crate::{bank::data::*, MARKET_DATA};
use rocket_contrib::json::Json;

#[derive(Deserialize, Serialize)]
pub enum PurchaseResult {
    Success(String),
    Fail(String),
}

#[get("/buy/<market_id>/<user_id>/<account_name>/<password>/<quantity>")]
pub fn purchase(
    market_id: String,
    user_id: usize,
    account_name: String,
    password: String,
    quantity: u32,
) -> Json<PurchaseResult> {
    let md = MARKET_DATA.lock().unwrap();

    // get info about user
    let mut bank = Bank::connect();
    let customer = match bank.get_user_mut(user_id) {
        Some(u) => u,
        None =>
            return Json(PurchaseResult::Fail(format!(
                "Could not find user with ID '{}'",
                user_id
            ))),
    }
    .clone();
    // ----

    let mut item = match md.items.get(&market_id) {
        Some(i) => i,
        None =>
            return Json(PurchaseResult::Fail(format!(
                "Could not find item with market ID '{}'",
                market_id
            ))),
    }
    .clone();

    let seller = match bank.get_user_mut(item.poster_id as usize) {
        Some(u) => u,
        None =>
            return Json(PurchaseResult::Fail(format!(
                "Could not find seller user with ID '{}'",
                item.poster_id
            ))),
    };

    if quantity == 0 {
        return Json(PurchaseResult::Fail("Cannot purchase 0 of an item".into()));
    } else if item.quantity == 0 {
        return Json(PurchaseResult::Fail(format!(
            "Item '{}' out of stock",
            item.item_id
        )));
    } else if quantity > item.quantity {
        return Json(PurchaseResult::Fail(
            "Cannot purchase over amount of stock".into(),
        ));
    }

    let account = match customer.accounts.clone() {
        Some(dm) => dm.get(&account_name).unwrap().clone(),
        None =>
            return Json(PurchaseResult::Fail(format!(
                "Could not find account with name '{}' for user '{}'",
                account_name, customer.name
            ))),
    };

    if quantity * item.price > account.balance as u32 {
        return Json(PurchaseResult::Fail(format!(
            "Total cost for purchasing {} {}(s) exceeds the '{}' balance.",
            quantity, item.display_name, account_name
        )));
    }

    item.quantity -= quantity;

    *md.items.get_mut(&market_id).unwrap() = item.clone();

    // remove amount of money from user

    let seller_account = seller.get_default_account().unwrap();

    if let Err(e) = bank.send_funds(
        user_id,
        account_name,
        item.poster_id as usize,
        seller_account,
        (quantity * item.price) as i32,
        password,
    ) {
        return Json(PurchaseResult::Fail(format!("Could not send funds: {}", e)));
    };

    if let Err(e) = bank.update_user(user_id, customer.clone()) {
        return Json(PurchaseResult::Fail(format!(
            "Could not update local user: {}",
            e
        )));
    }

    // ----

    Json(PurchaseResult::Success(format!(
        "Bought {} {}(s) for {} EO each!",
        quantity, item.display_name, item.price
    )))
}
