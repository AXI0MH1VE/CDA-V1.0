mod lambda_core;
mod phi_layer;
mod tools;
mod multimodal;

use lambda_core::ConstitutionalCore;
use phi_layer::PhiLayer;
use std::sync::Mutex;
use tauri::State;

// App state containing the constitutional core and the phi layer
struct AppState {
    lambda_core: Mutex<ConstitutionalCore>,
    phi_layer: Mutex<PhiLayer>,
}

#[tauri::command]
async fn process_query(
    query: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    // Validate query with λ Core
    let validated_prompt = state.lambda_core.lock().unwrap().validate_query(&query)
        .map_err(|e| format!("Query validation failed: {:?}", e))?;

    // Generate response with Φ Layer
    let response = state.phi_layer.lock().unwrap().generate(validated_prompt).await
        .map_err(|e| format!("Response generation failed: {:?}", e))?;

    Ok(response)
}

use tauri::{Window, Manager};
// ... (rest of the imports)

#[tauri::command]
async fn stream_query(
    query: String,
    state: State<'_, AppState>,
    window: Window,
) -> Result<(), String> {
    let validated_prompt = state.lambda_core.lock().unwrap().validate_query(&query)
        .map_err(|e| format!("Query validation failed: {:?}", e))?;

    let mut phi_layer = state.phi_layer.lock().unwrap();
    
    let mut tokens = phi_layer.model.tokenizer.encode(validated_prompt.content.as_str(), true).map_err(|e| e.to_string())?.get_ids().to_vec();
    let mut generated_tokens = 0;
    let max_tokens = 100;
    let mut logits_processor = LogitsProcessor::new(299792458, Some(0.7), Some(0.9));

    for index in 0..max_tokens {
        let context_size = if index > 0 { 1 } else { tokens.len() };
        let start_pos = tokens.len().saturating_sub(context_size);
        let input = Tensor::new(&tokens[start_pos..], &phi_layer.model.device).unwrap().unsqueeze(0).unwrap();
        let logits = phi_layer.model.model.forward(&input, start_pos).unwrap();
        let logits = logits.squeeze(0).unwrap();
        let next_token = logits_processor.sample(&logits).unwrap();
        tokens.push(next_token);
        
        if let Some(text) = phi_layer.model.tokenizer.decode(&[next_token], true).map_err(|e| e.to_string())?.into() {
            window.emit("stream-response", text).unwrap();
            if text.contains("<|endoftext|>") || text.contains("</s>") {
                break;
            }
        }
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenvy::dotenv().ok();
    let lambda_core = ConstitutionalCore::new();
    let phi_layer = PhiLayer::new().expect("Failed to initialize PhiLayer");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            lambda_core: Mutex::new(lambda_core),
            phi_layer: Mutex::new(phi_layer),
        })
        .invoke_handler(tauri::generate_handler![process_query, stream_query])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
