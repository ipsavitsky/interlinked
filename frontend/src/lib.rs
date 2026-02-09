use shared::{
    new_object_schemes::NewLinkScheme, proof_of_work::solve_pow_challenge, requests::create_record,
};
use url::Url;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn solution_wrapper(diff: usize, seed: u64) -> String {
    let (ch, _) = solve_pow_challenge(diff, seed);
    ch
}

#[wasm_bindgen]
pub async fn new_link_record(
    backend_url: String,
    payload: String,
    challenge: String,
) -> Result<String, JsValue> {
    let backend_url_parsed = Url::parse(&backend_url)
        .map_err(|e| JsValue::from_str(&format!("Invalid backend URL: {}", e)))?;

    let payload_parsed =
        Url::parse(&payload).map_err(|e| JsValue::from_str(&format!("Invalid link URL: {}", e)))?;

    let payload_struct = NewLinkScheme {
        payload: payload_parsed,
        challenge,
    };

    create_record(&backend_url_parsed, &payload_struct)
        .await
        .map_err(|e| JsValue::from_str(&format!("Error creating record: {}", e)))
}
