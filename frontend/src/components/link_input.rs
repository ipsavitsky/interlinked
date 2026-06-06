use leptos::{IntoView, component, html::Input, prelude::*, task::spawn_local, view};
use shared::{new_object_schemes::NewLinkScheme, requests::create_record, routes::RecordType};
use url::Url;
use web_sys::SubmitEvent;

#[component]
pub fn LinkInputComponent(payload: ReadSignal<Option<String>>) -> impl IntoView {
    let (name, set_name) = signal(None::<String>);
    let input_element: NodeRef<Input> = NodeRef::new();

    let backend_url = Url::parse("http://localhost:3000").unwrap();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let value = input_element.get().expect("<input> to exist").value();
        let backend_url = backend_url.clone();
        let value = Url::parse(&value).unwrap();
        spawn_local(async move {
            let ret = create_record(
                &backend_url,
                &NewLinkScheme {
                    payload: value,
                    challenge: payload.get().unwrap(),
                },
            )
            .await;
            match ret {
                Ok(link) => set_name.set(Some(format!(
                    "{}{}/{}",
                    backend_url,
                    RecordType::Link.route_prefix(),
                    link
                ))),
                Err(e) => set_name.set(Some(e.to_string())),
            }
        });
    };

    view! {
        <form on:submit=on_submit>
            <input type="text" value=name node_ref=input_element />
            <input type="submit" value="Submit" disabled=move || payload.get().is_none() />
        </form>
        <p>"Link is: " {name}</p>
    }
}
