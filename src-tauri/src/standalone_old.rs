// Standalone mode - Direct AI API integration without server
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Provider {
    pub name: String,
    pub api_key: String,
    pub model: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    pub provider: Provider,
    pub messages: Vec<Message>,
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    pub content: String,
    pub tokens: u32,
    pub cost: f64,
}

// Claude API integration
async fn call_claude(
    api_key: &str,
    model: &str,
    messages: &[Message],
    max_tokens: u32,
) -> Result<ChatResponse, Box<dyn Error>> {
    let client = reqwest::Client::new();
    
    let anthropic_messages: Vec<serde_json::Value> = messages
        .iter()
        .map(|m| {
            serde_json::json!({
                "role": m.role,
                "content": m.content
            })
        })
        .collect();

    let body = serde_json::json!({
        "model": model,
        "messages": anthropic_messages,
        "max_tokens": max_tokens,
    });

    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await?;

    let result: serde_json::Value = response.json().await?;
    
    let content = result["content"][0]["text"]
        .as_str()
        .unwrap_or("")
        .to_string();
    
    let input_tokens = result["usage"]["input_tokens"].as_u64().unwrap_or(0) as u32;
    let output_tokens = result["usage"]["output_tokens"].as_u64().unwrap_or(0) as u32;
    let total_tokens = input_tokens + output_tokens;
    
    // Claude 3.5 Sonnet pricing: $3/$15 per 1M tokens
    let cost = (input_tokens as f64 * 3.0 / 1_000_000.0) 
             + (output_tokens as f64 * 15.0 / 1_000_000.0);

    Ok(ChatResponse {
        content,
        tokens: total_tokens,
        cost,
    })
}

// OpenAI API integration
async fn call_openai(
    api_key: &str,
    model: &str,
    messages: &[Message],
    max_tokens: u32,
) -> Result<ChatResponse, Box<dyn Error>> {
    let client = reqwest::Client::new();
    
    let openai_messages: Vec<serde_json::Value> = messages
        .iter()
        .map(|m| {
            serde_json::json!({
                "role": m.role,
                "content": m.content
            })
        })
        .collect();

    let body = serde_json::json!({
        "model": model,
        "messages": openai_messages,
        "max_tokens": max_tokens,
    });

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await?;

    let result: serde_json::Value = response.json().await?;
    
    let content = result["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("")
        .to_string();
    
    let total_tokens = result["usage"]["total_tokens"].as_u64().unwrap_or(0) as u32;
    
    // GPT-4o pricing: $2.50/$10 per 1M tokens
    let prompt_tokens = result["usage"]["prompt_tokens"].as_u64().unwrap_or(0);
    let completion_tokens = result["usage"]["completion_tokens"].as_u64().unwrap_or(0);
    let cost = (prompt_tokens as f64 * 2.5 / 1_000_000.0) 
             + (completion_tokens as f64 * 10.0 / 1_000_000.0);

    Ok(ChatResponse {
        content,
        tokens: total_tokens,
        cost,
    })
}

// Google Gemini API integration
async fn call_gemini(
    api_key: &str,
    model: &str,
    messages: &[Message],
    _max_tokens: u32,
) -> Result<ChatResponse, Box<dyn Error>> {
    let client = reqwest::Client::new();
    
    let gemini_contents: Vec<serde_json::Value> = messages
        .iter()
        .map(|m| {
            serde_json::json!({
                "role": if m.role == "assistant" { "model" } else { "user" },
                "parts": [{ "text": m.content }]
            })
        })
        .collect();

    let body = serde_json::json!({
        "contents": gemini_contents,
    });

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model, api_key
    );

    let response = client
        .post(&url)
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await?;

    let result: serde_json::Value = response.json().await?;
    
    let content = result["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .unwrap_or("")
        .to_string();
    
    let input_tokens = result["usageMetadata"]["promptTokenCount"].as_u64().unwrap_or(0);
    let output_tokens = result["usageMetadata"]["candidatesTokenCount"].as_u64().unwrap_or(0);
    let total_tokens = (input_tokens + output_tokens) as u32;
    
    // Gemini 2.0 Flash pricing: Free up to rate limits
    let cost = 0.0;

    Ok(ChatResponse {
        content,
        tokens: total_tokens,
        cost,
    })
}

// Main chat function - routes to appropriate provider
pub async fn chat(request: ChatRequest) -> Result<ChatResponse, Box<dyn Error>> {
    let max_tokens = request.max_tokens.unwrap_or(4096);
    
    match request.provider.name.as_str() {
        "anthropic" | "claude" => {
            call_claude(
                &request.provider.api_key,
                &request.provider.model,
                &request.messages,
                max_tokens,
            ).await
        }
        "openai" => {
            call_openai(
                &request.provider.api_key,
                &request.provider.model,
                &request.messages,
                max_tokens,
            ).await
        }
        "google" | "gemini" => {
            call_gemini(
                &request.provider.api_key,
                &request.provider.model,
                &request.messages,
                max_tokens,
            ).await
        }
        _ => Err(format!("Unknown provider: {}", request.provider.name).into()),
    }
}

// Settings management
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct StandaloneSettings {
    pub providers: Vec<Provider>,
    pub default_provider: String,
    pub max_tokens: u32,
    pub temperature: f32,
}

impl StandaloneSettings {
    pub fn load() -> Self {
        // Load from app data directory
        if let Ok(config_dir) = tauri::api::path::config_dir() {
            let settings_path = config_dir.join("squan").join("settings.json");
            if let Ok(contents) = std::fs::read_to_string(&settings_path) {
                if let Ok(settings) = serde_json::from_str(&contents) {
                    return settings;
                }
            }
        }
        
        Self::default()
    }
    
    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        if let Ok(config_dir) = tauri::api::path::config_dir() {
            let squan_dir = config_dir.join("squan");
            std::fs::create_dir_all(&squan_dir)?;
            
            let settings_path = squan_dir.join("settings.json");
            let json = serde_json::to_string_pretty(self)?;
            std::fs::write(&settings_path, json)?;
        }
        
        Ok(())
    }
}
