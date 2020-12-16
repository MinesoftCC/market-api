use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarketData {
    pub items: Vec<Item>,
}

impl MarketData {
    pub fn update(&mut self) {
        *self = serde_json::from_str(&String::from_utf8(fs::read("data/data.json").unwrap()).unwrap()).unwrap();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub item_id: String,
    pub display_name: String,
    pub quantity: u32,
    pub price: u32,
    pub poster_id: u16,
    pub time_posted: DateTime<Utc>,
}
