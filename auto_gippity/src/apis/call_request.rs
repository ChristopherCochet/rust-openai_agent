use crate::models::general::llm::{ Message, ChatCompletion, APIResponse };
use dotenv::dotenv;
use reqwest::Client;
use std::env;
use reqwest::header::{ HeaderMap, HeaderValue };

// Call LLM 
pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>>  {
    dotenv().ok();
    // Extract API key information
    let api_key: String = env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY not found in env variables");
    let api_org: String = env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG not found in env variables");

    println!("{}",api_key);
    println!("{}",api_org);
    
    // Confirm endpoint
    let url: &str = "https://api.openai.com/v1/chat/completions";

    // Create headers
    let mut headers = HeaderMap::new();

    // Create api key header
    headers.insert(
        "authorization", 
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    // Create Open AI Org header
    headers.insert(
        "OpenAI-Organization", 
        HeaderValue::from_str(api_org.as_str())
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    //Create Client
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // Create ChatCompletion
    let chat_completion = ChatCompletion {
        model: "gpt-3.5-turbo".to_string(),
        messages: messages,
        temperature: 0.1,
    };

    // Troubleshooting
    let response_raw = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .unwrap();

    // Extract API Response
    let res: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // Send Response
    Ok(res.choices[0].message.content.clone())

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_call_to_openai() {
        let message = Message {
            role: "user".to_string(),
            content: "Hi there, this is a test. Giveme a short response".to_string(),
        };
    
        let messages = vec![message];
        let res= call_gpt(messages).await;
        match res {
            Ok(res_str) => {
                dbg!(res_str);
                assert!(true);
            }
            Err(_) => {
                assert!(false);
            }
        }

    }
}