use std::io::Cursor;
use std::sync::Mutex;
use base64;
use image::{DynamicImage, ImageOutputFormat};
use once_cell::sync::Lazy;

// Global variables for image data URL and description.
static GLOBAL_IMAGE_DATA_URL: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));
static GLOBAL_IMAGE_DESCRIPTION: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));

// A session structure that stores our UI state.
#[derive(Clone, Debug)]
struct Session {
    prompt: Option<String>,
    image: Option<String>,           // Stored as a data URL.
    image_description: Option<String>,
    chat: Vec<(String, String)>,     // (Speaker, Message)
    treatment_plan: Option<String>,
    topic_focus: Option<String>,
    key_details: Vec<String>,
    identified_details: Vec<String>,
    used_hints: Vec<String>,
    difficulty: String,
    autism_level: String,
    age: String,
    attempt_limit: u32,
    attempt_count: u32,
    details_threshold: f32,
    image_style: String,
    completed: bool,
}

impl Session {
    fn new() -> Self {
        Session {
            prompt: None,
            image: None,
            image_description: None,
            chat: Vec::new(),
            treatment_plan: None,
            topic_focus: None,
            key_details: Vec::new(),
            identified_details: Vec::new(),
            used_hints: Vec::new(),
            difficulty: "Very Simple".to_string(),
            autism_level: "Level 1".to_string(),
            age: "3".to_string(),
            attempt_limit: 3,
            attempt_count: 0,
            details_threshold: 0.7,
            image_style: "Realistic".to_string(),
            completed: false,
        }
    }
}

// A checklist item that tracks whether a key detail was identified.
#[derive(Clone, Debug)]
struct ChecklistItem {
    detail: String,
    identified: bool,
    id: usize,
}

// --- Dummy functions for imported functionality ---
//
// In your application these functions would call your APIs
// and contain the actual logic.

fn generate_prompt_from_options(
    difficulty: &str,
    age: &str,
    autism_level: &str,
    topic_focus: &str,
    treatment_plan: &str,
    image_style: &str,
) -> String {
    // Dummy implementation: construct a prompt string.
    format!(
        "Prompt: difficulty={}, age={}, autism_level={}, topic_focus={}, treatment_plan={}, image_style={}",
        difficulty, age, autism_level, topic_focus, treatment_plan, image_style
    )
}

fn generate_image_fn(prompt: String) -> Option<DynamicImage> {
    // Dummy image generation: return a blank RGB image.
    println!("Generating image for prompt:\n{}", prompt);
    Some(DynamicImage::new_rgb8(400, 300))
}

fn generate_detailed_description(
    image: &DynamicImage,
    prompt: String,
    difficulty: &str,
    topic_focus: &str,
) -> String {
    // Dummy implementation.
    format!("Detailed description for prompt: {}", prompt)
}

fn extract_key_details(
    image: &DynamicImage,
    prompt: String,
    topic_focus: &str,
) -> Vec<String> {
    // Dummy implementation: return several example details.
    vec![
        "detail1".to_string(),
        "detail2".to_string(),
        "detail3".to_string(),
        "detail4".to_string(),
        "detail5".to_string(),
    ]
}

fn compare_details_chat_fn(
    user_message: &str,
    session: &Session,
    image_data: Option<&String>,
    image_description: Option<&String>,
) -> String {
    // Dummy evaluation.
    "Evaluation: feedback, updated_difficulty, should_advance, newly_identified, score".to_string()
}

fn parse_evaluation(evaluation: &str, _session: &Session) -> (String, String, bool, Vec<String>, f32) {
    // Dummy parsing: return fixed feedback.
    (
        "Feedback message".to_string(),
        "Advanced Difficulty".to_string(),
        true,
        vec!["detail1".to_string()],
        0.8,
    )
}

