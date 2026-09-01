pub mod adapters;
#[cfg(feature = "ssr")]
pub mod app;
pub mod domain;
pub mod layout;
#[cfg(feature = "ssr")]
pub mod pages;

pub mod components;

use cfg_if::cfg_if;

cfg_if! {
  if #[cfg(feature = "hydrate")] {

    use wasm_bindgen::prelude::wasm_bindgen;

      #[wasm_bindgen]
      pub fn hydrate() {

        console_error_panic_hook::set_once();

        leptos::mount::hydrate_islands();
      }
  }
}
