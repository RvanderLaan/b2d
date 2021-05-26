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

struct GUI {
  // add_menu: {
  //   isOpen: bool,
  //   position: Vec2,
  // }
  is_add_menu_open: bool,
  add_menu_position: Vec2,
}

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

  // let button_style = root_ui()
  //   .style_builder()
  //   .font(font_bytes)
  //   .font_size(24)
  //   .build();

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

// Added init_unfolded
fn tree_node_custom<F: FnOnce(&mut Ui)>(
  ui: &mut Ui,
  id: Id,
  label: &str,
  init_unfolded: bool,
  f: F,
) -> bool {
  if init_unfolded {
    return TreeNode::new(id, label).init_unfolded().ui(ui, f);
  }
  TreeNode::new(id, label).ui(ui, f)
}

fn gui(scene: &mut Scene) {
  let sidebar_width = 400.;

  let sidebar_id = hash!("main_sidebar");

  egui_macroquad::ui(|egui_ctx| {
    // TODO: there also is egui::SidePanel:: but it only has ::left(), not right :/
    egui::Window::new("B2D egui window")
      .anchor(egui::Align2::RIGHT_TOP, [0., 0.])
      .title_bar(false)
      .fixed_size([sidebar_width, screen_height()])
      .show(egui_ctx, |ui| {
        ui.label("Test");

        // Blender sidebar:
        // - Scene outliner
        // - Properties panel (tabs)
        //  [scene]
        //  - Render
        //  - Output
        //  - Scene
        //  - World
        //  [object]
        //  - Object
        //  - Modififiers
        //  - ...
        //  - Materials

        // Outliner: Loop over scene entities
        ui.group(|ui| {
          ui.label("<<ENTITIES>>");
          for s in scene.shapes.iter_mut() {
            // TODO: Selected state
            if ui.selectable_label(false, &s.name).changed() {
              println!("Clicked {}", s.name);
            }
          }

          ui.separator();

          if ui.button( "Add shape").clicked() {
            scene.shapes.push(create_default_square(hash!().into()));
          }
        });

        ui.group(|ui| {
          ui.add(egui::tab)
        })
      });
  });

  widgets::Window::new(
    sidebar_id,
    vec2(screen_width() - sidebar_width, 0.),
    vec2(sidebar_width, screen_height()),
  )
  .titlebar(false)
  .movable(true)
  .ui(&mut *root_ui(), |ui| {
    ui.move_window(sidebar_id, vec2(screen_width() - sidebar_width, 0.));

    // TODO: Collapse button? blender doesn't have it..

    // ui.group(hash!(), vec2(sidebar_width, 400.), |ui| {
    // Group::new(hash!("outliner"), vec2(sidebar_width, 55.)).ui(ui, |ui| {
    tree_node_custom(ui, hash!(), "Scene", true, |ui| {});
    // });

    // ui.separator();

    // ui.group(hash!(), vec2(sidebar_width, 400.), |ui| {
    let tab = ui.tabbar(hash!(), vec2(200., 20.), &["world", "object", "materials"]);
    match tab {
      0 => ui.group(hash!(), vec2(sidebar_width, 0.), |ui| {
        ui.label(None, "WORLD");
      }),
      1 => ui.group(hash!(), vec2(sidebar_width, 0.), |ui| {
        ui.label(None, "OBJECT");
      }),
      2 => ui.group(hash!(), vec2(sidebar_width, 0.), |ui| {
        ui.label(None, "MATERIALS");
      }),
      _ => unreachable!(),
    };
  });

  // if ui.button(None, "click me") {
  //   println!("hi");
  // }

  // ui.label(None, "Some other random text");
  // if ui.button(None, "other button") {
  //   println!("hi2");
  // }
  // });

  if is_key_down(KeyCode::LeftShift) && is_key_down(KeyCode::A) {
    widgets::Popup::new(hash!(), vec2(300., 400.)).ui(&mut *root_ui(), |ui| {
      ui.label(None, "ADD SHAPE");
    });
  }
}

#[macroquad::main("New scene* • B2D")]
async fn main() {
  let mut model = SceneModel::create();
  // let scene = &mut model.scene;

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

  let skin = create_gui_skin();

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
    gui(&mut model.scene);
    egui_macroquad::draw();

    profiler::profiler(Default::default());

    next_frame().await
  }
}
