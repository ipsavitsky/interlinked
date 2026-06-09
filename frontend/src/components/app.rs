use crate::components::{LinkInputComponent, NoteInputComponent, PayloadComputationComponent};
use leptos::{IntoView, component, prelude::*, server::LocalResource, view};
use shared::requests::fetch_difficulty;
use url::Url;

#[component]
pub fn App() -> impl IntoView {
    let (payload, set_payload) = signal(None::<String>);
    let backend_url = Url::parse(&window().origin()).unwrap();

    let value_for_prop = backend_url.clone();
    let value_for_resource = value_for_prop.clone();
    let difficulty_data = LocalResource::new(move || {
        let url = value_for_resource.clone();
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
                    Some(Ok(val)) => {
                        Some(
                            view! {
                                <PayloadComputationComponent difficulty=*val payload=set_payload />
                            },
                        )
                    }
                    _ => None,
                })
        }}
        <h2>"Input link"</h2>
        <LinkInputComponent payload=payload backend_url=value_for_prop />
        <h2>"Input note"</h2>
        <NoteInputComponent payload=payload backend_url=backend_url />
    }
}
