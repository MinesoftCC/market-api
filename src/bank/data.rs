use dashmap::DashMap;
use reqwest::blocking::*;
use std::env;

pub struct Bank {
    pub users: DashMap<String, User>,
}

impl Bank {
    pub fn connect() -> Self {
        let client = Client::new();

        Self { users: DashMap::new() }
    }
}

pub struct User {}
