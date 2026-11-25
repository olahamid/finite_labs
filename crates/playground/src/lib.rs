//! Browser demos, compiled to WebAssembly.
//!
//! `start` runs automatically when `static/playground.js` loads the module. It
//! looks for demo containers on the page and wires up the ones it finds, so the
//! same bundle serves the homepage widget and (later) the playground pages.
//!
//! All cryptography goes through `crates/crypto`. Nothing typed into a demo
//! ever leaves the browser (PRD §6.4 rule 2): this crate makes no network calls.

#![forbid(unsafe_code)]

pub mod live_proof;
pub mod tamper;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("no window")?;
    let document = window.document().ok_or("no document")?;

    if let Some(root) = document.get_element_by_id("live-proof") {
        live_proof::mount(&window, &document, root)?;
    }
    Ok(())
}
