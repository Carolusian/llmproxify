use core::fmt;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use url::{ParseError, Url};

use axum::{
    body::{self, Bytes, Body},
    extract::{Path, Request, State},
    http::{Method, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};

const DEFAULT_PROVIDERS: &[(&str, &str)] = &[
    ("openai", "https://api.openai.com/"),
    ("groq", "https://api.groq.com/"),
    ("cerebras", "https://api.cerebras.ai/"),
    ("gemini", "https://generativelanguage.googleapis.com/"),
    ("sambanova", "https://api.sambanova.ai/"),
];

#[derive(Debug)]
enum AppError {
    UrlParseError(ParseError),
    RequestError(reqwest::Error),
    SerdeJsonError(serde_json::Error),
    Other(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::UrlParseError(e) => write!(f, "URL parse error: {}", e),
            AppError::RequestError(e) => write!(f, "Request error: {}", e),
            AppError::SerdeJsonError(e) => {
                write!(f, "JSON serialization/deserialization error: {}", e)
            }
            AppError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<ParseError> for AppError {
    fn from(e: ParseError) -> Self {
        AppError::UrlParseError(e)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        AppError::RequestError(e)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::SerdeJsonError(e)
    }
}

impl From<String> for AppError {
    fn from(msg: String) -> Self {
        AppError::Other(msg)
    }
}

#[tokio::main]
async fn main() {
    // Load api providers
    let api_providers = Arc::new(load_env_providers());
    let app = Router::new()
        .route("/", get(index))
        .route("/{provider}/{*rest}", get(handler).post(handler))
        .with_state(api_providers);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> &'static str {
    "Hello, llmproxify!"
}

async fn handler(
    Path(params): Path<Params>,
    State(api_providers): State<Arc<HashMap<String, String>>>,
    req: Request,
) -> impl IntoResponse {
    let url = get_upstream_url(&req, &params.provider, &params.rest, &api_providers);
    let full_url = match url {
        Ok(url) => url,
        Err(_) => "".to_string(),
    };
    println!("Upstream URL:{}", &full_url);
    let token = get_token(&req);
    let method = req.method().to_owned();
    let req_body = body::to_bytes(req.into_body(), usize::MAX).await.unwrap();
    let reqwest_resp = send_request(&full_url, &method, req_body, &token)
        .await
        .unwrap();
    let status = reqwest_resp.status();
    let mut response_builder = Response::builder().status(status);
    *response_builder.headers_mut().unwrap() = reqwest_resp.headers().clone();
    response_builder
        .body(Body::from_stream(reqwest_resp.bytes_stream()))
        .unwrap()
}

#[derive(Debug, Serialize, Deserialize)]
struct Params {
    provider: String,
    rest: String,
}

async fn send_request(
    url: &String,
    method: &Method,
    body: Bytes,
    token: &Option<String>,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = Client::new();
    let mut client = match *method {
        Method::GET => client.get(url),
        Method::POST => client.post(url),
        Method::PUT => client.put(url),
        Method::DELETE => client.delete(url),
        _ => client.get(url),
    };

    if let Some(token) = token {
        client = client.header("Authorization", token);
    }
    let response = client
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;
    Ok(response)
}

fn get_upstream_url(
    req: &Request,
    provider: &str,
    rest: &str,
    providers: &HashMap<String, String>,
) -> std::result::Result<String, AppError> {
    // Construct the URL for the provider API
    let base_url = match providers.get(provider) {
        Some(url) => url,
        None => return Err(AppError::Other(format!("Provider not found: {}", provider))),
    };

    let url = Url::parse(base_url)?.join(rest)?;
    if let Some(query) = req.uri().query() {
        Ok(format!("{}?{}", url.to_string(), query))
    } else {
        Ok(url.to_string())
    }
}

fn get_token(req: &Request) -> Option<String> {
    req.headers()
        .get("Authorization")
        .and_then(|auth| auth.to_str().ok().map(String::from))
}

fn load_env_providers() -> HashMap<String, String> {
    let mut providers = HashMap::new();
    for &(k, v) in DEFAULT_PROVIDERS.iter() {
        providers.insert(k.to_string(), v.to_string());
    }
    // Override with environment variables
    if let Ok(api_providers) = env::var("API_PROVIDERS") {
        if let Ok(api_providers_json) = serde_json::from_str::<serde_json::Value>(&api_providers) {
            if let Some(api_providers_map) = api_providers_json.as_object() {
                for (k, v) in api_providers_map {
                    if let Some(url) = v.as_str() {
                        providers.insert(k.to_string(), url.to_string());
                    }
                }
            }
        }
    }
    providers
}
