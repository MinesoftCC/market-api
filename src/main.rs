#![feature(proc_macro_hygiene, decl_macro)]

mod data;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde;

use crate::data::*;
use dotenv::dotenv;
use std::{borrow::Cow, fs::File, io::prelude::*, path::Path, sync::Mutex};

type StrRet = Cow<'static, str>;

lazy_static! {
    static ref MARKET_DATA: Mutex<MarketData> = Mutex::from(MarketData { items: Vec::new() });
}

fn check_file() -> Result<(), StrRet> {
    if !Path::new("data/data.json").exists() {
        if let Err(e) = File::create("data/data.json") {
            return Err(format!("Could not create data.json: {}", e).into());
        }

        let mut file = File::create("data/data.json").unwrap();
        let data = serde_json::to_string(&*MARKET_DATA.lock().unwrap()).unwrap();
        file.write_all(data.as_bytes()).unwrap();
    }

    let mut file = File::open("data/data.json").unwrap();
    let mut buffer = vec![];
    file.read_to_end(&mut buffer).unwrap();
    let data = String::from_utf8(buffer.to_vec()).unwrap();

    *MARKET_DATA.lock().unwrap() = serde_json::from_str(&data).unwrap();

    Ok(())
}

fn main() {
    dotenv().ok();

    if let Err(e) = check_file() {
        eprintln!("An error occured: {}", e);
        return;
    };

    rocket::ignite().mount("/", routes![]).launch();
}
