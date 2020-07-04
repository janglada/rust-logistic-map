use wasm_bindgen::prelude::*;
use web_sys::{console};
use js_sys::Array;




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
        let r = n as f32 * 1.0/steps as f32;
        let values : Array = Array::new();
        for j in 1..256 {
            values.push(&wasm_bindgen::JsValue::from_f64(logistic(r, (j as f32)/256.0)as f64) );

        }
        js_array.push(&wasm_bindgen::JsValue::from(values));
    }
    Ok(js_array)
}


pub fn logistic(r:f32, x0: f32) -> f32 {
    let  mut x:f32= x0;
    for n in 1..256 {
        x = r*x*(1. - x);
    }
    (((x*1000.0) as i32) as f32/1000.0) as f32
}

