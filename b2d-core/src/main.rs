use macroquad::{prelude::*, ui::Skin};

use macroquad::ui::{
  hash, root_ui,
  widgets::{self, Group},
  Drag, Ui,
};

mod scene;
use scene::scene_model::*;

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

fn update() {}

struct GUI {}

fn create_gui_skin() -> Skin {
  let font_bytes = include_bytes!("./assets/OpenSans-Regular.ttf");
  let label_style = root_ui()
    .style_builder()
    .font(font_bytes)
    .font_size(16)
    .build();

  let window_style = root_ui()
    .style_builder()
    .font(font_bytes)
    .font_size(24)
    .build();

  let button_style = root_ui()
    .style_builder()
    .font(font_bytes)
    .font_size(24)
    .build();

  let editbox_style = root_ui()
    .style_builder()
    .font(font_bytes)
    .font_size(24)
    .build();

  Skin {
    editbox_style,
    window_style,
    // button_style,
    label_style,
    ..root_ui().default_skin()
  }
}

fn gui() {
  widgets::Window::new(hash!(), vec2(400., 200.), vec2(320., 400.))
    .label("Shop")
    .titlebar(true)
    .ui(&mut *root_ui(), |ui| {
      for i in 0..30 {
        Group::new(hash!("shop", i), Vec2::new(300., 80.)).ui(ui, |ui| {
          ui.label(Vec2::new(10., 10.), &format!("Item N {}", i));
          ui.label(Vec2::new(260., 40.), "10/10");
          ui.label(Vec2::new(200., 58.), &format!("{} kr", 800));
          if ui.button(Vec2::new(260., 55.), "buy") {
            // data.inventory.push(format!("Item {}", i));
          }
        });
      }
    });

  widgets::Window::new(hash!(), vec2(100., 220.), vec2(542., 430.))
    .label("Fitting window")
    .titlebar(true)
    .ui(&mut *root_ui(), |ui| {
      Group::new(hash!(), Vec2::new(230., 400.)).ui(ui, |ui| {
        // data.slots(ui);
      });
      Group::new(hash!(), Vec2::new(280., 400.)).ui(ui, |ui| {
        // data.inventory(ui);
      });
    });

  widgets::Window::new(hash!(), vec2(470., 50.), vec2(300., 300.))
    .label("Outliner")
    .ui(&mut *root_ui(), |ui| {
      ui.tree_node(hash!(), "Scene", |ui| {
        ui.label(None, "Some random text");
        if ui.button(None, "click me") {
          println!("hi");
        }

        ui.separator();

        ui.label(None, "Some other random text");
        if ui.button(None, "other button") {
          println!("hi2");
        }

        ui.separator();

        // ui.input_text(hash!(), "<- input text 1", &mut data0);
        // ui.input_text(hash!(), "<- input text 2", &mut data1);
        // ui.label(
        //     None,
        //     &format!("Text entered: \"{}\" and \"{}\"", data0, data1),
        // );

        ui.separator();
      });

      // let tab = ui.tabbar(hash!(), vec2(200., 20.), &["object", "materials", "world"]);
      // match tab {
      //   0 => profiler_window(ui, &mut state),
      //   1 => ui.label(
      //     None,
      //     &format!(
      //       "scene allocated memory: {:.1} kb",
      //       (telemetry::scene_allocated_memory() as f32) / 1000.0
      //     ),
      //   ),
      //   _ => unreachable!(),
      // }

      ui.tree_node(hash!(), "sliders", |ui| {
        // ui.slider(hash!(), "[-10 .. 10]", -10f32..10f32, &mut number0);
        // ui.slider(hash!(), "[0 .. 100]", 0f32..100f32, &mut number1);
      });
      ui.tree_node(hash!(), "editbox 1", |ui| {
        ui.label(None, "This is editbox!");
        // ui.editbox(hash!(), vec2(285., 165.), &mut text0);
      });
      ui.tree_node(hash!(), "editbox 2", |ui| {
        ui.label(None, "This is editbox!");
        // ui.editbox(hash!(), vec2(285., 165.), &mut text1);
      });
    });
}

#[macroquad::main("New scene* • B2D")]
async fn main() {
  let mut model = SceneModel::create();
  let scene = &mut model.scene;

  let mut screen_rect = Rect {
    x: 0.,
    y: 0.,
    w: screen_width(),
    h: screen_height(),
  };
  set_camera(scene.main_camera.set_resolution(screen_rect));

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

  let skin = create_gui_skin();

  loop {
    scene.update();

    clear_background(RED);

    root_ui().push_skin(&skin);
    gui();
    root_ui().pop_skin();

    screen_rect.w = screen_width();
    screen_rect.h = screen_height();

    // cam2d.offset = -scene.main_camera.transform.location;
    // cam2d.rotation = scene.main_camera.transform.rotation;
    // cam2d.zoom = vec2(1. / screen_rect.w * 2., -1. / screen_rect.h * 2.) * scene.main_camera.zoom;
    // set_camera(scene.main_camera.set_resolution(screen_rect));

    draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
    draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
    draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

    draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

    set_default_camera();
    draw_text(
      &format!(
        "x {:.2}, y{:.2}",
        scene.main_camera.cam2d().target.x,
        scene.main_camera.cam2d().target.y
      ),
      20.0,
      20.0,
      30.0,
      DARKGRAY,
    );

    profiler::profiler(Default::default());

    next_frame().await
  }
}
