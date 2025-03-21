use dotenv::dotenv;
use std::env;

mod config;
mod models;
mod ui;
mod utils;

use config::{google_api_key, hf_token};
use models::prompt_generation;
use ui::interface::create_interface;

#[tokio::main]
async fn main() {
    // Load environment variables from the .env file
    dotenv().ok();

    // Retrieve API keys from configuration
    let hf_token_value = hf_token();
    let google_api_key_value = google_api_key();
    println!("HF Token: {}", hf_token_value);
    println!("Google API Key: {}", google_api_key_value);

    // Configure the Google API using the API key
    // For example, if you have a function like `google_api::configure()`,
    // you can call it here:
    // google_api::configure(&google_api_key_value);

    // Generate a sample prompt (for testing/demo purposes)
    let sample_prompt = prompt_generation::generate_prompt_from_options(
        "Very Simple",
        "3",
        "Level 1",
        "Emotions",
        "", // treatment_plan: empty string triggers default behavior
        "Realistic",
    );
    println!("Sample Prompt:\n{}", sample_prompt);

    // Launch the user interface.
    // This could start a web server (e.g. using Actix-web) or a frontend app (e.g. Yew).
    // Here, we call create_interface() as an async function.
    create_interface().await;
}
