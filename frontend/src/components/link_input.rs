use leptos::{IntoView, component, html::Input, prelude::*, view};
use shared::new_object_schemes::NewLinkScheme;
use url::Url;

use crate::components::input_component::generate_input_component;

#[component]
pub fn LinkInputComponent(
    payload: ReadSignal<Option<String>>,
    reload: Trigger,
    backend_url: Url,
) -> impl IntoView {
    let input_element: NodeRef<Input> = NodeRef::new();

    generate_input_component::<Input, NewLinkScheme>(
        input_element,
        view! {<input type="text" node_ref=input_element />},
        payload,
        reload,
        backend_url,
    )
}
