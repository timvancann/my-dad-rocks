pub mod app;
pub mod components;
#[cfg(feature = "ssr")]
pub mod database;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;
pub mod models;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    _ = console_log::init_with_level(log::Level::Debug);
    leptos::mount_to_body(App);
}
