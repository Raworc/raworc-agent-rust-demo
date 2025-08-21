/*!
Simple Pirate-speaking Raworc Agent

This agent responds to all messages in Pirate language using Claude.
*/

use serde_json::Value;
use std::env;
use reqwest::Client;
use anyhow::Result;

/// Main handler function called by Raworc (sync wrapper)
pub fn process_message_sync(message: &str, context: &Value) -> String {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(process_message(message, context))
}

/// Async implementation - responds in Pirate language using Claude
async fn process_message(message: &str, context: &Value) -> String {
    match env::var("ANTHROPIC_API_KEY") {
        Ok(api_key) if !api_key.is_empty() => {
            call_claude_pirate(message, &api_key).await
        }
        _ => "Arrr! ANTHROPIC_API_KEY not configured in space secrets, ye landlubber!".to_string()
    }
}

/// Call Claude with pirate-speaking instructions
async fn call_claude_pirate(message: &str, api_key: &str) -> String {
    match call_claude_api(message, api_key).await {
        Ok(response) => format!("{}\n\n[🏴‍☠️ Pirate Agent - Powered by Claude, ye scurvy dog!]", response),
        Err(e) => format!("Blimey! Error calling Claude: {}", e)
    }
}

/// Call Claude API
async fn call_claude_api(message: &str, api_key: &str) -> Result<String> {
    let client = Client::new();
    
    let system_prompt = "You are a helpful assistant that always responds like a stereotypical pirate. \
                        You are knowledgeable, friendly, and helpful, but you speak entirely in pirate language. \
                        Use pirate vocabulary like 'arrr', 'ye', 'matey', 'ahoy', 'avast', 'shiver me timbers', etc. \
                        You can help with any topic but always respond as a pirate would speak. \
                        Be enthusiastic and colorful in your pirate speech!";
    
    let request_body = serde_json::json!({
        "model": "claude-3-5-sonnet-20241022",
        "max_tokens": 500,
        "system": system_prompt,
        "messages": [{"role": "user", "content": message}]
    });
    
    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&request_body)
        .send()
        .await?;
    
    if response.status().is_success() {
        let response_json: Value = response.json().await?;
        let content = response_json["content"][0]["text"]
            .as_str()
            .unwrap_or("Arrr! No response from Claude, the scallywag!");
        Ok(content.to_string())
    } else {
        Err(anyhow::anyhow!("Claude API request failed with status: {}", response.status()))
    }
}