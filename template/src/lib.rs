use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point of your app
#[wasm_bindgen(start)]
pub fn start() {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let node = util::document().query_selector("#app").unwrap().unwrap();
    let app = smithy::smd!(
      <h1>Hello, Smithy!</h1>
      <p>
        Thank you for trying Smithy!
      </p>
      <p>
        Take a look at <code>src/lib.rs</code> and start editing!
      </p>
    );
    smithy::mount(Box::new(app), node);

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));
}
