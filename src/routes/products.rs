use crate::{Item, MarketData, MARKET_DATA};
use rocket_contrib::json::Json;

#[get("/get")]
pub fn all_products() -> Json<MarketData> {
    let mut md = MARKET_DATA.lock().unwrap().clone();
    md.update();
    Json(md)
}

#[get("/get/by_name/<item_name>")]
pub fn get_products_by_name(item_name: String) -> Json<Vec<Item>> {
    let mut md = MARKET_DATA.lock().unwrap().clone();
    md.update();

    Json(
        md.items
            .into_iter()
            .filter(|item| item.display_name == item_name)
            .collect(),
    )
}

#[get("/get/by_id/<item_id>")]
pub fn get_products_by_id(item_id: String) -> Json<Vec<Item>> {
    let mut md = MARKET_DATA.lock().unwrap().clone();
    md.update();

    Json(md.items.into_iter().filter(|item| item.item_id == item_id).collect())
}

#[get("/get/below_price/<price>")]
pub fn get_products_under_price(price: u32) -> Json<Vec<Item>> {
    let mut md = MARKET_DATA.lock().unwrap().clone();
    md.update();

    Json(md.items.into_iter().filter(|item| item.price <= price).collect())
}

#[get("/get/above_price/<price>")]
pub fn get_products_above_price(price: u32) -> Json<Vec<Item>> {
    let mut md = MARKET_DATA.lock().unwrap().clone();
    md.update();

    Json(md.items.into_iter().filter(|item| item.price >= price).collect())
}

#[get("/get/at_price/<price>")]
pub fn get_products_at_price(price: u32) -> Json<Vec<Item>> {
    let mut md = MARKET_DATA.lock().unwrap().clone();
    md.update();

    Json(md.items.into_iter().filter(|item| item.price == price).collect())
}

#[post("/add_item", format = "application/json", data = "<item>")]
pub fn add_item(item: Item) {
    let mut md = MARKET_DATA.lock().unwrap().clone();
    md.add_item(item);
}

#[post("/remove_item", format = "application/json", data = "<item>")]
pub fn remove_item(item: Item) {
    let mut md = MARKET_DATA.lock().unwrap().clone();
    md.remove_item(item);
}
