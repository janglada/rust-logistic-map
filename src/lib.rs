use wasm_bindgen::prelude::*;
use web_sys::{console};
use js_sys::Array;
use js_sys::Set;
use wasm_bindgen::__rt::std::collections::HashSet;
use std::collections::BTreeSet;
use std::cmp::Ordering;




// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen]
pub fn run(steps:i32) -> Result<Array, JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let js_array : Array = Array::new();
    let MAX = (4*steps) as i32;
    for n in 1..MAX
    {
        let r = n as f64 * 1.0/steps as f64;
        let values : Array = Array::new();
        for j in 1..256 {
            values.push(&wasm_bindgen::JsValue::from_f64(logistic(r, (j as f64)/256.0)));

        }
        js_array.push(&wasm_bindgen::JsValue::from(values));
    }
    Ok(js_array)
}


pub fn logistic(r:f64, x0: f64) -> f64 {
    let  mut x:f64= x0;
    for n in 1..100 {
        x = r*x*(1. - x);
    }
    x
}

