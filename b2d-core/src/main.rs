
use macroquad::{prelude::*, ui::Skin};

use macroquad::ui::{
  hash, root_ui,
  widgets::{self, Group, TreeNode},
  Id, Ui,
};

use std::borrow::BorrowMut;
use std::collections;
use std::ops::Deref;

use egui_macroquad::*;

// use egui_macroquad::{}

mod scene;
use scene::scene_model::*;

mod editor;
use editor::editor::*;
use editor::operators::*;

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

/// Draw a grid centered at (0, 0, 0)
pub fn draw_grid_2d(slices: u32, spacing: f32) {
  let half_slices = (slices as i32) / 2;
  for i in -half_slices..half_slices + 1 {
      let color = if i == 0 {
          Color::new(0.55, 0.55, 0.55, 0.75)
      } else {
          Color::new(0.75, 0.75, 0.75, 0.75)
      };

      draw_line_3d(
          vec3(i as f32 * spacing, -half_slices as f32 * spacing, 0.),
          vec3(i as f32 * spacing, half_slices as f32 * spacing, 0.),
          color,
      );
      draw_line_3d(
          vec3(-half_slices as f32 * spacing,  i as f32 * spacing, 0.),
          vec3(half_slices as f32 * spacing,  i as f32 * spacing, 0.),
          color,
      );
  }
}

#[derive(Default)]
struct OperatorController {
  undo_stack: Vec<Box<dyn Operator>>,
  redo_stack: Vec<Box<dyn Operator>>,
  max_stack_size: usize,
}

impl OperatorController {
  fn run_operator(&mut self, operator: &mut Box<dyn Operator>, editor: &mut Editor) {
    // let x = operator.deref();
    // self.undo_stack.push(operator.into()); TODO:

    operator.perform(editor);

    if self.undo_stack.len() > self.max_stack_size {
      // self.undo_stack.drain(self.max_stack_size);
      // TODO: truncate stack, but on the left, not the right
    }
  }
  fn undo(&mut self, editor: &mut Editor) {
    if let Some(op) = &mut self.undo_stack.pop() {
      // self.redo_stack.push(op);
      op.undo(editor);
    }
  }
  fn redo(&mut self, editor: &mut Editor) {
    if let Some(op) = &mut self.redo_stack.pop() {
      // self.undo_stack.push(op);
      op.perform(editor);
    }
  }
}

#[macroquad::main("New scene* • B2D")]
async fn main() -> ! {
  // let scene = &mut model.scene;

  let mut editor = Editor::default();
  let mut editor_ui = EditorUI::default();
  let mut controller = OperatorController::default();

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
    editor.scene.update();

    clear_background(Color::from_rgba(25, 25, 25, 255));

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
    draw_grid_2d(32, 1.);
    draw_rectangle_lines(-16., -16., 32., 32., 0.5, Color::from_rgba(255, 0, 0, 255));

    for shape in editor.scene.shapes.iter() {
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
        editor.scene.main_camera.cam2d().target.x,
        editor.scene.main_camera.cam2d().target.y
      ),
      20.0,
      20.0,
      30.0,
      DARKGRAY,
    );
    draw_text(
      &format!(
        "Mode: {}",
        editor.mode,
      ),
      20.0,
      50.0,
      30.0,
      DARKGRAY,
    );

    // TODO: put things in here
    // editor.update();

    if is_key_down(KeyCode::LeftControl) {
      if is_key_down(KeyCode::Z) {
        controller.undo(&mut editor);
      } else if is_key_down(KeyCode::Y) {
        controller.redo(&mut editor);
      }
    } else if is_key_down(KeyCode::Tab) {
      if editor.mode == EditorMode::Object {
        controller.run_operator(&mut Box::new(ToggleEditMode{}), &mut editor)
      } else if editor.mode == EditorMode::EditMesh {
        // controller.run_operator(Box::new(ToggleObjectMode{}), &mut editor)

      }
    }

    // Finally, update the GUI using EGUI (Dear Imgui rust alternative, runs in WASM)
    editor_ui.update(&mut editor);

    // and draw it
    egui_macroquad::draw();

    profiler::profiler(Default::default());

    next_frame().await
  }
}
