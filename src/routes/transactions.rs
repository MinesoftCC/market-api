use crate::{bank::data::*, data::ItemRatio, MARKET_DATA};
use rocket_contrib::json::Json;

#[derive(Deserialize, Serialize)]
pub enum PurchaseResult {
    Success(String),
    Fail(String),
}

#[post(
    "/buy/<market_id>/<user_id>/<account_name>/<diamond_amount>",
    format = "application/json",
    data = "<password>"
)]
pub fn purchase(
    market_id: String,
    user_id: usize,
    account_name: String,
    diamond_amount: u32,
    password: Json<String>,
) -> Json<PurchaseResult> {
    let md = MARKET_DATA.lock().unwrap();

    let password = password.to_string();

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

    let sixteen_stack_ids = [
        "minecraft:snowball",
        "minecraft:bucket",
        "minecraft:egg",
        "minecraft:oak_sign",
        "minecraft:spruce_sign",
        "minecraft:birch_sign",
        "minecraft:jungle_sign",
        "minecraft:acacia_sign",
        "minecraft:dark_oak_sign",
        "minecraft:crimson_sign",
        "minecraft:warped_sign",
        "minecraft:ender_pearl",
        "minecraft:honey_bottle",
    ];

    let quantity = match item.item_ratio {
        ItemRatio::Pair => diamond_amount * 2,
        ItemRatio::HalfStack =>
            if sixteen_stack_ids.contains(&&*item.item_id) {
                diamond_amount * 8
            } else {
                diamond_amount * 32
            },
        ItemRatio::Stack =>
            if sixteen_stack_ids.contains(&&*item.item_id) {
                diamond_amount * 16
            } else {
                diamond_amount * 64
            },
        ItemRatio::Custom(amt) => diamond_amount * amt,
        _ => diamond_amount,
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
        "Bought {} {}(s) for {} diamonds!",
        quantity,
        item.display_name,
        (quantity * item.price)
    )))
}
