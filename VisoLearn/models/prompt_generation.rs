use std::collections::HashMap;
use std::error::Error;
use once_cell::sync::Lazy;
use lazy_static::lazy_static;

// Configuration module with default treatment plans.
pub mod config {
    use std::collections::HashMap;
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref DEFAULT_TREATMENT_PLANS: HashMap<&'static str, &'static str> = {
            let mut m = HashMap::new();
            m.insert("Level 1", "Default treatment plan for Level 1");
            m.insert("Level 2", "Default treatment plan for Level 2");
            // Add other levels as needed.
            m
        };
    }
}

/// A dummy structure representing the Google Generative AI model.
pub struct GenerativeModel {
    model_name: String,
}

impl GenerativeModel {
    pub fn new(model_name: &str) -> Self {
        Self {
            model_name: model_name.to_string(),
        }
    }

    /// Dummy API call. Replace with your actual HTTP request to Google’s service.
    pub async fn generate_content(&self, prompt: &str) -> Result<String, Box<dyn Error>> {
        // For a real implementation, you might do:
        // let client = reqwest::Client::new();
        // let response = client.post("https://api.generativeai.google/v1/...")
        //     .json(&serde_json::json!({ "prompt": prompt, "model": self.model_name }))
        //     .send().await?;
        // let json_response = response.json::<serde_json::Value>().await?;
        // Extract the generated text from json_response.
        //
        // Here we simply return the prompt back for demonstration.
        Ok(prompt.to_string())
    }
}

/// Generates a detailed prompt from the provided options using the Google Gemini model.
pub async fn generate_prompt_from_options(
    difficulty: &str,
    age: &str,
    autism_level: &str,
    topic_focus: &str,
    treatment_plan: Option<&str>,
    image_style: &str,
) -> Result<String, Box<dyn Error>> {
    // Use default treatment plan if none provided.
    let treatment_plan = if let Some(tp) = treatment_plan {
        if tp.trim().is_empty() {
            config::DEFAULT_TREATMENT_PLANS
                .get(autism_level)
                .unwrap_or(&config::DEFAULT_TREATMENT_PLANS["Level 1"])
                .to_string()
        } else {
            tp.to_string()
        }
    } else {
        config::DEFAULT_TREATMENT_PLANS
            .get(autism_level)
            .unwrap_or(&config::DEFAULT_TREATMENT_PLANS["Level 1"])
            .to_string()
    };
    println!(
        "Using default treatment plan for {}: {}",
        autism_level, treatment_plan
    );

    // Determine style instructions based on the provided image style.
    let style_instruction = match image_style {
        "Realistic" => "Create a realistic image with natural lighting and detailed textures, capturing the essence of real-world environments. Ensure the scene has a lifelike feel, with accurate light and shadow play, and textures that convey a true-to-life appearance.",
        "Illustration" => "Create a clean and colorful illustration in the style of children's books, featuring bold outlines, vibrant colors, and a playful, engaging composition. Ensure the artwork has a soft, friendly feel with well-defined shapes and a sense of warmth and charm.",
        "Cartoon" => "Create a friendly cartoon-style illustration with simplified shapes, bold outlines, and expressive characters. Ensure the characters have exaggerated facial expressions and dynamic poses to convey emotion and personality in a warm and inviting way.",
        "Watercolor" => "Create a soft watercolor illustration with gentle color transitions, delicate brushstrokes, and a dreamy, ethereal quality. Ensure the colors blend seamlessly, evoking a sense of warmth and tranquility.",
        "3D Rendering" => "Create a highly detailed 3D-rendered image with realistic depth, rich textures, and natural lighting effects. Ensure accurate reflections, shadows, and materials to enhance the sense of realism and immersion.",
        _ => "",
    };

    // Build the detailed prompt.
    let query = format!(
r#"Your task is to create an EXCEPTIONAL image generation prompt that will produce an educational image.
PARAMETERS:
- Difficulty: {}
- Person's Age: {}
- Autism Level: {}
- Topic Focus: {}
- Treatment Plan: {}
- Image Style: {}
CRITICAL PROMPT REQUIREMENTS:
1. START WITH A CLEAR CONCEPT: Begin with "A {} [scene description]" or "An {} of [scene description]"
2. ULTRA-SPECIFIC VISUAL DETAILS: Include at least 8-10 specific visual elements with clear positions and relationships
3. EXACT COLOR SPECIFICATION: Use precise color terminology (e.g., "pastel mint green" not just "green")
4. LIGHTING DIRECTIVES: Specify lighting quality (e.g., "soft diffused morning light", "dramatic side lighting")
5. CAMERA ANGLE & PERSPECTIVE: Include exact viewing angle (e.g., "eye-level close-up", "overhead view")
6. ARTISTIC STYLE: Reference specific art styles appropriate for autism education reflecting the selected style: {}
7. EMOTIONAL TONE: Explicitly state the emotional quality (e.g., "calm", "joyful", "serene atmosphere")
8. TEXTURE SPECIFICS: Detail textures visible in the image (e.g., "soft plush texture", "smooth polished surface")
9. Realism: Incorporate elements, textures, and lighting to enhance the image's depth according to the {} style.
TECHNICAL REQUIREMENTS:
- Your prompt MUST be at least 150 words long
- Include the exact phrase "high detail, high quality , 4k" in your prompt
- End with a technical directive: "8k resolution, professional {}, masterful composition"
- Add style-appropriate elements for {} imagery
- Ensure the Image follows the {} style guidelines
- Ensure the Image is not blurry or pixelated.
- Ensure the Image is not overly saturated or desaturated.
- Ensure the Image is not overly bright or dark.
- Ensure the Image is not overly contrasty or flat.
- Ensure the Image is not overly abstract or overly detailed for the selected style.
- Ensure there are no deformations or distortions.
- Ensure the image is not blurry
-Ensure maximum detail and  quality.

TOPIC INTEGRATION:
The image MUST focus primarily on "{}" while incorporating elements from the treatment plan: "{}".
EXAMPLE FORMAT:
"A {} scene of [main subject] with [specific details]. The [subject] is positioned [exact location] with [specific posture/action]. The lighting is [specific lighting description] creating [specific effect]. The background features [specific background elements] in [specific colors]. The foreground includes [specific foreground elements]. The scene conveys a feeling of [emotional quality]. In the style of [specific artistic reference]. High detail, sharp focus, 8k resolution, professional {}, masterful composition."
CREATE YOUR DETAILED PROMPT NOW:"#,
        difficulty, age, autism_level, topic_focus,
        treatment_plan, image_style,
        image_style.to_lowercase(), image_style.to_lowercase(),
        style_instruction,
        image_style, image_style.to_lowercase(),
        image_style, image_style, image_style,
        topic_focus, treatment_plan,
        image_style.to_lowercase(),
        image_style.to_lowercase()
    );

    // Instantiate the GenerativeModel and generate content.
    let model = GenerativeModel::new("gemini-2.0-pro-exp-02-05");
    let response_text = model.generate_content(&query).await?;
    Ok(response_text.trim().to_string())
}

#[tokio::main]
async fn main() {
    let difficulty = "Medium";
    let age = "10";
    let autism_level = "Level 1";
    let topic_focus = "Emotional Regulation";
    let treatment_plan = "";  // Empty treatment plan to trigger the default.
    let image_style = "Realistic";

    match generate_prompt_from_options(difficulty, age, autism_level, topic_focus, Some(treatment_plan), image_style).await {
        Ok(prompt) => println!("Generated prompt:\n{}", prompt),
        Err(e) => eprintln!("Error: {}", e),
    }
}
