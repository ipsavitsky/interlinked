use components::app::App;
use js_sys::global;
use wasm_bindgen::JsCast;

pub mod components;

fn main() {
    if global().dyn_ref::<web_sys::Window>().is_some() {
        console_error_panic_hook::set_once();
        leptos::mount::mount_to_body(App)
    }
}
