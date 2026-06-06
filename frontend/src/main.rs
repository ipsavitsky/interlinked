use chrono::Utc;
use js_sys::global;
use leptos::ev::SubmitEvent;
use leptos::html::{Input, Textarea};
use leptos::prelude::*;
use leptos_workers::worker;
use serde::{Deserialize, Serialize};
use shared::proof_of_work::solve_pow_challenge;
use shared::requests::fetch_difficulty;
use url::Url;
use wasm_bindgen::JsCast;

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
fn App() -> impl IntoView {
    let backend_url: Url = Url::parse("http://localhost:3000").unwrap();

    let difficulty_data = LocalResource::new(move || {
        let url = backend_url.clone();
        async move { fetch_difficulty(&url).await }
    });

    let difficulty = move || {
        difficulty_data.with(|opt| match opt {
            Some(Ok(val)) => format!("Current difficulty is {val}"),
            Some(Err(e)) => format!("Error: {e}"),
            None => "Loading...".to_string(),
        })
    };

    view! {
        <h2>{difficulty}</h2>
        {move || {
            difficulty_data
                .with(|opt| match opt {
                    Some(Ok(val)) => Some(view! { <PayloadComputationComponent difficulty=*val /> }),
                    _ => None,
                })
        }}
        <h2>"Input link"</h2>
        <LinkInputComponent />
        <h2>"Input note"</h2>
        <NoteInputComponent />
    }
}

#[component]
fn PayloadComputationComponent(difficulty: usize) -> impl IntoView {
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
            Some(Ok(res)) => res.attempt.clone(),
            Some(Err(e)) => format!("Error: {e}"),
            None => "*calculating*".to_string(),
        })
    };

    view! { <p>"Payload is: " {pow_payload}</p> }
}

#[component]
fn LinkInputComponent() -> impl IntoView {
    let (name, set_name) = signal("".to_string());
    let input_element: NodeRef<Input> = NodeRef::new();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let value = input_element.get().expect("<input> to exist").value();

        set_name.set(value);
    };

    view! {
        <form on:submit=on_submit>
            <input type="text" value=name node_ref=input_element />
            <input type="submit" value="Submit" />
        </form>
        <p>"Link is: " {name}</p>
    }
}

#[component]
fn NoteInputComponent() -> impl IntoView {
    let (name, set_name) = signal("".to_string());
    let textarea_element: NodeRef<Textarea> = NodeRef::new();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default(); // says this stops the page from reloading, why would it reload?

        let value = textarea_element.get().expect("<input> to exist").value();

        set_name.set(value);
    };

    view! {
        <form on:submit=on_submit>
            <textarea node_ref=textarea_element />
            <input type="submit" value="Submit" />
        </form>
        <p>"Link is: " {name}</p>
    }
}

fn main() {
    if global().dyn_ref::<web_sys::Window>().is_some() {
        console_error_panic_hook::set_once();
        leptos::mount::mount_to_body(App)
    }
}
