use rig::{client::{CompletionClient, ProviderClient}, completion::Prompt, providers::gemini::{self, completion::GEMINI_2_0_FLASH_LITE}};
use dotenv::dotenv;

#[allow(dead_code)]
pub async fn send_to_llm(prompt: String) -> String {
    dotenv().ok();
    let gemini_client = gemini::Client::from_env();
    let gemini_agent = gemini_client.agent(GEMINI_2_0_FLASH_LITE).build();

    gemini_agent
        .prompt(&prompt)
        .await
        .expect("Failed to prompt Gemini")
}