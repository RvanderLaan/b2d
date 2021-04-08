mod utils;

use wasm_bindgen::prelude::*;

mod scene;

use scene::{renderer::Renderer, scene_model::*};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn main() {
    let model = SceneModel::create();
    let renderer = Renderer{scene: model.scene};
    renderer.start()
}
