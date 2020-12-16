use crate::{MarketData, MARKET_DATA};
use rocket_contrib::json::Json;

#[get("/")]
pub fn all_products() -> Json<MarketData> {
    let mut md = MARKET_DATA.lock().unwrap().clone();
    md.update();
    Json(md)
}
