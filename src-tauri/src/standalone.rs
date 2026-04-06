// Standalone mode - Direct AI API integration without server (Tauri v2 compatible)
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
    pub temperature: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    pub message: String,
    pub tokens_used: u32,
    pub cost: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub provider: String,
    pub api_key: String,
    pub model: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            provider: "claude".to_string(),
            api_key: String::new(),
            model: "claude-3-5-sonnet-20241022".to_string(),
        }
    }
}

// Get config directory (Tauri v2 compatible)
fn get_config_dir() -> Result<PathBuf, String> {
    // Try APPDATA (Windows) or HOME (Mac/Linux)
    std::env::var("APPDATA")
        .or_else(|_| std::env::var("HOME"))
        .map(|p| PathBuf::from(p))
        .map_err(|e| format!("Could not get config directory: {}", e))
}

#[tauri::command]
pub async fn standalone_chat(request: ChatRequest) -> Result<ChatResponse, String> {
    match request.provider.name.as_str() {
        "claude" => call_claude(&request).await,
        "openai" => call_openai(&request).await,
        "gemini" => call_gemini(&request).await,
        _ => Err(format!("Unknown provider: {}", request.provider.name)),
    }
}

async fn call_claude(request: &ChatRequest) -> Result<ChatResponse, String> {
    let client = reqwest::Client::new();
    
    let anthropic_messages: Vec<serde_json::Value> = request.messages.iter().map(|m| {
        serde_json::json!({
            "role": m.role,
            "content": m.content
        })
    }).collect();
    
    let body = serde_json::json!({
        "model": request.provider.model,
        "messages": anthropic_messages,
        "max_tokens": 4096,
        "temperature": request.temperature
    });
    
    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", &request.provider.api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("API error: {}", error_text));
    }
    
    let data: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;
    
    let message = data["content"][0]["text"]
        .as_str()
        .unwrap_or("No response")
        .to_string();
    
    let input_tokens = data["usage"]["input_tokens"].as_u64().unwrap_or(0) as u32;
    let output_tokens = data["usage"]["output_tokens"].as_u64().unwrap_or(0) as u32;
    let tokens_used = input_tokens + output_tokens;
    
    // Cost: $3 per 1M input, $15 per 1M output tokens
    let cost = (input_tokens as f64 * 3.0 / 1_000_000.0) + (output_tokens as f64 * 15.0 / 1_000_000.0);
    
    Ok(ChatResponse {
        message,
        tokens_used,
        cost,
    })
}

async fn call_openai(request: &ChatRequest) -> Result<ChatResponse, String> {
    let client = reqwest::Client::new();
    
    let openai_messages: Vec<serde_json::Value> = request.messages.iter().map(|m| {
        serde_json::json!({
            "role": m.role,
            "content": m.content
        })
    }).collect();
    
    let body = serde_json::json!({
        "model": request.provider.model,
        "messages": openai_messages,
        "temperature": request.temperature
    });
    
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", request.provider.api_key))
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("API error: {}", error_text));
    }
    
    let data: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;
    
    let message = data["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("No response")
        .to_string();
    
    let prompt_tokens = data["usage"]["prompt_tokens"].as_u64().unwrap_or(0) as u32;
    let completion_tokens = data["usage"]["completion_tokens"].as_u64().unwrap_or(0) as u32;
    let tokens_used = prompt_tokens + completion_tokens;
    
    // Cost for GPT-4: $0.03 per 1K input, $0.06 per 1K output tokens
    let cost = (prompt_tokens as f64 * 0.03 / 1000.0) + (completion_tokens as f64 * 0.06 / 1000.0);
    
    Ok(ChatResponse {
        message,
        tokens_used,
        cost,
    })
}

async fn call_gemini(request: &ChatRequest) -> Result<ChatResponse, String> {
    let client = reqwest::Client::new();
    
    let gemini_contents: Vec<serde_json::Value> = request.messages.iter().map(|m| {
        serde_json::json!({
            "role": if m.role == "assistant" { "model" } else { "user" },
            "parts": [{ "text": m.content }]
        })
    }).collect();
    
    let body = serde_json::json!({
        "contents": gemini_contents,
        "generationConfig": {
            "temperature": request.temperature
        }
    });
    
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        request.provider.model,
        request.provider.api_key
    );
    
    let response = client
        .post(&url)
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("API error: {}", error_text));
    }
    
    let data: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;
    
    let message = data["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .unwrap_or("No response")
        .to_string();
    
    let prompt_tokens = data["usageMetadata"]["promptTokenCount"].as_u64().unwrap_or(0) as u32;
    let completion_tokens = data["usageMetadata"]["candidatesTokenCount"].as_u64().unwrap_or(0) as u32;
    let tokens_used = prompt_tokens + completion_tokens;
    
    // Cost for Gemini 1.5 Pro: Free up to limits, then minimal
    let cost = (tokens_used as f64 * 0.001 / 1000.0);
    
    Ok(ChatResponse {
        message,
        tokens_used,
        cost,
    })
}

#[tauri::command]
pub async fn save_settings(settings: Settings) -> Result<(), String> {
    let config_dir = get_config_dir()?;
    let settings_path = config_dir.join("squan").join("settings.json");
    
    std::fs::create_dir_all(settings_path.parent().unwrap())
        .map_err(|e| e.to_string())?;
    
    let json = serde_json::to_string_pretty(&settings)
        .map_err(|e| e.to_string())?;
    
    std::fs::write(settings_path, json)
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn load_settings() -> Result<Settings, String> {
    let config_dir = get_config_dir()?;
    let squan_dir = config_dir.join("squan");
    let settings_path = squan_dir.join("settings.json");
    
    if settings_path.exists() {
        let json = std::fs::read_to_string(settings_path)
            .map_err(|e| e.to_string())?;
        
        let settings: Settings = serde_json::from_str(&json)
            .map_err(|e| e.to_string())?;
        
        return Ok(settings);
    }
    
    Ok(Settings::default())
}

#[tauri::command]
pub async fn test_connection(url: String) -> Result<bool, String> {
    let client = reqwest::Client::new();
    
    let response = client
        .get(&format!("{}/health", url))
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await;
    
    match response {
        Ok(resp) => Ok(resp.status().is_success()),
        Err(_) => Ok(false),
    }
}
