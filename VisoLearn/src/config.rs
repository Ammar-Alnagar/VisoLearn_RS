use dotenv::dotenv;
use std::env;
use std::collections::HashMap;
use lazy_static::lazy_static;

pub fn hf_token() -> String {
    env::var("HF_TOKEN").unwrap_or_default()
}

pub fn google_api_key() -> String {
    env::var("GOOGLE_API_KEY").unwrap_or_default()
}

pub static DIFFICULTY_LEVELS: [&str; 5] = [
    "Very Simple", "Simple", "Moderate", "Detailed", "Very Detailed",
];

lazy_static! {
    pub static ref DEFAULT_TREATMENT_PLANS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Level 1", "Develop social communication skills and manage specific interests while maintaining independence.");
        m.insert("Level 2", "Focus on structured learning environments with visual supports and consistent routines.");
        m.insert("Level 3", "Provide highly structured support with simplified visual information and sensory-appropriate environments.");
        m
    };
}

pub static IMAGE_STYLES: [&str; 5] = [
    "Realistic", "Illustration", "Cartoon", "Watercolor", "3D Rendering",
];

#[derive(Debug, Clone)]
pub struct DefaultSession {
    pub prompt: Option<String>,
    pub image: Option<String>,
    pub image_description: Option<String>,
    pub chat: Vec<(String, String)>,
    pub treatment_plan: String,
    pub topic_focus: String,
    pub key_details: Vec<String>,
    pub identified_details: Vec<String>,
    pub used_hints: Vec<String>,
    pub difficulty: String,
    pub age: String,
    pub autism_level: String,
    pub attempt_limit: u32,
    pub attempt_count: u32,
    pub details_threshold: f32,
    pub image_style: String,
}

impl Default for DefaultSession {
    fn default() -> Self {
        Self {
            prompt: None,
            image: None,
            image_description: None,
            chat: Vec::new(),
            treatment_plan: String::new(),
            topic_focus: String::new(),
            key_details: Vec::new(),
            identified_details: Vec::new(),
            used_hints: Vec::new(),
            difficulty: "Very Simple".to_string(),
            age: "3".to_string(),
            autism_level: "Level 1".to_string(),
            attempt_limit: 3,
            attempt_count: 0,
            details_threshold: 0.7,
            image_style: "Realistic".to_string(),
        }
    }
}
