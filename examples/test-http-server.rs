// curl http://localhost:3000/
// curl -X POST http://localhost:3000/oauth2/token --json '{"audience": "z", "grant_type": "client_credentials", "client_id": "a", "client_secret": "b"}' | jq
// return error messages: https://blog.logrocket.com/rust-axum-error-handling/
// would want to sanitize errors (thiserror/snafu), and log/trace the real error

use std::{net::SocketAddr, time::Duration};

use async_trait::async_trait;
use axum::{
    extract::FromRequest,
    http::{Request, StatusCode},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

const TOKEN: &'static str = "some-opaque-token";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let app = Router::new()
        .route("/", get(index))
        .route("/oauth2/token", post(oauth_token))
        .route("/resource", get(resource));

    // axum 0.7 switches to this:
    // let listener = TcpListener::bind("127.0.0.1:8080")
    //     .await
    //     .expect("listener should bind");
    // axum::serve(listener, app).await.expect("axum should start");

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> &'static str {
    "Tokio, world!"
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Oauth2GrantType {
    ClientCredentials,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Oauth2TokenRequest {
    audience: String,
    client_id: String,
    client_secret: String,
    grant_type: Oauth2GrantType,
    #[serde(default)]
    scopes: Vec<String>,
}

#[derive(Debug, Serialize)]
enum TokenType {
    Bearer,
}

#[serde_with::serde_as]
#[derive(Debug, Serialize)]
struct Oauth2TokenResponse {
    access_token: String,
    #[serde_as(as = "serde_with::DurationSeconds<u64>")]
    expires_in: Duration,
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_token: Option<String>,
    token_type: TokenType,
}

// This endpoint just responds with the same thing each time.
#[axum::debug_handler]
async fn oauth_token(
    Json(_payload): Json<Oauth2TokenRequest>,
) -> (StatusCode, Json<Oauth2TokenResponse>) {
    let response = Oauth2TokenResponse {
        access_token: TOKEN.to_owned(),
        expires_in: Duration::from_secs(3600),
        refresh_token: None,
        token_type: TokenType::Bearer,
    };

    (StatusCode::OK, Json(response))
}

struct Authorized;

#[async_trait]
impl<S, B> FromRequest<S, B> for Authorized
where
    S: Send + Sync,
    B: Send + 'static,
{
    type Rejection = StatusCode;

    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(authorization_header) = req.headers().get("Authorization") {
            let valid_token = format!("Bearer {TOKEN}");
            match authorization_header.to_str() {
                Ok(token) if token == valid_token.as_str() => return Ok(Self),
                _ => return Err(StatusCode::UNAUTHORIZED),
            };
        }
        Err(StatusCode::FORBIDDEN)
    }
}

// protected endpoint where the request needs to pass authorization
async fn resource(Authorized: Authorized) -> (StatusCode, Json<Oauth2TokenResponse>) {
    let response = Oauth2TokenResponse {
        access_token: TOKEN.to_owned(),
        expires_in: Duration::from_secs(3600),
        refresh_token: None,
        token_type: TokenType::Bearer,
    };

    (StatusCode::OK, Json(response))
}
