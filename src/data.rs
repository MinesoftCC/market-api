use chrono::prelude::*;
use dashmap::DashMap;
use rocket::{
    data::{FromDataSimple, Outcome},
    http::{ContentType, Status},
    Data, Request,
};
use std::{collections::hash_map::DefaultHasher, default::Default, fs, hash::*, io::Read};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarketData {
    pub items: DashMap<String, Item>,
}

impl MarketData {
    pub fn write(&self) {
        let data = serde_json::to_string_pretty(self).unwrap();
        fs::write("data/data.json", data).unwrap();
    }

    pub fn add_item(&mut self, item: Item) { self.items.insert(Item::gen_market_id(&item), item); }

    pub fn remove_item(&mut self, item: Item) {
        self.items = self.items.clone().into_iter().filter(|x| x.1 != item).collect();
        println!("Item req: {:#?}", item);
        println!("Items\n{:#?}", self.items);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, FromForm, PartialEq, Hash)]
pub struct Item {
    pub item_id: String,
    pub item_image_url: String,
    pub display_name: String,
    pub quantity: u32,
    pub price: u32,
    pub poster_id: u16,
    pub time_posted: String,
}

impl Item {
    pub fn gen_market_id(item: &Item) -> String {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);

        format!("{:x}", hasher.finish())
    }
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
            time_posted: Utc::now().to_rfc2822(),
        }
    }
}

impl FromDataSimple for Item {
    type Error = String;

    fn from_data(request: &Request, data: Data) -> Outcome<Self, Self::Error> {
        if request.content_type() != Some(&ContentType::JSON) {
            return Outcome::Failure((Status::UnsupportedMediaType, "Data must be in JSON format".into()));
        }

        let mut string = String::new();
        if let Err(e) = data.open().take(1024).read_to_string(&mut string) {
            return Outcome::Failure((Status::BadRequest, format!("{:?}", e)));
        }

        let item: Item = match serde_json::from_str(&string.as_str()) {
            Ok(i) => i,
            Err(e) => return Outcome::Failure((Status::BadRequest, format!("{:?}", e))),
        };

        Outcome::Success(item)
    }
}
