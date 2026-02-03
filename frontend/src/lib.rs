use shared::proof_of_work::come_up_with_solution;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn solution_wrapper(diff: usize, seed: u64) -> String {
    let (ch, _) = come_up_with_solution(diff, seed);
    ch
}
