// Copyright 2025 Dotanuki Labs
// SPDX-License-Identifier: MIT

use crate::SharedState;
use crate::journal::JournalEntry;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum MovementType {
    Debit,
    Credit,
}

impl MovementType {
    fn opposite(&self) -> MovementType {
        match self {
            MovementType::Debit => MovementType::Credit,
            MovementType::Credit => MovementType::Debit,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateNewTransaction {
    movement_type: MovementType,
    lhs_account_id: Uuid,
    rhs_account_id: Uuid,
    description: String,
    amount_in_cents: u64,
}

impl CreateNewTransaction {
    #[cfg(test)]
    pub fn new_debit(from: Uuid, to: Uuid, description: &str, amount: u64) -> Self {
        CreateNewTransaction {
            movement_type: MovementType::Debit,
            lhs_account_id: from,
            rhs_account_id: to,
            description: description.to_string(),
            amount_in_cents: amount,
        }
    }

    #[cfg(test)]
    pub fn new_credit(from: Uuid, to: Uuid, description: &str, amount: u64) -> Self {
        CreateNewTransaction {
            movement_type: MovementType::Credit,
            lhs_account_id: from,
            rhs_account_id: to,
            description: description.to_string(),
            amount_in_cents: amount,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatedTransaction {
    pub created_at: DateTime<Utc>,
    pub transaction_id: Uuid,
}

#[derive(Clone, Debug, Serialize)]
pub struct Transaction {
    pub created_at: DateTime<Utc>,
    pub transaction_id: Uuid,
    pub movement_type: MovementType,
    pub lhs_account_id: Uuid,
    pub rhs_account_id: Uuid,
    pub description: String,
    pub amount_in_cents: u64,
}

#[derive(Default)]
pub struct TransactionsRepository {
    transactions: Vec<Transaction>,
}

impl TransactionsRepository {
    fn save_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    fn fetch_transaction(&self, id: &Uuid) -> Option<&Transaction> {
        self.transactions.iter().find(|tx| tx.transaction_id == *id)
    }
}

pub async fn new_transaction(
    State(state): State<SharedState>,
    Json(payload): Json<CreateNewTransaction>,
) -> Result<Json<CreatedTransaction>, StatusCode> {
    let mut repos = state.write().expect("Cannot acquire shared state");

    let lhs_account_id = &payload.lhs_account_id;
    let rhs_account_id = &payload.rhs_account_id;

    // Validate existing accounts
    let Some(lhs_account) = repos.accounts.fetch_by_id(lhs_account_id) else {
        tracing::debug!("Account not found -> account_id = {:?}", lhs_account_id);
        return Err(StatusCode::NOT_FOUND);
    };

    let Some(rhs_account) = repos.accounts.fetch_by_id(rhs_account_id) else {
        tracing::debug!("Account not found -> account_id = {:?}", rhs_account_id);
        return Err(StatusCode::NOT_FOUND);
    };

    let amount_to_move = payload.amount_in_cents;

    // Validate sufficient balance and update
    match &payload.movement_type {
        MovementType::Debit => {
            if lhs_account.balance < amount_to_move {
                tracing::debug!("Insufficient balance -> account_id = {:?}", lhs_account_id);
                return Err(StatusCode::CONFLICT);
            } else {
                lhs_account.subtract_balance(amount_to_move);
                rhs_account.add_balance(amount_to_move)
            }
        },
        MovementType::Credit => {
            if rhs_account.balance < amount_to_move {
                tracing::debug!("Insufficient balance -> account_id = {:?}", rhs_account);
                return Err(StatusCode::CONFLICT);
            } else {
                rhs_account.subtract_balance(amount_to_move);
                lhs_account.add_balance(amount_to_move)
            }
        },
    }

    // Create a transaction record
    let transaction_id = Uuid::new_v4();
    let created_at = Utc::now();

    let tx = Transaction {
        transaction_id,
        created_at,
        movement_type: payload.movement_type,
        lhs_account_id: payload.lhs_account_id,
        rhs_account_id: payload.rhs_account_id,
        description: payload.description,
        amount_in_cents: payload.amount_in_cents,
    };

    // Create double-entries
    let left_entry = JournalEntry {
        created_at: Utc::now(),
        entry_id: Uuid::new_v4(),
        transaction_id: tx.transaction_id,
        account_id: tx.lhs_account_id,
        movement_type: tx.movement_type,
        amount_in_cents: tx.amount_in_cents,
    };

    let right_entry = JournalEntry {
        account_id: tx.rhs_account_id,
        movement_type: tx.movement_type.opposite(),
        ..left_entry.clone()
    };

    // Store results
    repos.journal.save_entries(vec![left_entry, right_entry]);
    repos.transactions.save_transaction(tx);

    // Return status
    let tx = CreatedTransaction {
        created_at,
        transaction_id,
    };

    tracing::debug!("Transaction created -> {:?}", tx);
    Ok(Json(tx))
}

pub async fn transaction_details(
    State(state): State<SharedState>,
    Path(transaction_id): Path<Uuid>,
) -> Result<Json<Transaction>, StatusCode> {
    let repos = &state.read().expect("Cannot acquire shared state");

    let existing = repos.transactions.fetch_transaction(&transaction_id);

    let Some(account) = existing.cloned() else {
        tracing::debug!("Not found -> account_id = {:?}", &transaction_id);
        return Err(StatusCode::NOT_FOUND);
    };

    Ok(Json(account))
}
