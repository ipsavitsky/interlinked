use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! { "Hello world!" }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
