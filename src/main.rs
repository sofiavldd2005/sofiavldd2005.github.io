mod app;
mod pages;
use app::App;
use leptos::mount::mount_to_body;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> })
}
