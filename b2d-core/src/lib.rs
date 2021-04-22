mod utils;

use wasm_bindgen::prelude::*;

mod scene;

use scene::{renderer::Renderer, scene_model::*};
use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn main() {
    let mut model = SceneModel::create();

    // Focusing on the renderer now, commented out all web-rtc stuff
    // let scene = 
    let renderer = Renderer::create(model.get_scene());
    let res = renderer.start();
    if res.is_err() {
        console::log_1(&res.err().into());

    }
    // !(res);
}
