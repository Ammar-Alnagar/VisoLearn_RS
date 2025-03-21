use base64::{engine::general_purpose, Engine as _};
use image::io::Reader as ImageReader;
use image::ImageOutputFormat;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
use std::io::Cursor;

// Define structs to represent the Gemini API request and response structure.
// You might need to adjust these based on the actual Gemini API documentation.

#[derive(Serialize)]
struct GeminiPart {
    inline_data: Option<GeminiInlineData>,
    text: Option<String>,
}

#[derive(Serialize)]
struct GeminiInlineData {
    mime_type: String,
    data: String,
}

#[derive(Serialize)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
}

#[derive(Deserialize, Debug)]
struct GeminiResponse {
    candidates: Option<Vec<GeminiCandidate>>,
}

#[derive(Deserialize, Debug)]
struct GeminiCandidate {
    content: GeminiContentResponse,
}

#[derive(Deserialize, Debug)]
struct GeminiContentResponse {
    parts: Vec<GeminiPartResponse>,
}

#[derive(Deserialize, Debug)]
struct GeminiPartResponse {
    text: Option<String>,
}

// --- Function: generate_detailed_description ---
pub async fn generate_detailed_description(
    image_input: Option<Vec<u8>>, // Using Option<Vec<u8>> to represent optional image input as bytes
    prompt: &str,
    difficulty: &str,
    topic_focus: &str,
) -> Result<String, String> {
    if image_input.is_none() {
        return Err(
            "Error: No image provided. Please make sure an image is generated or uploaded first."
                .to_string(),
        );
    }
    let image_bytes = image_input.unwrap();

    let base64_img = general_purpose::STANDARD.encode(&image_bytes);
    let query = format!(
        r#"
            You are an expert educator specializing in teaching users with autism.
            Please provide a detailed description of this image that was generated based on the prompt:
            "{}"
            The image is intended for a person with autism, focusing on the topic: "{}" at a {} difficulty level.
            In your description:
            1. List all key objects, characters, and elements present in the image
            2. Describe colors, shapes, positions, and relationships between elements
            3. Note any emotions, actions, or interactions depicted
            4. Highlight details that would be important for the child to notice
            5. Organize your description in a structured, clear way
            6. Dont generate a certain style , use the topic of focus to guide your descriptions
            Your description will be used as a reference to evaluate the child's observations,
            so please be comprehensive but focus on observable details rather than interpretations.
            "#,
        prompt, topic_focus, difficulty
    );

    let gemini_request = GeminiRequest {
        contents: vec![GeminiContent {
            parts: vec![
                GeminiPart {
                    inline_data: Some(GeminiInlineData {
                        mime_type: "image/png".to_string(), // Assuming PNG format for simplicity
                        data: base64_img,
                    }),
                    text: None,
                },
                GeminiPart {
                    inline_data: None,
                    text: Some(query),
                },
            ],
        }],
    };

    let api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set"); // Make sure you have environment variable GEMINI_API_KEY
    let client = Client::new();
    let gemini_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-thinking-exp-01-21:generateContent?key={}",
        api_key
    );


    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    match client
        .post(&gemini_url)
        .headers(headers)
        .json(&gemini_request)
        .send()
        .await
    {
        Ok(response) => {
            match response.json::<GeminiResponse>().await {
                Ok(gemini_response) => {
                    if let Some(candidates) = gemini_response.candidates {
                        if let Some(candidate) = candidates.get(0) {
                            if let Some(part_response) = candidate.content.parts.get(0) {
                                if let Some(text) = &part_response.text {
                                    return Ok(text.trim().to_string());
                                }
                            }
                        }
                    }
                    Err("Error: No text response from Gemini API".to_string())
                }
                Err(e) => Err(format!("Error parsing Gemini API response: {}", e)),
            }
        }
        Err(e) => Err(format!("Error calling Gemini API: {}", e)),
    }
}


