use dashmap::DashMap;
use reqwest::blocking::*;
use std::env;

pub struct Bank {
    pub users: Vec<User>,
}

impl Bank {
    pub fn connect() -> Self {
        let bank_api = format!("http://{}:80/BankApi", env::var("BANK_IP").unwrap());

        let client = Client::new();
        let response: Vec<String> = serde_json::from_str(
            client
                .get(format!("{}/listusers", bank_api).as_str())
                .send()
                .unwrap()
                .text()
                .unwrap()
                .as_str(),
        )
        .unwrap();

        let mut users = Vec::new();

        response.into_iter().for_each(|username| {
            users.push(User {
                name: username,
                balance: 0,
                perm_count: 0,
                accounts: None,
            })
        });

        users.iter_mut().enumerate().into_iter().for_each(|(id, user)| {
            user.populate(id);
        });

        Self { users }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Account {
    balance: i32,
    clearance: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    balance: i32,
    name: String,
    perm_count: i32,
    accounts: Option<DashMap<String, Account>>,
}

impl User {
    pub fn populate(&mut self, id: usize) {
        let bank_api = format!("http://{}:80/BankApi", env::var("BANK_IP").unwrap());

        let client = Client::new();
        let User {
            balance, perm_count, ..
        } = serde_json::from_str(
            client
                .get(format!("{}/total/{}", bank_api, id).as_str())
                .send()
                .unwrap()
                .text()
                .unwrap()
                .as_str(),
        )
        .unwrap();

        let accounts: Option<DashMap<String, Account>> = serde_json::from_str(
            client
                .get(format!("{}/listaccounts/{}", bank_api, id).as_str())
                .send()
                .unwrap()
                .text()
                .unwrap()
                .as_str(),
        )
        .unwrap();

        self.balance = balance;
        self.perm_count = perm_count;
        self.accounts = accounts;
    }
}
