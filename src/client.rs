use std::env;
use reqwest::{
    Client,
    header
};
use dotenv::dotenv;

pub fn client_builder() -> Client {
    Client::builder()
        .default_headers(get_headers())
        .build()
        .unwrap()
}

pub fn get_headers() -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();
    let token = get_auth_token();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(
            &format!("Bearer {}", token)
        ).unwrap()
    );
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json")
    );
    headers
}

pub fn get_auth_token() -> String {
    dotenv().ok();
    env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set")
}
