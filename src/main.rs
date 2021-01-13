#![feature(proc_macro_hygiene, decl_macro)]
#![allow(clippy::unit_arg)]

mod bank;
mod data;
mod routes;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;

use crate::data::*;
use dashmap::DashMap;
use dotenv::dotenv;
use rocket::Request;
use rocket_contrib::json::Json;
use std::{
    borrow::Cow,
    fs::{self, OpenOptions},
    io::prelude::*,
    sync::Mutex,
};

type StrRet = Cow<'static, str>;

lazy_static! {
    static ref MARKET_DATA: Mutex<MarketData> = Mutex::from(MarketData {
        items: {
            let dm = DashMap::new();
            let item = Item::default();
            dm.insert(Item::gen_market_id(&item), item);
            dm
        }
    });
}

#[derive(Deserialize, Serialize)]
struct ErrorResponse {
    code: u32,
    message: String,
}

#[catch(400)]
fn bad_request(req: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        code: 400,
        message: format!("Recieved bad request:<br />{:#?}", req).replace("\n", "<br />"),
    })
}

#[catch(404)]
fn not_found(req: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        code: 404,
        message: format!("'{}' not found", req.uri()),
    })
}

#[catch(500)]
fn internal_server_error(req: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        code: 500,
        message: format!(
            "Welp, it looks like either the server broke on it's own or you somehow \
             managed to break the server by yourself. You'll be hearing from our \
             lawyers.<br /><br />Just kidding, we can't afford lawyers.<br />The \
             following caused an internal server error: '{:#?}'.",
            req
        )
        .replace("\n", "<br />"),
    })
}

fn check_file() -> Result<(), StrRet> {
    let mut file = match OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open("data/data.json")
    {
        Ok(f) => f,
        Err(e) => return Err(format!("Could not find/create data.json: {}", e).into()),
    };

    let mut buffer = vec![];
    file.read_to_end(&mut buffer).unwrap();
    let mut data = String::from_utf8(buffer.to_vec()).unwrap();
    let mut market_data = MARKET_DATA.lock().unwrap();

    if data.is_empty() {
        data = serde_json::to_string_pretty(&*market_data).unwrap();
        buffer = data.as_bytes().to_vec();
        file.write_all(&buffer).unwrap();
    }

    file.read_to_end(&mut buffer).unwrap();

    *market_data = match serde_json::from_str(&data) {
        Ok(md) => md,
        Err(_) => {
            data = serde_json::to_string_pretty(&*market_data).unwrap();
            buffer = data.as_bytes().to_vec();

            fs::write("data/data.json", &mut buffer).unwrap();
            file.read_to_end(&mut buffer).unwrap();

            serde_json::from_str(&data).unwrap()
        },
    };

    Ok(())
}

fn main() {
    dotenv().ok();

    // handle Ctrl+C gracefully by saving data before closing.
    ctrlc::set_handler(|| {
        println!("\n--| Stopping server |--");
        print!("Saving market data...");
        MARKET_DATA.lock().unwrap().write();
        println!("Done!");
        std::process::exit(0);
    })
    .unwrap();

    if let Err(e) = check_file() {
        eprintln!("An error occured: {}", e);
        return;
    };

    rocket::ignite()
        .mount(
            "/",
            routes![
                // products
                routes::products::all_products,
                routes::products::get_products_by_name,
                routes::products::get_products_by_id,
                routes::products::get_products_under_price,
                routes::products::get_products_above_price,
                routes::products::get_products_at_price,
                routes::products::add_item,
                routes::products::remove_item,
                // purchase
                routes::transactions::purchase,
            ],
        )
        .register(catchers![bad_request, not_found, internal_server_error])
        .launch();
}
