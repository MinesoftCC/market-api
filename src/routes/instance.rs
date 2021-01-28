use rocket_contrib::json::Json;
use std::env;

#[derive(Serialize)]
pub struct BankIp {
    pub ip: String,
    pub port: u32,
}

#[get("/get_bank_ip")]
pub fn get_bank_ip() -> Json<BankIp> {
    Json(BankIp {
        ip: env::var("BANK_IP").unwrap(),
        port: env::var("BANK_PORT")
            .unwrap_or("80".to_string())
            .parse::<u32>()
            .unwrap(),
    })
}
