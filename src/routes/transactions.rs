use crate::MARKET_DATA;
use rocket_contrib::json::Json;

#[derive(Deserialize, Serialize)]
pub enum PurchaseResult {
    Success(String),
    Fail(String),
}

#[get("/buy?<market_id>&<_user_id>&<quantity>")]
pub fn purchase(market_id: String, _user_id: String, quantity: u32) -> Json<PurchaseResult> {
    let md = MARKET_DATA.lock().unwrap();

    // get info about user

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

    if quantity == 0 {
        return Json(PurchaseResult::Fail("Cannot purchase 0 of an item".into()));
    } else if item.quantity == 0 {
        return Json(PurchaseResult::Fail(format!("Item '{}' out of stock", item.item_id)));
    } else if quantity > item.quantity {
        return Json(PurchaseResult::Fail("Cannot purchase over amount of stock".into()));
    }

    item.quantity -= quantity;

    *md.items.get_mut(&market_id).unwrap() = item.clone();

    // remove amount of money from user

    // ----

    Json(PurchaseResult::Success(format!(
        "Bought {} {}(s) for {} EO each!",
        quantity, item.display_name, item.price
    )))
}
