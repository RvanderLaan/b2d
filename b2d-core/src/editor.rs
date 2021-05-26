use std::collections::HashSet;

#[derive(Default)]
// TODO: persist UI state https://github.com/emilk/egui/blob/master/egui_demo_lib/src/wrap_app.rs#L203
// #[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
// #[cfg_attr(feature = "persistence", serde(default))]
struct properties_panel {
  selected_tab: String,

}

// Object mode 
struct object_mode {
  selected_objects: HashSet::new(),
}

enum ObjectEditModeGeometryType {
  VERTEX,
  EDGE,
  FACE,
}

// Every object should have its own edit mode
struct object_edit_mode {
  geometry_type: ObjectEditModeGeometryType,
  selected_geometry: HashSet::new(),
}

impl object_edit_mode {
  pub fn set_geometry_type() {
    // TODO: This one should be undoable. How are we doing operations again?
    // when changing mode, change the selected geometry into the new type (e.g. two edges become one edge)
  }
}

type ID = u64;

enum MeshEditModeGeometryType {
  VERTEX,
  EDGE,
  FACE,
}

struct curve_edit_mode {
  geometry_type: ObjectEditModeGeometryType,
  selected_point: ID,
}

struct sdf_edit_mode {
  // ...
}

enum EditorMode {
  Object,
  EditMesh,
  EditCurve,
  // enum in [‘EDIT_MESH’, ‘EDIT_CURVE’, ‘EDIT_SURFACE’, ‘EDIT_TEXT’, ‘EDIT_ARMATURE’, ‘EDIT_METABALL’, ‘EDIT_LATTICE’, ‘POSE’,
  // ‘SCULPT’, ‘PAINT_WEIGHT’, ‘PAINT_VERTEX’, ‘PAINT_TEXTURE’, ‘PARTICLE’, ‘OBJECT’, ‘PAINT_GPENCIL’, ‘EDIT_GPENCIL’, ‘SCULPT_GPENCIL’,
  // ‘WEIGHT_GPENCIL’, ‘VERTEX_GPENCIL’], default ‘EDIT_MESH’, (readonly)
}

struct context {
}

struct editor {
  object_mode: object_mode,
  mode: EditorMode,
  // TODO: expose context singleton?
}
