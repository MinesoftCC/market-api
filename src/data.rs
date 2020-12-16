use chrono::{DateTime, Utc};
use std::{borrow::Cow, default::Default, fs};

type Str = Cow<'static, str>;

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
    pub item_id: Str,
    pub item_image_url: Str,
    pub display_name: Str,
    pub quantity: u32,
    pub price: u32,
    pub poster_id: u16,
    pub time_posted: DateTime<Utc>,
}

impl Default for Item {
    fn default() -> Self {
        Self {
            item_id: "null".into(),
            item_image_url: "null".into(),
            display_name: "null".into(),
            quantity: 0,
            price: 0,
            poster_id: 1,
            time_posted: Utc::now(),
        }
    }
}
