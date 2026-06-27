use leptos::{IntoView, html::ElementType, prelude::*, task::spawn_local};
use serde::Serialize;
use shared::{new_object_schemes::RecordPayload, requests::create_record};
use url::Url;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlTextAreaElement, SubmitEvent, window};

pub trait InputValue {
    fn value(&self) -> String;
    fn set_value(&self, value: &str);
}

impl InputValue for HtmlInputElement {
    fn value(&self) -> String {
        HtmlInputElement::value(self)
    }
    fn set_value(&self, value: &str) {
        HtmlInputElement::set_value(self, value)
    }
}

impl InputValue for HtmlTextAreaElement {
    fn value(&self) -> String {
        HtmlTextAreaElement::value(self)
    }
    fn set_value(&self, value: &str) {
        HtmlTextAreaElement::set_value(self, value)
    }
}

pub fn generate_input_component<E, S>(
    input_element: NodeRef<E>,
    input_html: impl IntoView,
    payload: ReadSignal<Option<String>>,
    reload: Trigger,
    backend_url: Url,
) -> impl IntoView
where
    E: ElementType,
    <E as ElementType>::Output: JsCast + Clone + 'static + InputValue,
    S: RecordPayload + Serialize,
{
    let (link, set_link) = signal(None::<String>);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let value = input_element.get().expect("input element to exist").value();
        let backend_url = backend_url.clone();
        spawn_local(async move {
            let ret = create_record(
                &backend_url,
                &S::from_parts(value, payload.get_untracked().unwrap()),
            )
            .await;
            reload.notify();
            match ret {
                Ok(link) => set_link.set(Some(
                    backend_url
                        .join(&format!("{}/{}", S::record_type().route_prefix(), link))
                        .unwrap()
                        .to_string(),
                )),
                Err(e) => set_link.set(Some(e.to_string())),
            }
        });
    };

    let copy_url = move |_| {
        if let Some(window) = window() {
            match link.get_untracked() {
                Some(name_str) => {
                    let _ = window.navigator().clipboard().write_text(&name_str);
                }
                None => {
                    tracing::warn!("No name to copy");
                }
            }
        }
    };

    let paste_url = move |_| {
        let textarea = match input_element.get() {
            Some(t) => t,
            None => {
                tracing::warn!("input element not mounted yet");
                return;
            }
        };
        let Some(window) = window() else { return };
        spawn_local(async move {
            let promise = window.navigator().clipboard().read_text();
            match wasm_bindgen_futures::JsFuture::from(promise).await {
                Ok(value) => match value.as_string() {
                    Some(text) => textarea.set_value(&text),
                    None => tracing::warn!("clipboard value was not a string"),
                },
                Err(e) => tracing::warn!("failed to read clipboard: {e:?}"),
            }
        });
    };

    view! {
        <button on:click=paste_url>paste</button>

        <form on:submit=on_submit>
            {input_html}
            <input type="submit" value="Submit" disabled=move || payload.get().is_none() />
        </form>

        <p>"Link is: " {link}</p>
        <button on:click=copy_url disabled=move || link.get().is_none()>copy</button>
    }
}
