use crate::models::general::llm::{APIResponse, ChatCompletion, Message};
use anyhow::Result;
use dotenv::dotenv;
use reqwest::Client;
use std::env;

use reqwest::header::{HeaderMap, HeaderValue};

// Call LLM (i.e GPT-4)
pub async fn call_gpt(messages: Vec<Message>) -> Result<String> {
    dotenv().ok();

    // Extract API Key Information
    let api_key = env::var("OPENAI_API_KEY").expect("Unable to find OpenAI API Key");
    let org = env::var("OPENAI_ORG_ID").expect("Unable to find OpenAI Org ID");

    // Confirm endpoint
    let url = "https://api.openai.com/v1/chat/completions";

    // Create Headers
    let mut headers = HeaderMap::new();

    // Create API Key Header
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))?,
    );

    // Create OpenAI Org header
    headers.insert("OpenAI-Organization", HeaderValue::from_str(&org).unwrap());
    let client = Client::builder().default_headers(headers).build()?;

    let chat_completion = ChatCompletion {
        model: "gpt-4".to_string(),
        messages,
        temperature: 0.1,
    };

    let res: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await?
        .json()
        .await?;

    // Send Response
    Ok(res.choices[0].message.content.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_call_to_openai() {
        let message = Message {
            role: "user".to_string(),
            content: "Hi there, this is a test. Give me a short response.".to_string(),
        };

        let res = call_gpt(vec![message]).await;

        match res {
            Ok(res_string) => dbg!(res_string),
            Err(e) => dbg!(e.to_string()),
        };
    }
}
