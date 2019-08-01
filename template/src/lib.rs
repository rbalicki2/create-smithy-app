#![feature(proc_macro_hygiene, slice_patterns)]

use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn get_root_element() -> Result<web_sys::Element, JsValue> {
    web_sys::window()
        .and_then(|w| w.document())
        // N.B. query_selector returns Result<Option<Element>>
        // So, calling .ok() on that converts it to an Option<Option<Element>>
        // and hence, we must call .ok_or() twice.
        .and_then(|d| d.query_selector("#app").ok())
        .ok_or(JsValue::NULL)?
        .ok_or(JsValue::NULL)
}

// This is the entry point of your app
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let root_element = get_root_element()?;

    let app = smithy::smd!(
        <h1>Hello, Smithy!</h1>
        <p>
            Thank you for trying Smithy!
        </p>
        <p>
            Take a look at <code>src/lib.rs</code>. Happy hacking!
        </p>
    );
    smithy::mount(Box::new(app), root_element);

    console::log_1(&JsValue::from_str(
        "Welcome to Smithy! Head to `src/lib.rs`. Happy hacking!",
    ));

    Ok(())
}
