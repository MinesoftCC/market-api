use crate::{Item, MARKET_DATA};
use dashmap::DashMap;
use rocket_contrib::json::Json;

type Ret = Vec<(String, Item)>;

#[get("/get")]
pub fn all_products() -> Json<DashMap<String, Item>> {
    let items = MARKET_DATA.try_lock().unwrap().items.clone();
    Json(items)
}

#[get("/get/by_name/<item_name>")]
pub fn get_products_by_name(item_name: String) -> Json<Ret> {
    let md = MARKET_DATA.lock().unwrap();

    Json(
        md.items
            .clone()
            .into_iter()
            .filter(|item| item.1.display_name == item_name)
            .map(|item| item)
            .collect(),
    )
}

#[get("/get/by_id/<item_id>")]
pub fn get_products_by_id(item_id: String) -> Json<Ret> {
    let md = MARKET_DATA.lock().unwrap();

    Json(
        md.items
            .clone()
            .into_iter()
            .filter(|item| item.1.item_id == item_id)
            .collect(),
    )
}

#[get("/get/below_price/<price>")]
pub fn get_products_under_price(price: u32) -> Json<Ret> {
    let md = MARKET_DATA.lock().unwrap();

    Json(
        md.items
            .clone()
            .into_iter()
            .filter(|item| item.1.price <= price)
            .collect(),
    )
}

#[get("/get/above_price/<price>")]
pub fn get_products_above_price(price: u32) -> Json<Ret> {
    let md = MARKET_DATA.lock().unwrap();

    Json(
        md.items
            .clone()
            .into_iter()
            .filter(|item| item.1.price >= price)
            .collect(),
    )
}

#[get("/get/at_price/<price>")]
pub fn get_products_at_price(price: u32) -> Json<Ret> {
    let md = MARKET_DATA.lock().unwrap();

    Json(
        md.items
            .clone()
            .into_iter()
            .filter(|item| item.1.price == price)
            .collect(),
    )
}

#[post("/add_item", format = "application/json", data = "<item>")]
pub fn add_item(item: Item) {
    let mut md = MARKET_DATA.lock().unwrap();
    md.add_item(item);
}

#[post("/remove_item", format = "application/json", data = "<item>")]
pub fn remove_item(item: Item) {
    let mut md = MARKET_DATA.lock().unwrap();
    md.remove_item(item);
}