fn update_checklist(
    checklist: &Vec<ChecklistItem>,
    newly_identified: Vec<String>,
    key_details: &Vec<String>,
) -> Vec<ChecklistItem> {
    // For each checklist item, mark it identified if its detail is in newly_identified.
    checklist
        .iter()
        .map(|item| ChecklistItem {
            detail: item.detail.clone(),
            identified: newly_identified.contains(&item.detail),
            id: item.id,
        })
        .collect()
}

// --- Main functions ---

/// Generate a new image (with the current difficulty) and reset the chat.
/// Returns a tuple: (image, new_active_session, new_sessions, checklist_items)
fn generate_image_and_reset_chat(
    age: &str,
    autism_level: &str,
    topic_focus: &str,
    treatment_plan: &str,
    attempt_limit_input: Option<u32>,
    details_threshold_input: Option<f32>,
    active_session: Session,
    saved_sessions: &Vec<Session>,
    image_style: &str,
) -> Result<(Option<DynamicImage>, Session, Vec<Session>, Vec<ChecklistItem>), Box<dyn std::error::Error>> {
    let mut new_sessions = saved_sessions.clone();
    if active_session.prompt.is_some() {
        new_sessions.push(active_session.clone());
    }

    let current_difficulty = active_session.difficulty.clone();
    let generated_prompt = generate_prompt_from_options(
        &current_difficulty,
        age,
        autism_level,
        topic_focus,
        treatment_plan,
        image_style,
    );

    // Generate the image.
    let image_opt = generate_image_fn(generated_prompt.clone());
    if image_opt.is_none() {
        return Ok((None, active_session, new_sessions, Vec::new()));
    }
    let image = image_opt.unwrap();

    // Convert the image to a data URL.
    let mut buffer = Vec::new();
    image.write_to(&mut Cursor::new(&mut buffer), ImageOutputFormat::Png)?;
    let img_b64 = base64::encode(&buffer);
    let image_data_url = format!("data:image/png;base64,{}", img_b64);
    {
        let mut global_url = GLOBAL_IMAGE_DATA_URL.lock().unwrap();
        *global_url = Some(image_data_url.clone());
    }

    // Generate detailed description.
    let image_description = generate_detailed_description(&image, generated_prompt.clone(), &current_difficulty, topic_focus);
    {
        let mut global_desc = GLOBAL_IMAGE_DESCRIPTION.lock().unwrap();
        *global_desc = Some(image_description.clone());
    }

    let key_details = extract_key_details(&image, generated_prompt.clone(), topic_focus);

    // Process details threshold.
    let mut details_threshold = details_threshold_input.unwrap_or(0.7);
    if details_threshold > 1.0 {
        details_threshold /= 100.0;
    }
    details_threshold = details_threshold.max(0.1).min(1.0);

    // Create a new active session.
    let new_active_session = Session {
        prompt: Some(generated_prompt),
        image: Some(image_data_url),
        image_description: Some(image_description),
        chat: Vec::new(),
        treatment_plan: Some(treatment_plan.to_string()),
        topic_focus: Some(topic_focus.to_string()),
        key_details: key_details.clone(),
        identified_details: Vec::new(),
        used_hints: Vec::new(),
        difficulty: current_difficulty,
        autism_level: autism_level.to_string(),
        age: age.to_string(),
        attempt_limit: attempt_limit_input.unwrap_or(3),
        attempt_count: 0,
        details_threshold,
        image_style: image_style.to_string(),
        completed: false,
    };

    let mut checklist_items = Vec::new();
    for (i, detail) in key_details.iter().enumerate() {
        checklist_items.push(ChecklistItem {
            detail: detail.clone(),
            identified: false,
            id: i,
        });
    }

    Ok((Some(image), new_active_session, new_sessions, checklist_items))
}

