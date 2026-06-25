use components::app::App;
use js_sys::global;
use tracing_web::MakeWebConsoleWriter;
use wasm_bindgen::JsCast;

pub mod components;

fn main() {
    if global().dyn_ref::<web_sys::Window>().is_some() {
        tracing_subscriber::fmt::fmt()
            .compact()
            .with_ansi(false)
            .without_time()
            .with_writer(MakeWebConsoleWriter::new())
            .with_line_number(true)
            .with_thread_ids(true)
            .with_max_level(tracing::Level::DEBUG)
            .init();
        console_error_panic_hook::set_once();
        leptos::mount::mount_to_body(App)
    }
}
