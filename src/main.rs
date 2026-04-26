//! Entry point for the ADR Timeline WASM application.

mod app;
mod canvas;
mod components;
mod models;
mod utils;

use app::App;

/// Application entry point — mounts the Leptos app to the document body.
fn main() {
  console_error_panic_hook::set_once();
  leptos::mount::mount_to_body(App);
}
