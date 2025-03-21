// Export modules to make them accessible to the rest of the crate
pub mod evaluation;
pub mod image_generation;
pub mod prompt_generation;

// Re-export commonly used items
pub use evaluation::evaluate_description;
pub use image_generation::{generate_image, ImageGenerationConfig};
pub use prompt_generation::{generate_educational_prompt, PromptOptions};