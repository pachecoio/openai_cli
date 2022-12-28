use std::fmt::Display;
use std::io::{stdout, Write};
use reqwest::Client;
use serde::{Serialize, Deserialize};
use crate::commands::{ImageCommand, TextCommand};

const API_URL: &str = "https://api.openai.com/v1";
const COMPLETIONS_ENDPOINT: &str = "/completions";
const IMAGE_GENERATION_ENDPOINT: &str = "/images/generations";

const DEFAULT_TEXT_COMPLETION_MODEL: &str = "text-davinci-003";

pub async fn generate_text(client: Client, cmd: &TextCommand) -> Result<String, ServiceError> {
    let payload = TextCompletionPayload::new(
        &cmd.description,
        &cmd.model,
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
            Err(ServiceError::new(
                "Error generating text",
                response.status()
            ))
        }
    }
}

pub async fn generate_images(client: Client, cmd: &ImageCommand) -> Result<Vec<String>, ServiceError> {
    let payload = ImageGenerationPayload::new(
        &cmd.description,
        None,
        None
    );

    let response = client
        .post(format!("{}{}", API_URL, IMAGE_GENERATION_ENDPOINT))
        .json(&payload)
        .send().await.unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body = response.json::<ImageGenerationResponse>().await.unwrap();
            let mut result = Vec::new();
            response_body.data.iter().for_each(|image| {
                result.push(image.url.clone());
            });
            Ok(result)
        },
        _ => {
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
    #[serde(rename = "prompt")]
    description: String,
    temperature: f32,
    max_tokens: u32,
}

impl TextCompletionPayload {
    pub fn new(prompt: &str, model: &Option<String>, temperature: Option<f32>, max_tokens: Option<u32>) -> Self {
        let m = match model {
            Some(m) => m.clone(),
            None => DEFAULT_TEXT_COMPLETION_MODEL.to_string()
        };
        Self {
            model: m,
            description: prompt.to_string(),
            temperature: temperature.unwrap_or(0.7),
            max_tokens: max_tokens.unwrap_or(100),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ImageGenerationPayload {
    #[serde(rename = "prompt")]
    description: String,

    #[serde(rename = "n")]
    number_of_images: i8,

    #[serde(rename = "size")]
    image_size: String,
}

impl ImageGenerationPayload {
    pub fn new(prompt: &str, number_of_images: Option<i8>, image_size: Option<String>) -> Self {
        Self {
            description: prompt.to_string(),
            number_of_images: number_of_images.unwrap_or(1),
            image_size: image_size.unwrap_or("1024x1024".to_string()),
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

#[derive(Debug, Deserialize)]
pub struct ImageGenerationResponse {
    created: u64,
    data: Vec<ImageGenerationData>,
}

#[derive(Debug, Deserialize)]
pub struct ImageGenerationData {
    url: String
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