// --- Function: extract_key_details ---
pub async fn extract_key_details(
    image_input: Option<Vec<u8>>, // Using Option<Vec<u8>> to represent optional image input as bytes
    prompt: &str,
    topic_focus: &str,
) -> Result<Vec<String>, String> {
    if image_input.is_none() {
        return Ok(vec!["Error: No image provided".to_string()]); // Return Ok with error string as per Python
    }
    let image_bytes = image_input.unwrap();
    let base64_img = general_purpose::STANDARD.encode(&image_bytes);

    let query = format!(
        r#"
            You are analyzing an educational image created for a person with autism, based on the prompt: "{}".
            The image focuses on the topic: "{}".
            Please extract a list of unique key details that a person might identify in this image minimum 5 , max 15 depending on the image.
            Each detail should be a simple, clear phrase describing one observable element.
            Focus on concrete, visible elements rather than abstract concepts.
            Format your response as a JSON array of strings, each representing one key detail.
            Example format: ["red ball on the grass", "smiling girl with brown hair", "blue sky with clouds"]
            Ensure each detail is:
            1. Directly observable in the image
            2. Unique (not a duplicate)
            3. Described in simple, concrete language
            4. Relevant to what a person would notice
            5. Avoid duplicates
            "#,
        prompt, topic_focus
    );

    let gemini_request = GeminiRequest {
        contents: vec![GeminiContent {
            parts: vec![
                GeminiPart {
                    inline_data: Some(GeminiInlineData {
                        mime_type: "image/png".to_string(), // Assuming PNG format for simplicity
                        data: base64_img,
                    }),
                    text: None,
                },
                GeminiPart {
                    inline_data: None,
                    text: Some(query),
                },
            ],
        }],
    };

    let api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");
    let client = Client::new();
    let gemini_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-pro-exp-02-05:generateContent?key={}",
        api_key
    );

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));


    match client
        .post(&gemini_url)
        .headers(headers)
        .json(&gemini_request)
        .send()
        .await
    {
        Ok(response) => {
            match response.json::<GeminiResponse>().await {
                Ok(gemini_response) => {
                    if let Some(candidates) = gemini_response.candidates {
                        if let Some(candidate) = candidates.get(0) {
                            if let Some(part_response) = candidate.content.parts.get(0) {
                                if let Some(text) = &part_response.text {
                                    // Attempt to parse JSON from the text response
                                    match serde_json::from_str::<Vec<String>>(text.trim()) {
                                        Ok(details) => Ok(details),
                                        Err(_) => {
                                            // If JSON parsing fails, attempt to extract bullet points or lines (similar to Python)
                                            let lines: Vec<&str> = text.trim().split('\n').collect();
                                            let mut details = Vec::new();
                                            for line in lines {
                                                if line.trim().starts_with('-') || line.trim().starts_with('*') {
                                                    details.push(line.trim()[1..].trim().to_string());
                                                }
                                            }
                                            if !details.is_empty() {
                                                Ok(details.into_iter().take(15).collect()) // Limit to max 15
                                            } else {
                                                Ok(vec!["object in image".to_string(), "color".to_string(), "shape".to_string(), "background".to_string()]) // Default fallback
                                            }
                                        }
                                    }
                                } else {
                                    Err("Error: No text response from Gemini API".to_string())
                                }
                            } else {
                                Err("Error: Unexpected API response format (no parts)".to_string())
                            }
                        } else {
                            Err("Error: Unexpected API response format (no candidates)".to_string())
                        }
                    } else {
                        Err("Error: Unexpected API response format (no candidates)".to_string())
                    }
                }
                Err(e) => Err(format!("Error parsing Gemini API response: {}", e)),
            }
        }
        Err(e) => Err(format!("Error calling Gemini API: {}", e)),
    }
}


// --- Placeholder functions for the remaining Python functions ---
// You will need to implement these functions based on the Python code provided,
// translating the logic to Rust and using appropriate Rust libraries.

pub async fn compare_details_chat_fn(
    _user_details: &str,
    _active_session: &mut std::collections::HashMap<String, serde_json::Value>, // Using HashMap as a placeholder for active_session
    _global_image_data_url: Option<String>, // Placeholder for image data URL
    _global_image_description: Option<String>, // Placeholder for image description
) -> Result<String, String> {
    // Implement the logic of compare_details_chat_fn here in Rust
    // ...
    Ok("Function compare_details_chat_fn not yet implemented in Rust".to_string()) // Placeholder return
}

pub fn parse_evaluation(
    _evaluation_text: &str,
    _active_session: &mut std::collections::HashMap<String, serde_json::Value>, // Placeholder for active_session
) -> Result<(String, String, bool, Vec<String>, i32), String> {
    // Implement the logic of parse_evaluation here in Rust
    // ...
    Ok((
        "Function parse_evaluation not yet implemented".to_string(),
        "Very Simple".to_string(),
        false,
        vec![],
        0,
    )) // Placeholder return
}


pub fn update_checklist(
    _checklist: Vec<serde_json::Value>, // Placeholder for checklist type
    _newly_identified: Vec<String>,
    _key_details: Vec<String>,
) -> Vec<serde_json::Value> {
    // Implement the logic of update_checklist here in Rust
    // ...
    vec![] // Placeholder return
}


// --- Helper function to load image from path (for testing) ---
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    async fn load_image_from_path(path: &str) -> Result<Vec<u8>, String> {
        fs::read(path).map_err(|e| format!("Error reading image file: {}", e))
    }

    #[tokio::test]
    async fn test_generate_detailed_description() {
        // Replace "path/to/your/image.png" with the actual path to a test image file
        let image_path = "path/to/your/image.png"; // <---  Put your image path here for testing
        let image_bytes_result = load_image_from_path(image_path).await;

        if let Ok(image_bytes) = image_bytes_result {
            let prompt = "A cat sitting on a mat";
            let difficulty = "Simple";
            let topic_focus = "Animals";

            match generate_detailed_description(Some(image_bytes), prompt, difficulty, topic_focus).await {
                Ok(description) => {
                    println!("Detailed Description:\n{}", description);
                    assert!(!description.is_empty()); // Just check if description is not empty for now
                }
                Err(err) => {
                    eprintln!("Error generating detailed description: {}", err);
                    panic!("Test failed due to error: {}", err);
                }
            }
        } else if let Err(err) = image_bytes_result {
            eprintln!("Error loading image: {}", err);
            panic!("Test setup failed: {}", err);
        }
    }


    #[tokio::test]
    async fn test_extract_key_details() {
        // Replace "path/to/your/image.png" with the actual path to a test image file
        let image_path = "path/to/your/image.png"; // <---  Put your image path here for testing
        let image_bytes_result = load_image_from_path(image_path).await;

        if let Ok(image_bytes) = image_bytes_result {
            let prompt = "A dog playing in the park";
            let topic_focus = "Outdoor activities";

            match extract_key_details(Some(image_bytes), prompt, topic_focus).await {
                Ok(details) => {
                    println!("Key Details:\n{:?}", details);
                    assert!(!details.is_empty()); // Check if details list is not empty
                }
                Err(err) => {
                    eprintln!("Error extracting key details: {}", err);
                    panic!("Test failed due to error: {}", err);
                }
            }
        } else if let Err(err) = image_bytes_result {
            eprintln!("Error loading image: {}", err);
            panic!("Test setup failed: {}", err);
        }
    }
}