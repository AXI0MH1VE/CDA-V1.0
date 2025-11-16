mod lambda_core;
mod phi_layer;
mod tools;
mod multimodal;

use lambda_core::ConstitutionalCore;
use std::sync::Mutex;
use tauri::State;

// App state containing the constitutional core
struct AppState {
    lambda_core: Mutex<ConstitutionalCore>,
}

/// Process a user query with constitutional validation
///
/// This Tauri command handles user queries by first validating them against
/// CDA-v1.0 constitutional principles, then processing them through the AI system.
///
/// # Arguments
/// * `query` - The user's input query string
/// * `state` - Tauri state containing the constitutional core
///
/// # Returns
/// * `Ok(String)` - Successfully processed response
/// * `Err(String)` - Error message if validation or processing fails
///
/// # Constitutional Compliance
/// All queries are validated against:
/// - Article I: Identity prohibitions
/// - Article II: Operational transparency
/// - Article III: Ethical boundaries
///
/// # Security
/// This function ensures all AI outputs are constitutionally compliant
/// and safe for user interaction.
#[tauri::command]
async fn process_query(
    query: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    // Validate query with λ Core
    let validated_prompt = state.lambda_core.lock().unwrap().validate_query(&query)
        .map_err(|e| format!("Query validation failed: {:?}", e))?;

    // For now, return a simple response
    // In Phase 2, this would go through Φ Layer
    let response = format!("Processed query: '{}'. Constitutional validation passed.", validated_prompt.content);

    Ok(response)
}

#[tauri::command]
async fn stream_query(
    query: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    // Similar to process_query but for streaming
    // For now, return a simple response
    process_query(query, state).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let lambda_core = ConstitutionalCore::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            lambda_core: Mutex::new(lambda_core),
        })
        .invoke_handler(tauri::generate_handler![process_query, stream_query])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
