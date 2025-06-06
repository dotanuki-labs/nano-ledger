// Copyright 2025 Dotanuki Labs
// SPDX-License-Identifier: MIT

use crate::SharedState;
use crate::transactions::MovementType;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JournalEntry {
    pub created_at: DateTime<Utc>,
    pub entry_id: Uuid,
    pub transaction_id: Uuid,
    pub account_id: Uuid,
    pub movement_type: MovementType,
    pub amount_in_cents: u64,
}

#[derive(Default)]
pub struct JournalRepository {
    entries: Vec<JournalEntry>,
}

impl JournalRepository {
    pub fn save_entries(&mut self, entries: Vec<JournalEntry>) {
        self.entries.extend(entries);
    }

    pub fn fetch_by_transaction(&self, transaction_id: &Uuid) -> Vec<JournalEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.transaction_id == *transaction_id)
            .cloned()
            .collect()
    }
}

pub async fn entries_for_transaction(
    State(state): State<SharedState>,
    Path(transaction_id): Path<Uuid>,
) -> Result<Json<Vec<JournalEntry>>, StatusCode> {
    let repos = state.read().expect("Cannot acquire shared state");

    let entries = repos.journal.fetch_by_transaction(&transaction_id);

    if entries.is_empty() {
        tracing::debug!("No entries for transaction -> transaction_id = {:?}", &transaction_id);
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(entries))
}
