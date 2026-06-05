use leptos::ev::SubmitEvent;
use leptos::html::{Input, Textarea};
use leptos::prelude::*;
use leptos::task::spawn_local;
use shared::proof_of_work::solve_pow_challenge;
use shared::requests::fetch_difficulty;
use url::Url;

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
        <PayloadComputationComponent />
        <h2>"Input link"</h2>
        <LinkInputComponent />
        <h2>"Input note"</h2>
        <NoteInputComponent />
    }
}

#[component]
fn PayloadComputationComponent() -> impl IntoView {
    let (pending, set_pending) = signal(true);
    let (payload, set_payload) = signal(String::new());

    spawn_local(async move {
        let (res, _) = solve_pow_challenge(2, 0);
        set_payload.set(res);
        set_pending.set(false);
    });

    view! {
        <p>
            "Payload is: "
            {move || if pending.get() { "*calculating*".to_string() } else { payload.get() }}
        </p>
    }
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

// This `main` function is the entry point into the app
// It just mounts our component to the <body>
// Because we defined it as `fn App`, we can now use it in a
// template as <App/>
fn main() {
    leptos::mount::mount_to_body(App)
}
