use std::sync::Mutex;
use once_cell::sync::Lazy;
use reqwest;
use serde_json::json;
use base64::encode;
use image::DynamicImage;
use std::error::Error;

// Global variables similar to Python globals.
static GLOBAL_IMAGE_DATA_URL: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));
static GLOBAL_IMAGE_PROMPT: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));
// static GLOBAL_IMAGE_DESCRIPTION: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));

// Assume you have a config module that provides your Hugging Face token.
mod config {
    // Replace with your actual Hugging Face API token.
    pub const HF_TOKEN: &str = "your_huggingface_api_token";
}

/// Asynchronously generates an image from the given prompt using the Hugging Face Inference API,
/// converts the image to a data URL, and updates global variables.
pub async fn generate_image_fn(
    selected_prompt: &str,
    guidance_scale: f32,
    negative_prompt: &str,
    num_inference_steps: i32,
) -> Result<DynamicImage, Box<dyn Error>> {
    // Update global prompt variable.
    {
        let mut prompt_lock = GLOBAL_IMAGE_PROMPT.lock().unwrap();
        *prompt_lock = Some(selected_prompt.to_string());
    }

    // Create a new reqwest client.
    let client = reqwest::Client::new();
    let url = "https://api-inference.huggingface.co/models/stabilityai/stable-diffusion-3.5-large-turbo";

    // Construct the JSON payload.
    let payload = json!({
         "inputs": selected_prompt,
         "parameters": {
              "guidance_scale": guidance_scale,
              "negative_prompt": negative_prompt,
              "num_inference_steps": num_inference_steps
         }
    });

    // Send the POST request with the authorization header.
    let response = client.post(url)
        .header("Authorization", format!("Bearer {}", config::HF_TOKEN))
        .json(&payload)
        .send()
        .await?;

    // Check if the request was successful.
    if !response.status().is_success() {
         println!("Error generating image: HTTP {}", response.status());
         return Err(format!("HTTP error: {}", response.status()).into());
    }

    // Get the image bytes from the response.
    let bytes = response.bytes().await?;
    
    // Load the image from memory using the `image` crate.
    let img = image::load_from_memory(&bytes)?;

    // Encode the image bytes to base64 and create the data URL.
    let img_b64 = encode(&bytes);
    let data_url = format!("data:image/png;base64,{}", img_b64);
    
    {
        let mut data_url_lock = GLOBAL_IMAGE_DATA_URL.lock().unwrap();
        *data_url_lock = Some(data_url);
    }

    println!(
        "Successfully generated image with prompt: {}...",
        selected_prompt.chars().take(50).collect::<String>()
    );

    // Return the generated image.
    Ok(img)
}

// Example usage with Tokio async runtime.
#[tokio::main]
async fn main() {
    // You can adjust these parameters as needed.
    let prompt = "A beautiful futuristic cityscape at sunset";
    let guidance_scale = 8.0;
    let negative_prompt = "blurry, distorted, low quality, pixelated, poorly drawn, deformed, unfinished, sketchy, cartoon, blur";
    let num_inference_steps = 50;

    match generate_image_fn(prompt, guidance_scale, negative_prompt, num_inference_steps).await {
        Ok(img) => {
            // Here, you might save the image or do further processing.
            println!("Image generated successfully!");
            // For example, save to disk:
            if let Err(e) = img.save("output.png") {
                eprintln!("Failed to save image: {}", e);
            }
        }
        Err(e) => eprintln!("Error generating image: {}", e),
    }
}
