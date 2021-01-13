use dashmap::DashMap;
use reqwest::blocking::*;
use std::{borrow::Cow, env};

#[derive(Deserialize, Serialize)]
struct FundResponse {
    pub content: String,
    pub value: i32,
}

#[derive(Clone)]
pub struct Bank {
    users: Vec<User>,
}

impl Bank {
    pub fn connect() -> Self {
        let bank_api =
            format!("http://{}:80/BankApi", env::var("BANK_IP").unwrap());

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

        users
            .iter_mut()
            .enumerate()
            .into_iter()
            .for_each(|(id, user)| {
                user.populate(id);
            });

        Self { users }
    }

    pub fn get_user_mut(&mut self, user_id: usize) -> Option<&mut User> {
        self.users.get_mut(user_id)
    }

    pub fn get_user(&mut self, user_id: usize) -> User {
        self.users.get(user_id).unwrap().clone()
    }

    pub fn update_user(
        &mut self,
        user_id: usize,
        user_updated: User,
    ) -> Result<(), Cow<'static, str>> {
        let user = match self.users.get_mut(user_id) {
            Some(u) => u,
            None =>
                return Err(format!(
                    "Could not find user with ID '{}'",
                    user_id
                )
                .into()),
        };

        *user = user_updated;

        Ok(())
    }

    pub fn send_funds(
        &mut self,
        from_id: usize,
        from_account: String,
        to_id: usize,
        to_account: String,
        amount: i32,
        from_pass: String,
    ) -> Result<(), Cow<'static, str>> {
        let mut from_user = self.get_user(from_id);

        let from_accounts = match from_user.clone().accounts {
            Some(accts) => accts,
            None =>
                return Err(format!(
                    "Could not find any accounts for user '{}'",
                    from_user.name
                )
                .into()),
        };

        let from_account = match from_accounts.get_mut(&from_account) {
            Some(acct) => acct.key().clone(),
            None =>
                return Err(format!(
                    "Could not find account '{}' for user '{}'",
                    from_account, from_user.name
                )
                .into()),
        };

        let mut to_user = self.get_user(to_id);

        let to_accounts = match to_user.clone().accounts {
            Some(accts) => accts,
            None =>
                return Err(format!(
                    "Could not find any accounts for user '{}'",
                    to_user.name
                )
                .into()),
        };

        let to_account = match to_accounts.get_mut(&to_account) {
            Some(acct) => acct.key().clone(),
            None =>
                return Err(format!(
                    "Could not find account '{}' for user '{}'",
                    to_account, to_user.name
                )
                .into()),
        };

        from_user
            .remove_balance(from_account.clone(), amount)
            .unwrap();
        to_user.add_balance(to_account.clone(), amount).unwrap();

        self.update_user(from_id, from_user).unwrap();
        self.update_user(to_id, to_user).unwrap();

        let bank_api =
            format!("http://{}:80/BankApi", env::var("BANK_IP").unwrap());
        let client = Client::new();
        let response: FundResponse = serde_json::from_str(
            client
                .post(
                    format!(
                        "{}/sendfunds/{}/{}/{}/{}/{}",
                        bank_api,
                        from_account,
                        to_account,
                        amount,
                        from_id,
                        from_pass
                    )
                    .as_str(),
                )
                .send()
                .unwrap()
                .text()
                .unwrap()
                .as_str(),
        )
        .unwrap();

        if response.value == 0 {
            return Err(response.content.into());
        }

        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Account {
    pub balance: i32,
    pub clearance: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub balance: i32,
    pub name: String,
    pub perm_count: i32,
    pub accounts: Option<DashMap<String, Account>>,
}

impl User {
    pub fn populate(&mut self, id: usize) {
        let bank_api =
            format!("http://{}:80/BankApi", env::var("BANK_IP").unwrap());

        let client = Client::new();
        let User {
            balance,
            perm_count,
            ..
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

    fn remove_balance(
        &mut self,
        account_name: String,
        amount: i32,
    ) -> Result<(), Cow<'static, str>> {
        let accounts = match self.accounts.as_mut() {
            Some(accts) => accts,
            None =>
                return Err("User doesn't have any assigned accounts".into()),
        };

        let mut account = match accounts.get_mut(&account_name) {
            Some(acct) => acct,
            None =>
                return Err(format!(
                    "User doesn't have an account with the name '{}'",
                    account_name
                )
                .into()),
        };

        account.balance -= amount;

        Ok(())
    }

    fn add_balance(
        &mut self,
        account_name: String,
        amount: i32,
    ) -> Result<(), Cow<'static, str>> {
        let accounts = match self.accounts.as_mut() {
            Some(accts) => accts,
            None =>
                return Err("User doesn't have any assigned accounts".into()),
        };

        let mut account = match accounts.get_mut(&account_name) {
            Some(acct) => acct,
            None =>
                return Err(format!(
                    "User doesn't have an account with the name '{}'",
                    account_name
                )
                .into()),
        };

        account.balance += amount;

        Ok(())
    }

    pub fn get_default_account(&self) -> Result<String, String> {
        let accounts = match self.accounts.clone() {
            Some(accts) => accts,
            None => return Err("User doesn't have any accounts".into()),
        };

        let accounts: Vec<String> =
            accounts.into_iter().map(|item| item.0).collect();

        Ok(accounts[0].clone())
    }
}
