use wasm_bindgen::prelude::*;
use shared::come_up_with_solution;

#[wasm_bindgen]
pub fn solution_wrapper(diff: usize) -> String {
    let (ch, _) = come_up_with_solution(diff);
    ch
}
