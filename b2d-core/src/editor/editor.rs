use std::collections::HashSet;
use std::{
  hash::{self, Hash},
  rc::Rc,
  str::FromStr,
};

use std::fmt;

use macroquad::{models::*, prelude::*};
use egui_macroquad::*;

use crate::scene;
use scene::scene_model::*;

// use crate::operators;
// mod operators;

type ID = u64;

// UI STUFF
//////////////////////

#[derive(Default)]
// TODO: persist UI state https://github.com/emilk/egui/blob/master/egui_demo_lib/src/wrap_app.rs#L203
// #[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
// #[cfg_attr(feature = "persistence", serde(default))]
struct PropertiesPanel {
  selected_tab: PropertiesPanelTab,
}


enum PropertiesPanelTab {
  Scene,
  World,
  Object,
  Material,
}

impl Default for PropertiesPanelTab {
  fn default() -> Self { PropertiesPanelTab::Object }
}

trait Popover {
  fn is_open(&self) -> Vec2;
  fn get_position(&self) -> Vec2;
}

struct AddObjectPopover {
  position: Vec2,
  
}

#[derive(Default)]
pub struct EditorUI {
  properties: PropertiesPanel,
  popover: Option<AddObjectPopover>,
}

impl EditorUI {
  pub fn update(&mut self, editor: &mut Editor) {
    let sidebar_width = 400.;
    let scene = &mut editor.scene;

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
            if scene.shapes.len() == 0 {
              ui.label("No shapes created yet.");
            }
            for s in scene.shapes.iter_mut() {
              // TODO: Selected state
              if ui.selectable_label(false, &s.name).changed() {
                println!("Clicked {}", s.name);
              }
            }

            ui.separator();

            if ui.button( "Add shape").clicked() {
              scene.shapes.push(create_default_square());
            }
          });

          // Tabs: https://github.com/emilk/egui/blob/master/egui_demo_lib/src/wrap_app.rs#L77
          // ui.horizontal_wrapped(|ui| {
          //   for (anchor, app) in self.apps.iter_mut() {
          //     if ui
          //         .selectable_label(self.selected_anchor == anchor, app.name())
          //         .clicked()
          //     {
          //         self.selected_anchor = anchor.to_owned();
          //         if frame.is_web() {
          //             ui.output().open_url(format!("#{}", anchor));
          //         }
          //     }
          //   }
          // })
        });
    });

   
    // if ui.button(None, "click me") {
    //   println!("hi");
    // }

    // ui.label(None, "Some other random text");
    // if ui.button(None, "other button") {
    //   println!("hi2");
    // }
    // });

    // TODO: Open AddObjectPopover with shift+A
    // if is_key_down(KeyCode::LeftShift) && is_key_down(KeyCode::A) {
    //   widgets::Popup::new(hash!(), vec2(300., 400.)).ui(&mut *root_ui(), |ui| {
    //     ui.label(None, "ADD SHAPE");
    //   });
    // }
  }
}



// Note: Use Box for owning a trait type,
// Rc else

// Object mode 
// #[derive(Default)]
// struct ObjectMode {
//   // TODO: Storing pointers versus IDs?
//   selected_objects: HashSet<ID>,
// }


// Editor state stuff 
/////////////////////////////////

enum ObjectEditModeGeometryType {
  VERTEX,
  EDGE,
  FACE,
}

// Every object should have its own edit mode
struct MeshEditMode {
  geometry_type: ObjectEditModeGeometryType,
  selected_geometry: HashSet<ID>,
}

impl MeshEditMode {
  pub fn set_geometry_type() {
    // TODO: This one should be undoable. How are we doing operations again?
    // when changing mode, change the selected geometry into the new type (e.g. two edges become one edge)
  }
}

enum MeshEditModeGeometryType {
  VERTEX,
  EDGE,
  FACE,
}

struct CurveEditMode {
  geometry_type: ObjectEditModeGeometryType,
  selected_point: ID,
}

struct SDFEditMode {
  // ...
}

#[derive(Debug, PartialEq)]
pub enum EditorMode {
  Object,
  EditMesh,
  EditCurve,
  // enum in [‘EDIT_MESH’, ‘EDIT_CURVE’, ‘EDIT_SURFACE’, ‘EDIT_TEXT’, ‘EDIT_ARMATURE’, ‘EDIT_METABALL’, ‘EDIT_LATTICE’, ‘POSE’,
  // ‘SCULPT’, ‘PAINT_WEIGHT’, ‘PAINT_VERTEX’, ‘PAINT_TEXTURE’, ‘PARTICLE’, ‘OBJECT’, ‘PAINT_GPENCIL’, ‘EDIT_GPENCIL’, ‘SCULPT_GPENCIL’,
  // ‘WEIGHT_GPENCIL’, ‘VERTEX_GPENCIL’], default ‘EDIT_MESH’, (readonly)
}

impl fmt::Display for EditorMode {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{:?}", self)
  }
}

impl Default for EditorMode {
  fn default() -> Self { EditorMode::Object }
}


#[derive(Default)]
pub struct SceneContext {
  pub active_object: Option<Rc<Shape>>,
  pub selected_objects: Vec<Rc<Shape>>,
}


#[derive(Default)]
pub struct Editor {
  // pub object_mode: ObjectMode,
  pub mode: EditorMode,
  
  // The scene contains the objects
  pub scene: Scene,
  // The context describes the editor state for the current scene,
  // including selected objects, the active camera
  pub scene_context: SceneContext,
}

impl Editor {

  // pub fn new() -> Editor {
  //   return Editor {
  //     object_mode: ObjectMode::default(),
  //     mode: EditorMode::Object,

  //     scene: Scene::default(),
  //     gui: EditorUI::default(),
  //     scene_context: SceneContext::defaul
  //   }
  // }
  
  pub fn load(&mut self) {
    // TODO: de-serialize scene
    todo!()
  }
  pub fn save_to_file(&self, path_to_file: String) {
    // TODO: serialize scene
    todo!()
  }

  // pub fn update(&mut self) {
  //   // self.gui.update( &mut self);
    
  // }
}
