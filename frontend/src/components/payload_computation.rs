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
    reload: Trigger,
    payload: WriteSignal<Option<String>>,
) -> impl IntoView {
    let pow_data = LocalResource::new(move || {
        reload.track();
        payload.set(None);
        async move {
            tracing::debug!("calculating pow challenge");
            solve_pow(PowRequest {
                difficulty,
                seed: Utc::now().timestamp() as u64,
            })
            .await
        }
    });

    view! {
        <Suspense
            fallback=move || view! { <p>"*calculating*"</p> }
        >
            <p>"Payload is: "
                {move || Suspend::new(async move {
                    match pow_data.await {
                        Ok(res) => {
                            payload.set(Some(res.attempt.clone()));
                            res.attempt
                        }
                        Err(e) => format!("Error: {e}"),
                    }
                })}
            </p>
        </Suspense>
    }
}
