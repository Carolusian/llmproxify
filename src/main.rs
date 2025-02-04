mod errors;

use self::errors::AppError;
use dotenv::dotenv;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use url::Url;

use axum::{
    body::{self, Body},
    extract::{Path, Request, State},
    http::Method,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use reqwest::{self, Client, StatusCode};
use serde::{Deserialize, Serialize};

const DEFAULT_PROVIDERS: &[(&str, &str)] = &[
    ("openai", "https://api.openai.com/"),
    ("groq", "https://api.groq.com/"),
    ("cerebras", "https://api.cerebras.ai/"),
    ("gemini", "https://generativelanguage.googleapis.com/"),
    ("sambanova", "https://api.sambanova.ai/"),
    ("anthropic", "https://api.anthropic.com/"),
];

#[tokio::main]
async fn main() {
    // Load api providers
    dotenv().ok();

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
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let url = get_upstream_url(&req, &params.provider, &params.rest, &api_providers)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("{}", e)))?;

    println!("Upstream URL:{}", &url);
    let reqwest_resp = send_request(&url, req)
        .await
        .map_err(|e| (e.status().unwrap(), format!("{}", e)))?;

    let status = reqwest_resp.status();
    let mut response_builder = Response::builder().status(status);
    *response_builder.headers_mut().unwrap() = reqwest_resp.headers().clone();
    let response = response_builder
        .body(Body::from_stream(reqwest_resp.bytes_stream()))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)));
    Ok(response)
}

#[derive(Debug, Serialize, Deserialize)]
struct Params {
    provider: String,
    rest: String,
}

async fn send_request(url: &String, req: Request) -> Result<reqwest::Response, reqwest::Error> {
    let client = if let Some(proxy) = load_proxy() {
        Client::builder()
            .proxy(reqwest::Proxy::all(proxy)?)
            .build()?
    } else {
        Client::builder().build()?
    };

    let token = get_token(&req);
    let method = req.method();
    let mut client = match *method {
        Method::GET => client.get(url),
        Method::POST => client.post(url),
        Method::PUT => client.put(url),
        Method::DELETE => client.delete(url),
        _ => client.get(url),
    };

    if let Some(token) = token {
        client = client.header("authorization", token);
    }

    let header_keys = vec!["x-api-key", "anthropic-version", "content-type"];
    for (key, val) in req.headers() {
        if header_keys.contains(&key.as_str()) {
            client = client.header(key, val);
        }
    }

    let req_body = body::to_bytes(req.into_body(), usize::MAX).await.unwrap();
    let response = client.body(req_body).send().await?;
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

fn get_token(req: &Request) -> Option<&str> {
    req.headers()
        .get("Authorization")
        .and_then(|auth| auth.to_str().ok())
}

fn load_env_providers() -> HashMap<String, String> {
    let mut providers = HashMap::new();
    for &(k, v) in DEFAULT_PROVIDERS.iter() {
        providers.insert(k.to_string(), v.to_string());
    }
    // Override with environment variables
    if let Some(api_providers) = env::var("API_PROVIDERS").ok() {
        if let Some(api_providers_json) =
            serde_json::from_str::<serde_json::Value>(&api_providers).ok()
        {
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

fn load_proxy() -> Option<String> {
    env::var("ALL_PROXY").ok()
}
