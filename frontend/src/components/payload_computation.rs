use chrono::Utc;
use leptos::{IntoView, component, prelude::*, server::LocalResource, view};
use leptos_workers::worker;
use serde::{Deserialize, Serialize};
use shared::proof_of_work::solve_pow_challenge;

#[derive(Clone, Serialize, Deserialize)]
struct PowRequest {
    difficulty: usize,
    seed: u64,
}

#[derive(Clone, Serialize, Deserialize)]
struct PowResponse {
    attempt: String,
}

#[worker(PowWorker)]
fn solve_pow(req: PowRequest) -> PowResponse {
    let (attempt, _) = solve_pow_challenge(req.difficulty, req.seed);
    PowResponse { attempt }
}

#[component]
pub fn PayloadComputationComponent(
    difficulty: usize,
    payload: WriteSignal<Option<String>>,
) -> impl IntoView {
    let pow_data = LocalResource::new(move || {
        let difficulty = difficulty;
        async move {
            solve_pow(PowRequest {
                difficulty,
                seed: Utc::now().timestamp() as u64,
            })
            .await
        }
    });

    let pow_payload = move || {
        pow_data.with(|opt| match opt {
            Some(Ok(res)) => {
                payload.set(Some(res.attempt.clone()));
                res.attempt.clone()
            }
            Some(Err(e)) => format!("Error: {e}"),
            None => "*calculating*".to_string(),
        })
    };

    view! { <p>"Payload is: " {pow_payload}</p> }
}
