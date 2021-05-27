use macroquad::{prelude::*, ui::Skin};

use macroquad::ui::{
  hash, root_ui,
  widgets::{self, Group, TreeNode},
  Id, Ui,
};

use egui_macroquad::*;

// use egui_macroquad::{}

mod scene;
use scene::scene_model::*;

mod editor;
use editor::editor::*;

mod profiler;

fn short_angle_dist(a0: f32, a1: f32) -> f32 {
  let max = 360.0;
  let da = (a1 - a0) % max;
  2.0 * da % max - da
}

fn angle_lerp(a0: f32, a1: f32, t: f32) -> f32 {
  a0 + short_angle_dist(a0, a1) * t
}

fn draw_cross(x: f32, y: f32, color: Color) {
  let size = 0.1;
  let thickness = 0.005;
  draw_line(x - size, y, x + size, y, thickness, color);
  draw_line(x, y - size, x, y + size, thickness, color);
}

#[macroquad::main("New scene* • B2D")]
async fn main() {
  let mut model = SceneModel::create();
  // let scene = &mut model.scene;

  let mut editor = Editor::default();
  let mut editor_ui = EditorUI::default();

  let mut screen_rect = Rect {
    x: 0.,
    y: 0.,
    w: screen_width(),
    h: screen_height(),
  };
  // set_camera(scene.main_camera.set_resolution(screen_rect));

  // JS bindings are available: https://github.com/not-fl3/quad-net/tree/master/examples
  // could do web-rtc after all. Or do it through PeerJS

  // TODO: next-up: create UI.
  // - Menu for adding shape (Shift+A)
  // - Outliner panel
  //  - List/tree of shapes
  //  - Materials
  // - UI focus context: clicking in world vs. clicking in UI
  // - Selection (active object)
  // - Transform tools
  //   - Grab/Rotate/Scale
  // https://github.com/not-fl3/macroquad/blob/master/examples/events.rs
  // https://github.com/not-fl3/macroquad/blob/master/examples/ui.rs

  loop {
    model.scene.update();

    clear_background(RED);

    // root_ui().push_skin(&skin);
    // gui(&mut model.scene);
    // root_ui().pop_skin();

    screen_rect.w = screen_width();
    screen_rect.h = screen_height();

    // cam2d.offset = -scene.main_camera.transform.location;
    // cam2d.rotation = scene.main_camera.transform.rotation;
    // cam2d.zoom = vec2(1. / screen_rect.w * 2., -1. / screen_rect.h * 2.) * scene.main_camera.zoom;
    // set_camera(scene.main_camera.set_resolution(screen_rect));

    draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
    draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
    draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

    // TODO:
    draw_grid(32, 1.);

    for shape in model.scene.shapes.iter() {
      draw_mesh(&shape.get_mesh());
      // If edit mode is enabled, and active object is this shape,
      // - draw a point at every point!
      // - draw a line at every edge!
      // - draw a face at every face!
    }

    // draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

    set_default_camera();
    draw_text(
      &format!(
        "x {:.2}, y{:.2}",
        model.scene.main_camera.cam2d().target.x,
        model.scene.main_camera.cam2d().target.y
      ),
      20.0,
      20.0,
      30.0,
      DARKGRAY,
    );

    // Finally, draw the GUI using EGUI (Dear Imgui rust alternative, runs in WASM)
    // editor.update();

    editor_ui.update(&mut editor);
    egui_macroquad::draw();

    profiler::profiler(Default::default());

    next_frame().await
  }
}