/// Process a chat message and update the session state accordingly.
/// Returns a tuple:
/// (user_input, updated_chat, saved_sessions, updated_active_session, updated_checklist, current_image)
fn chat_respond(
    user_message: &str,
    mut active_session: Session,
    saved_sessions: Vec<Session>,
    checklist: Vec<ChecklistItem>,
) -> Result<(String, Vec<(String, String)>, Vec<Session>, Session, Vec<ChecklistItem>, Option<DynamicImage>), Box<dyn std::error::Error>> {
    if active_session.image.is_none() {
        let bot_message = "Please generate an image first.".to_string();
        active_session.chat.push(("Child".to_string(), user_message.to_string()));
        active_session.chat.push(("Teacher".to_string(), bot_message));
        return Ok((String::new(), active_session.chat.clone(), saved_sessions, active_session, checklist, None));
    }

    // Convert the data URL back to an image.
    let mut current_image: Option<DynamicImage> = None;
    if let Some(ref data_url) = active_session.image {
        if data_url.starts_with("data:image") {
            if let Some(comma_index) = data_url.find(',') {
                let b64 = &data_url[comma_index + 1..];
                let img_bytes = base64::decode(b64)?;
                current_image = image::load_from_memory(&img_bytes).ok();
            }
        }
    }

    // Evaluate the child's message.
    let raw_evaluation = compare_details_chat_fn(
        user_message,
        &active_session,
        active_session.image.as_ref(),
        active_session.image_description.as_ref(),
    );
    let (feedback, updated_difficulty, should_advance, newly_identified, _score) =
        parse_evaluation(&raw_evaluation, &active_session);

    if newly_identified.is_empty() {
        active_session.attempt_count += 1;
    }

    let updated_checklist = update_checklist(&checklist, newly_identified.clone(), &active_session.key_details);

    active_session.chat.push(("Child".to_string(), user_message.to_string()));
    active_session.chat.push(("Teacher".to_string(), feedback.clone()));

    let identified_count = active_session.identified_details.len();
    let key_details_count = active_session.key_details.len();
    let threshold_count = ((key_details_count as f32) * active_session.details_threshold).ceil() as usize;

    let all_identified = updated_checklist.iter().all(|item| item.identified);
    let attempts_exhausted = active_session.attempt_count >= active_session.attempt_limit;
    let threshold_reached = identified_count >= threshold_count;

    println!("Details identified: {}/{}", identified_count, key_details_count);
    println!("Threshold count: {}", threshold_count);
    println!("Threshold reached: {}", threshold_reached);
    println!("All identified: {}", all_identified);
    println!("Attempts exhausted: {}", attempts_exhausted);
    println!("Should advance: {}", should_advance);

    // If conditions are met, generate a new image and advance.
    if threshold_reached || all_identified || attempts_exhausted || should_advance {
        println!("Generating new image and advancing...");

        let mut new_sessions = saved_sessions.clone();
        let mut completed_session = active_session.clone();
        completed_session.completed = true;
        new_sessions.push(completed_session);

        let age = active_session.age.clone();
        let autism_level = active_session.autism_level.clone();
        let topic_focus = active_session.topic_focus.clone().unwrap_or_default();
        let treatment_plan = active_session.treatment_plan.clone().unwrap_or_default();
        let image_style = active_session.image_style.clone();

        let difficulty_to_use = if threshold_reached || should_advance {
            updated_difficulty
        } else {
            active_session.difficulty.clone()
        };

        println!("Using difficulty level: {} for new image", difficulty_to_use);

        let generated_prompt = generate_prompt_from_options(
            &difficulty_to_use,
            &age,
            &autism_level,
            &topic_focus,
            &treatment_plan,
            &image_style,
        );
        let image_opt = generate_image_fn(generated_prompt.clone());
        if image_opt.is_none() {
            let advancement_message = "There was an issue generating a new image. Please try again.".to_string();
            active_session.chat.push(("System".to_string(), advancement_message));
            return Ok((
                String::new(),
                active_session.chat.clone(),
                new_sessions,
                active_session,
                updated_checklist,
                current_image,
            ));
        }
        let new_image = image_opt.unwrap();
        let mut buffer = Vec::new();
        new_image.write_to(&mut Cursor::new(&mut buffer), ImageOutputFormat::Png)?;
        let img_b64 = base64::encode(&buffer);
        let image_data_url = format!("data:image/png;base64,{}", img_b64);
        {
            let mut global_url = GLOBAL_IMAGE_DATA_URL.lock().unwrap();
            *global_url = Some(image_data_url.clone());
        }

        let image_description = generate_detailed_description(&new_image, generated_prompt.clone(), &difficulty_to_use, &topic_focus);
        let key_details = extract_key_details(&new_image, generated_prompt.clone(), &topic_focus);

        // Create a new session with the new image.
        let new_active_session = Session {
            prompt: Some(generated_prompt),
            image: Some(image_data_url),
            image_description: Some(image_description),
            chat: vec![("System".to_string(), {
                if attempts_exhausted {
                    "You've used all your allowed attempts. Let's try a new image.".to_string()
                } else if threshold_reached && updated_difficulty != active_session.difficulty {
                    format!(
                        "Congratulations! You've identified enough details ({}/{}) to advance to {} difficulty! Here's a new image to describe.",
                        identified_count, key_details.len(), updated_difficulty
                    )
                } else if should_advance {
                    format!(
                        "Congratulations! You've advanced to {} difficulty! Here's a new image to describe.",
                        updated_difficulty
                    )
                } else if threshold_reached || all_identified {
                    "Great job identifying the details! Here's a new image at the same difficulty level.".to_string()
                } else {
                    "Let's try a new image!".to_string()
                }
            })],
            treatment_plan: Some(treatment_plan),
            topic_focus: Some(topic_focus),
            key_details: key_details.clone(),
            identified_details: Vec::new(),
            used_hints: Vec::new(),
            difficulty: difficulty_to_use,
            autism_level: autism_level,
            age: age,
            attempt_limit: active_session.attempt_limit,
            attempt_count: 0,
            details_threshold: active_session.details_threshold,
            image_style: image_style,
            completed: false,
        };

        let mut new_checklist = Vec::new();
        for (i, detail) in key_details.iter().enumerate() {
            new_checklist.push(ChecklistItem {
                detail: detail.clone(),
                identified: false,
                id: i,
            });
        }

        return Ok((
            String::new(),
            new_active_session.chat.clone(),
            new_sessions,
            new_active_session,
            new_checklist,
            Some(new_image),
        ));
    }

    // Otherwise, return the updated state.
    Ok((
        String::new(),
        active_session.chat.clone(),
        saved_sessions,
        active_session,
        updated_checklist,
        current_image,
    ))
}

