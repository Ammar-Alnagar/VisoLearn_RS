use yew::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;

/// A stub for the default session value.
const DEFAULT_SESSION: &str = "default_session";
/// A stub list of image styles.
const IMAGE_STYLES: &[&str] = &["Realistic", "Illustration", "Cartoon", "Watercolor", "3D Rendering"];

#[function_component(App)]
fn app() -> Html {
    // Define application state.
    let active_session = use_state(|| DEFAULT_SESSION.to_string());
    let saved_sessions = use_state(|| Vec::<String>::new());
    let checklist_state = use_state(|| Vec::<String>::new());

    // Input states.
    let age_input = use_state(|| "3".to_string());
    let autism_level = use_state(|| "Level 1".to_string());
    let topic_focus = use_state(|| "".to_string());
    let treatment_plan = use_state(|| "".to_string());
    let attempt_limit = use_state(|| 3);
    let details_threshold = use_state(|| 70);
    let image_style = use_state(|| IMAGE_STYLES[0].to_string());
    let chat_input = use_state(|| "".to_string());
    let chatbot_history = use_state(|| Vec::<String>::new());
    let img_src = use_state(|| "".to_string());

    // Event handler stubs.
    let on_generate_click = {
        let age_input = age_input.clone();
        let autism_level = autism_level.clone();
        let topic_focus = topic_focus.clone();
        let treatment_plan = treatment_plan.clone();
        let attempt_limit = attempt_limit.clone();
        let details_threshold = details_threshold.clone();
        let active_session = active_session.clone();
        let saved_sessions = saved_sessions.clone();
        let image_style = image_style.clone();
        let img_src = img_src.clone();

        Callback::from(move |_| {
            // Here you would call your backend function (e.g. generate_image_and_reset_chat)
            // and update the state accordingly. For demo purposes we just log and update a dummy image.
            web_sys::console::log_1(&"Generate Image clicked".into());
            // Simulate generating an image (in practice, call an async API)
            img_src.set("https://via.placeholder.com/400x300.png?text=Generated+Image".to_string());
            // You could also update active_session, saved_sessions, checklist_state, etc.
        })
    };

    let on_chat_send = {
        let chat_input = chat_input.clone();
        let chatbot_history = chatbot_history.clone();
        Callback::from(move |_| {
            web_sys::console::log_1(&"Chat Send clicked".into());
            let input = (*chat_input).clone();
            if !input.is_empty() {
                // Append chat input to chatbot history.
                let mut history = (*chatbot_history).clone();
                history.push(input.clone());
                chatbot_history.set(history);
                chat_input.set("".to_string());
            }
        })
    };

    // Below we build a layout similar to your Gradio interface.
    html! {
        <div style="display: flex; flex-direction: column; gap: 2rem; padding: 1rem;">
            // Main header.
            <h1>{ "Autism Education Image Description Tool" }</h1>
            // Display current difficulty (stubbed as “Very Simple”).
            <div id="difficulty-label">
                <strong>{ "Current Difficulty: " }</strong>
                { "Very Simple" }
            </div>
            // Main content area with two columns.
            <div style="display: flex; gap: 2rem;">
                // Left Column (Image Generation & Chat)
                <div style="flex: 2;">
                    <section>
                        <h2>{ "Generate Image" }</h2>
                        <p>{ "Enter the child's details to generate an appropriate educational image." }</p>
                        <div style="display: flex; gap: 1rem;">
                            <input
                                type="text"
                                placeholder="Child's Age"
                                value={(*age_input).clone()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    age_input.set(input.value());
                                })}
                            />
                            <select onchange={Callback::from(move |e: Event| {
                                let select: HtmlInputElement = e.target_unchecked_into();
                                autism_level.set(select.value());
                            })}>
                                <option value="Level 1">{ "Level 1" }</option>
                                <option value="Level 2">{ "Level 2" }</option>
                                <option value="Level 3">{ "Level 3" }</option>
                            </select>
                        </div>
                        <input
                            type="text"
                            placeholder="Topic Focus (e.g., 'animals', 'emotions')"
                            value={(*topic_focus).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                topic_focus.set(input.value());
                            })}
                        />
                        <textarea
                            placeholder="Treatment Plan"
                            value={(*treatment_plan).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                treatment_plan.set(input.value());
                            })}
                        />
                        <div style="display: flex; gap: 1rem;">
                            <input
                                type="number"
                                value={attempt_limit.to_string()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    if let Ok(val) = input.value().parse::<i32>() {
                                        attempt_limit.set(val);
                                    }
                                })}
                            />
                            <input
                                type="range"
                                min="10"
                                max="100"
                                step="5"
                                value={details_threshold.to_string()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    if let Ok(val) = input.value().parse::<i32>() {
                                        details_threshold.set(val);
                                    }
                                })}
                            />
                        </div>
                        <select onchange={Callback::from(move |e: Event| {
                            let select: HtmlInputElement = e.target_unchecked_into();
                            image_style.set(select.value());
                        })}>
                            { for IMAGE_STYLES.iter().map(|style| html! {
                                <option value={style.to_string()}>{ style }</option>
                            })}
                        </select>
                        <div>
                            <button onclick={on_generate_click}>{ "Generate Image" }</button>
                        </div>
                        <div>
                            if !(*img_src).is_empty() {
                                <img src={(*img_src).clone()} alt="Generated Image" style="max-width: 100%;" />
                            }
                        </div>
                    </section>
                    <section>
                        <h2>{ "Image Description Practice" }</h2>
                        <p>{ "After generating an image, ask the child to describe what they see. Type their description below. The system will provide supportive feedback and track their progress." }</p>
                        <div id="chatbot" style="border: 1px solid #ccc; padding: 1rem; min-height: 100px;">
                            { for (*chatbot_history).iter().map(|msg| html! { <p>{ msg }</p> } ) }
                        </div>
                        <div style="display: flex; gap: 1rem;">
                            <input
                                type="text"
                                placeholder="Child's Description"
                                value={(*chat_input).clone()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    chat_input.set(input.value());
                                })}
                            />
                            <button onclick={on_chat_send}>{ "Submit" }</button>
                        </div>
                    </section>
                </div>
                // Right Column (Details to Identify)
                <div style="flex: 1;">
                    <h2>{ "Details to Identify" }</h2>
                    <p>{ "The child should try to identify these elements in the image:" }</p>
                    <div id="checklist-container" style="background-color: #000; color: #fff; padding: 15px; border-radius: 8px;">
                        <p>{ "Generate an image to see details to identify." }</p>
                    </div>
                    <div id="attempt-counter" style="margin-top: 10px; padding: 10px; background-color: #000; color: #fff; border-radius: 5px; border: 1px solid #444;">
                        <p style="margin: 0; font-weight: bold;">{ "Attempts: 0/3" }</p>
                    </div>
                    <div id="progress-container" style="background-color: #000; color: #fff; padding: 15px; border-radius: 8px;">
                        <p>{ "No active session." }</p>
                    </div>
                </div>
            </div>
            // Progress Tracking section.
            <section>
                <h2>{ "Progress Tracking" }</h2>
                <p>{ "This section tracks the child's progress across sessions. Each session includes the difficulty level, identified details, and the full conversation history." }</p>
                <pre>{ "{}" }</pre>
            </section>
            // Save Images and Save Session Log section.
            <section style="display: flex; gap: 2rem;">
                <div style="flex: 1;">
                    <h2>{ "Save Images" }</h2>
                    <p>{ "Click the button below to save all images from all sessions to disk." }</p>
                    <button>{ "💾 Save All Session Images" }</button>
                    <input type="text" placeholder="Save Result" readonly=true />
                </div>
                <div style="flex: 1;">
                    <h2>{ "Save Session Log" }</h2>
                    <p>{ "Click the button below to save the complete session log as a JSON file." }</p>
                    <button>{ "📝 Save Session Log" }</button>
                    <input type="text" placeholder="Save Log Result" readonly=true />
                </div>
            </section>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
