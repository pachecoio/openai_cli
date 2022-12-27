use std::fmt::Display;
use std::io::{stdout, Write};
use reqwest::Client;
use serde::{Serialize, Deserialize};
use crate::commands::TextCommand;

const API_URL: &str = "https://api.openai.com/v1";
const COMPLETIONS_ENDPOINT: &str = "/completions";

const DEFAULT_TEXT_COMPLETION_MODEL: &str = "text-davinci-003";

pub async fn generate_text(client: Client, cmd: &TextCommand) -> Result<String, ServiceError> {
    let payload = TextCompletionPayload::new(
        &cmd.description,
        cmd.temperature,
        cmd.max_tokens,
    );

    let response = client
        .post(format!("{}{}", API_URL, COMPLETIONS_ENDPOINT))
        .json(&payload)
        .send().await.unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body = response.json::<TextCompletionResponse>().await.unwrap();
            let mut result = String::new();
            response_body.choices.iter().for_each(|choice| {
                result.push_str(&choice.text);
            });
            Ok(result)
        },
        _ => {
            println!("Error: {}", response.status());
            Err(ServiceError::new(
                "Error generating text",
                response.status()
            ))
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TextCompletionPayload {
    model: String,
    prompt: String,
    temperature: f32,
    max_tokens: u32,
}

impl TextCompletionPayload {
    pub fn new(prompt: &str, temperature: Option<f32>, max_tokens: Option<u32>) -> Self {
        Self {
            model: DEFAULT_TEXT_COMPLETION_MODEL.to_string(),
            prompt: prompt.to_string(),
            temperature: temperature.unwrap_or(0.7),
            max_tokens: max_tokens.unwrap_or(100),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TextCompletionResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<TextCompletionChoice>,
}

#[derive(Debug, Deserialize)]
pub struct TextCompletionChoice {
    text: String,
    index: u32,
    logprobs: Option<TextCompletionLogprobs>,
    finish_reason: String,
}

#[derive(Debug, Deserialize)]
pub struct TextCompletionLogprobs {
    token_logprobs: Vec<Vec<f32>>,
    text_offset: Vec<Vec<f32>>,
    top_logprobs: Vec<Vec<f32>>,
}

#[derive(Debug)]
pub struct ServiceError {
    message: String,
    status: reqwest::StatusCode,
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ServiceError {
    pub fn new(message: &str, status: reqwest::StatusCode) -> Self {
        Self {
            message: message.to_string(),
            status,
        }
    }
}

pub async fn generate_image(client: Client, description: &str) {
    println!("Generating image: {}", description);
}