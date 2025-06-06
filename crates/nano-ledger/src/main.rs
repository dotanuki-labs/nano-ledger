// Copyright 2025 Dotanuki Labs
// SPDX-License-Identifier: MIT

mod accounts;

use crate::accounts::AccountsRepository;
use axum::Router;
use axum::routing::{get, post};
use std::sync::{Arc, RwLock};
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

type SharedState = Arc<RwLock<Repositories>>;

#[derive(Default)]
struct Repositories {
    pub accounts: AccountsRepository,
}

fn app(state: SharedState) -> Router {
    Router::new()
        .route("/accounts/new", post(accounts::new_account))
        .route("/accounts/{account_id}", get(accounts::account_details))
        .with_state(state)
}

#[tokio::main]
async fn main() {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into());

    let binding_address = match std::env::var("DOCKER_CONTAINER_HOST") {
        Ok(_) => "0.0.0.0:3000",
        Err(_) => "127.0.0.1:3000",
    };

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    let listener = TcpListener::bind(binding_address)
        .await
        .expect("cannot bind to local port");

    tracing::debug!("Listening on {}", binding_address);

    let shared_state = SharedState::default();

    axum::serve(listener, app(shared_state))
        .await
        .expect("failed to run server");
}

#[cfg(test)]
mod tests {
    use crate::accounts::{Account, AccountsRepository, CreateNewAccount};
    use crate::{Repositories, SharedState, app};
    use axum::body::Body;
    use http::{Method, Request, StatusCode, header};
    use serde::Serialize;
    use serde_json::json;
    use std::sync::{Arc, RwLock};
    use tower::ServiceExt;
    use uuid::Uuid;

    fn post_request(endpoint: &str, payload: impl Serialize) -> Request<Body> {
        Request::builder()
            .method(Method::POST)
            .header(header::CONTENT_TYPE, "application/json")
            .uri(endpoint)
            .body(Body::from(json!(payload).to_string()))
            .unwrap()
    }

    fn get_request(endpoint: &str) -> Request<Body> {
        Request::builder()
            .method(Method::GET)
            .header(header::CONTENT_TYPE, "application/json")
            .uri(endpoint)
            .body(Body::empty())
            .unwrap()
    }

    #[tokio::test]
    async fn should_report_account_not_found() {
        // Given
        let shared_state = SharedState::default();
        let app = app(shared_state);

        // When
        let get_account = format!("/accounts/{}", Uuid::new_v4());
        let request = get_request(&get_account);
        let response = app.oneshot(request).await.unwrap();

        // Then
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn should_create_new_account_with_success() {
        // Given
        let shared_state = SharedState::default();
        let app = app(shared_state);

        // When
        let new_account = json!(CreateNewAccount {
            alias: "ufs.main".to_string(),
            balance: Some(100000)
        });

        let request = post_request("/accounts/new", new_account);
        let response = app.oneshot(request).await.unwrap();

        // Then
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn should_not_create_account_with_existing_alias() {
        // Given
        let existing_account = Account::new("ufs.savings", 10000);

        let accounts_repository = AccountsRepository {
            accounts: vec![existing_account.clone()],
        };

        let repos = Repositories {
            accounts: accounts_repository,
        };

        let shared_state = Arc::new(RwLock::new(repos));
        let app = app(shared_state);

        // When
        let new_account = json!(CreateNewAccount {
            alias: "ufs.savings".to_string(),
            balance: None
        });

        let request = post_request("/accounts/new", new_account);
        let response = app.oneshot(request).await.unwrap();

        // Then
        assert_eq!(response.status(), StatusCode::CONFLICT);
    }
}
