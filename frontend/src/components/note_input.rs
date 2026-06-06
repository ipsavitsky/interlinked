use leptos::{IntoView, component, html::Textarea, prelude::*, task::spawn_local, view};
use shared::{new_object_schemes::NewNoteScheme, requests::create_record, routes::RecordType};
use url::Url;
use web_sys::SubmitEvent;

#[component]
pub fn NoteInputComponent(payload: ReadSignal<Option<String>>) -> impl IntoView {
    let (name, set_name) = signal(None::<String>);
    let textarea_element: NodeRef<Textarea> = NodeRef::new();

    let backend_url = Url::parse("http://localhost:3000").unwrap();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default(); // says this stops the page from reloading, why would it reload?
        let value = textarea_element.get().expect("<input> to exist").value();
        let backend_url = backend_url.clone();
        spawn_local(async move {
            let ret = create_record(
                &backend_url,
                &NewNoteScheme {
                    payload: value,
                    challenge: payload.get().unwrap(),
                },
            )
            .await;
            match ret {
                Ok(link) => set_name.set(Some(format!(
                    "{}{}/{}",
                    backend_url,
                    RecordType::Note.route_prefix(),
                    link
                ))),
                Err(e) => set_name.set(Some(e.to_string())),
            }
        });
    };

    view! {
        <form on:submit=on_submit>
            <textarea node_ref=textarea_element />
            <input type="submit" value="Submit" disabled=move || payload.get().is_none() />
        </form>
        <p>"Link is: " {name}</p>
    }
}
