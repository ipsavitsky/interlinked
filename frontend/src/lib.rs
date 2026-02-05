use shared::proof_of_work::solve_pow_challenge;
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
