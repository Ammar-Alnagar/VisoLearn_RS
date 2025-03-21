use std::fs;
use std::io::Write;
use std::path::Path;
use chrono::Local;
use serde_json::{json, Value};
use base64;

/// Save all images from the saved sessions and active session to disk.
fn save_all_session_images(saved_sessions: &[Value], active_session: &Value) -> String {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let mut saved_count = 0;
    
    // Create a directory for the images if it doesn't exist
    let output_dir = format!("saved_images_{}", timestamp);
    fs::create_dir_all(&output_dir).unwrap_or_else(|e| {
        eprintln!("Error creating directory: {}", e);
    });
    
    // Save images from saved sessions
    for (i, session) in saved_sessions.iter().enumerate() {
        if let Some(image) = session.get("image")
                                   .and_then(|i| i.as_str())
                                   .filter(|url| url.starts_with("data:image")) {
            let filename = format!("{}/session_{}_{}.png", output_dir, i, timestamp);
            if save_image_from_data_url(image, &filename) {
                saved_count += 1;
            }
        }
    }
    
    // Save image from active session if it exists
    if let Some(image) = active_session.get("image")
                                     .and_then(|i| i.as_str())
                                     .filter(|url| url.starts_with("data:image")) {
        let filename = format!("{}/active_session_{}.png", output_dir, timestamp);
        if save_image_from_data_url(image, &filename) {
            saved_count += 1;
        }
    }
    
    format!("✅ Successfully saved {} images to folder: {}", saved_count, output_dir)
}

/// Extract base64 data from a data URL, decode it, and save it as an image file.
fn save_image_from_data_url(data_url: &str, filename: &str) -> bool {
    if !data_url.starts_with("data:image") {
        println!("Invalid data URL format");
        return false;
    }
    
    match data_url.split(',').nth(1) {
        Some(base64_data) => {
            match base64::decode(base64_data) {
                Ok(image_data) => {
                    match fs::File::create(filename) {
                        Ok(mut file) => {
                            match file.write_all(&image_data) {
                                Ok(_) => {
                                    println!("Successfully saved image to {}", filename);
                                    true
                                },
                                Err(e) => {
                                    println!("Error writing image data: {}", e);
                                    false
                                }
                            }
                        },
                        Err(e) => {
                            println!("Error creating file: {}", e);
                            false
                        }
                    }
                },
                Err(e) => {
                    println!("Error decoding base64 data: {}", e);
                    false
                }
            }
        },
        None => {
            println!("Invalid data URL format - couldn't split base64 data");
            false
        }
    }
}

/// Save all session data (including active session) to a JSON file.
fn save_session_log(saved_sessions: &[Value], active_session: &Value) -> String {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let filename = format!("session_log_{}.json", timestamp);
    
    // Combine all sessions
    let mut all_sessions = saved_sessions.to_vec();
    if active_session.get("prompt").is_some() {
        all_sessions.push(active_session.clone());
    }
    
    // Clean up the sessions for saving (removing data URLs to reduce file size)
    let clean_sessions: Vec<Value> = all_sessions.iter().map(|session| {
        let mut clean_session = session.clone();
        
        if clean_session.get("image").is_some() {
            clean_session["image"] = json!("[IMAGE_DATA_REMOVED]");
        }
        
        clean_session
    }).collect();
    
    // Save to file
    match serde_json::to_string_pretty(&clean_sessions) {
        Ok(json_str) => {
            match fs::write(&filename, json_str) {
                Ok(_) => format!("✅ Session log saved to: {}", filename),
                Err(e) => {
                    println!("Error writing session log: {}", e);
                    format!("❌ Error saving session log: {}", e)
                }
            }
        },
        Err(e) => {
            println!("Error serializing session data: {}", e);
            format!("❌ Error saving session log: {}", e)
        }
    }
}

// Example of how you might use these functions in a main function
fn main() {
    // This would be your actual data
    let saved_sessions: Vec<Value> = vec![];
    let active_session = json!({});
    
    println!("{}", save_all_session_images(&saved_sessions, &active_session));
    println!("{}", save_session_log(&saved_sessions, &active_session));
}