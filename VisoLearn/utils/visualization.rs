use std::collections::HashMap;
use serde_json::{Value, json};

/// Updates the difficulty label based on the active session
pub fn update_difficulty_label(active_session: &Value) -> String {
    let difficulty = active_session.get("difficulty")
        .and_then(|d| d.as_str())
        .unwrap_or("Very Simple");
    
    format!("**Current Difficulty:** {}", difficulty)
}

/// Updates the checklist HTML based on the provided checklist items
pub fn update_checklist_html(checklist: &[Value]) -> String {
    if checklist.is_empty() {
        return r#"
            <div id="checklist-container" style="background-color: #000000; color: #ffffff; padding: 15px; border-radius: 8px;">
                <p>Generate an image to see details to identify.</p>
            </div>
        "#.to_string();
    }

    let mut html_content = r#"
        <div id="checklist-container" style="background-color: #000000; color: #ffffff; padding: 15px; border-radius: 8px;">
            <style>
                .checklist-item {
                    display: flex;
                    align-items: center;
                    margin-bottom: 10px;
                    padding: 8px;
                    border-radius: 5px;
                    transition: background-color 0.3s;
                }
                .identified {
                    background-color: #1e4620;
                    text-decoration: line-through;
                    color: #7fff7f;
                }
                .not-identified {
                    background-color: #222222;
                    color: #ffffff;
                }
                .checkmark {
                    margin-right: 10px;
                    font-size: 1.2em;
                }
            </style>
    "#.to_string();

    for item in checklist {
        let detail = item.get("detail")
            .and_then(|d| d.as_str())
            .unwrap_or("Unknown detail");
        
        let identified = item.get("identified")
            .and_then(|i| i.as_bool())
            .unwrap_or(false);
        
        let css_class = if identified { "identified" } else { "not-identified" };
        let checkmark = if identified { "✅" } else { "❌" };
        
        html_content.push_str(&format!(r#"
            <div class="checklist-item {}">
                <span class="checkmark">{}</span>
                <span>{}</span>
            </div>
        "#, css_class, checkmark, detail));
    }

    html_content.push_str(r#"
        </div>
    "#);
    
    html_content
}

/// Updates the progress HTML based on the checklist and active session
pub fn update_progress_html(checklist: &[Value], active_session: &Value) -> String {
    if checklist.is_empty() {
        return r#"
            <div id="progress-container" style="background-color: #000000; color: #ffffff; padding: 15px; border-radius: 8px;">
                <p>No active session.</p>
            </div>
        "#.to_string();
    }

    let total_items = checklist.len();
    let identified_items = checklist.iter()
        .filter(|item| item.get("identified").and_then(|i| i.as_bool()).unwrap_or(false))
        .count();
    
    let percentage = if total_items > 0 {
        (identified_items as f64 / total_items as f64) * 100.0
    } else {
        0.0
    };
    
    let progress_bar_width = format!("{}%", percentage);

    // Calculate threshold
    let details_threshold = active_session.get("details_threshold")
        .and_then(|t| t.as_f64())
        .unwrap_or(0.7);
    
    let threshold_count = (total_items as f64 * details_threshold).ceil() as usize;
    let threshold_percentage = (threshold_count as f64 / total_items as f64) * 100.0;

    let mut html_content = format!(r#"
        <div id="progress-container" style="background-color: #000000; color: #ffffff; padding: 15px; border-radius: 8px;">
            <h3>Progress: {identified_items} / {total_items} details</h3>
            <div style="width: 100%; background-color: #333333; border-radius: 5px; margin-bottom: 10px; position: relative;">
                <div style="width: {progress_bar_width}; height: 24px; background-color: #4CAF50; border-radius: 5px;"></div>
                <div style="position: absolute; top: 0; bottom: 0; left: {threshold_percentage}%; width: 2px; background-color: #ff6b6b;"></div>
                <div style="position: absolute; top: -15px; left: {percentage_offset}%; color: #ff6b6b; font-weight: bold;">⚠️</div>
            </div>
            <p style="font-size: 14px; text-align: center; color: #dddddd;">
                Need to identify at least {threshold_count} details ({threshold_percent}%) to advance
            </p>
            <p style="font-size: 16px; font-weight: bold; text-align: center; color: #ffffff;">
    "#, 
        identified_items = identified_items,
        total_items = total_items,
        progress_bar_width = progress_bar_width,
        threshold_percentage = threshold_percentage,
        percentage_offset = threshold_percentage - 5.0,
        threshold_count = threshold_count,
        threshold_percent = (details_threshold * 100.0) as i32
    );

    let message = if identified_items >= threshold_count {
        "🎉 Threshold reached! Ready to advance! 🎉"
    } else if percentage >= 75.0 {
        "Almost there! Keep going!"
    } else if percentage >= 50.0 {
        "Halfway there! You're doing great!"
    } else if percentage >= 25.0 {
        "Good start! Keep looking!"
    } else {
        "Let's find more details!"
    };

    html_content.push_str(message);
    html_content.push_str(r#"
            </p>
        </div>
    "#);
    
    html_content
}

/// Updates the attempt counter based on the active session
pub fn update_attempt_counter(active_session: &Value) -> String {
    let current_count = active_session.get("attempt_count")
        .and_then(|c| c.as_i64())
        .unwrap_or(0);
    
    let limit = active_session.get("attempt_limit")
        .and_then(|l| l.as_i64())
        .unwrap_or(3);
    
    format!(r#"
        <div id="attempt-counter" style="margin-top: 10px; padding: 10px; background-color: #000000; color: #ffffff; border-radius: 5px; border: 1px solid #444;">
            <p style="margin: 0; font-weight: bold; text-align: center;">Attempts: {current_count}/{limit}</p>
        </div>
    "#)
}

// Example usage function
pub fn example_usage() {
    // Create a sample active session
    let active_session = json!({
        "difficulty": "Medium",
        "details_threshold": 0.7,
        "attempt_count": 1,
        "attempt_limit": 3
    });

    // Create a sample checklist
    let checklist = vec![
        json!({
            "detail": "Red flower in the corner",
            "identified": true
        }),
        json!({
            "detail": "Blue sky background",
            "identified": true
        }),
        json!({
            "detail": "Mountain silhouette",
            "identified": false
        }),
        json!({
            "detail": "Small bird in flight",
            "identified": false
        })
    ];

    // Use the functions
    let difficulty_label = update_difficulty_label(&active_session);
    let checklist_html = update_checklist_html(&checklist);
    let progress_html = update_progress_html(&checklist, &active_session);
    let attempt_counter = update_attempt_counter(&active_session);

    println!("Difficulty Label: {}", difficulty_label);
    println!("Checklist HTML length: {} chars", checklist_html.len());
    println!("Progress HTML length: {} chars", progress_html.len());
    println!("Attempt Counter HTML length: {} chars", attempt_counter.len());
}