/// Combine finished sessions with the active session for display.
fn update_sessions(saved_sessions: Vec<Session>, active_session: Session) -> Vec<Session> {
    if active_session.prompt.is_some() {
        let mut sessions = saved_sessions;
        sessions.push(active_session);
        sessions
    } else {
        saved_sessions
    }
}

fn main() {
    // Example usage of generate_image_and_reset_chat.
    let active_session = Session::new();
    let saved_sessions = Vec::new();
    match generate_image_and_reset_chat(
        "5",
        "Level 2",
        "Emotions",
        "Plan A",
        Some(3),
        Some(70.0),
        active_session.clone(),
        &saved_sessions,
        "Realistic",
    ) {
        Ok((image_opt, new_active_session, new_sessions, checklist_items)) => {
            println!(
                "Generated image and reset chat. New session prompt: {:?}",
                new_active_session.prompt
            );
            println!("Checklist items: {}", checklist_items.len());
        }
        Err(e) => println!("Error: {}", e),
    }

    // Example usage of chat_respond.
    let mut active_session = Session::new();
    active_session.image = Some("data:image/png;base64,dummydata".to_string());
    let saved_sessions = Vec::new();
    let checklist = Vec::new();
    match chat_respond("Child description", active_session, saved_sessions, checklist) {
        Ok((user_input, chat_history, _saved_sessions, _active_session, _checklist, current_image)) => {
            println!("Chat response updated. Chat history length: {}", chat_history.len());
            if let Some(_img) = current_image {
                println!("Current image available.");
            }
        }
        Err(e) => println!("Error in chat_respond: {}", e),
    }
}
