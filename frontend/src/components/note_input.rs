use crate::components::input_component::generate_input_component;
use leptos::{IntoView, component, html::Textarea, prelude::*, view};
use shared::new_object_schemes::NewNoteScheme;
use url::Url;

#[component]
pub fn NoteInputComponent(
    payload: ReadSignal<Option<String>>,
    reload: Trigger,
    backend_url: Url,
) -> impl IntoView {
    let textarea_element: NodeRef<Textarea> = NodeRef::new();

    generate_input_component::<Textarea, NewNoteScheme>(
        textarea_element,
        view! {
            <textarea node_ref=textarea_element />
        },
        payload,
        reload,
        backend_url,
    )
}
