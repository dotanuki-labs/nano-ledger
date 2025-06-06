// Copyright 2025 Dotanuki Labs
// SPDX-License-Identifier: MIT

use crate::SharedState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Error, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNewAccount {
    pub alias: String,
    pub balance: Option<u64>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Account {
    pub account_id: Uuid,
    pub alias: String,
    pub balance: u64,
}

impl Account {
    #[cfg(test)]
    pub fn new(alias: &str, balance: u64) -> Self {
        Account {
            account_id: Uuid::new_v4(),
            alias: alias.to_string(),
            balance,
        }
    }

    pub fn add_balance(&self, amount: u64) {
        self.balance.checked_add(amount).unwrap();
    }

    pub fn subtract_balance(&self, amount: u64) {
        self.balance.checked_sub(amount).unwrap();
    }
}

#[derive(Default)]
pub struct AccountsRepository {
    pub accounts: Vec<Account>,
}

impl AccountsRepository {
    pub fn save_account(&mut self, account: Account) -> Result<(), Error> {
        let None = self.fetch_by_alias(&account.alias) else {
            tracing::debug!("Alias already taken | alias = {:?}", &account.alias);
            return Err(Error::new("Alias already taken by another account"));
        };

        self.accounts.push(account);
        Ok(())
    }

    pub fn fetch_by_id(&self, account_id: &Uuid) -> Option<&Account> {
        self.accounts.iter().find(|&a| a.account_id == *account_id)
    }

    pub fn fetch_by_alias(&self, alias: &str) -> Option<&Account> {
        self.accounts.iter().find(|&a| a.alias == alias)
    }
}

pub async fn new_account(
    State(state): State<SharedState>,
    Json(payload): Json<CreateNewAccount>,
) -> Result<Json<Account>, StatusCode> {
    let new_account = Account {
        account_id: Uuid::new_v4(),
        alias: payload.alias.clone(),
        balance: payload.balance.unwrap_or_default(),
    };

    let mut shared_state = state.write().expect("Cannot acquire shared state");

    tracing::debug!("Creating | alias = {:?}", &payload.alias);
    let saved = shared_state.accounts.save_account(new_account.clone());
    tracing::debug!("Created | account_id = {:?}", &new_account.account_id);

    match saved {
        Ok(_) => Ok(Json(new_account)),
        Err(_) => Err(StatusCode::CONFLICT),
    }
}

pub async fn account_details(
    State(state): State<SharedState>,
    Path(account_id): Path<Uuid>,
) -> Result<Json<Account>, StatusCode> {
    let repos = &state.read().expect("Cannot acquire shared state");

    let existing = repos.accounts.fetch_by_id(&account_id);

    let Some(account) = existing.cloned() else {
        tracing::debug!("Not found | account_id = {:?}", &account_id);
        return Err(StatusCode::NOT_FOUND);
    };

    Ok(Json(account))
}
