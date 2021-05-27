use std::{hash::{self, Hash}, ops::Index, rc::Rc};

use crate::editor;
use editor::editor::*;

use crate::scene;
use scene::scene_model::*;

// An operator that modifies a state
// follows the Command pattern
// TODO: Invoke/cancel + perform/undo feels like 2 commands in 1
// Inspiration: 
trait Operator {
  // Start the operation (e.g. enable a widget)
  fn invoke(&mut self, c: &mut Editor);
  
  // Cancel whatever the invoke call did
  fn cancel(&mut self, c: &mut Editor);

  // Perform the operation
  fn perform(&mut self, c: &mut Editor);
  
  // Undo the effects of the operation perform call
  fn undo(&mut self, c: &mut Editor);
}

pub struct SelectObject {
  object: Rc<Shape>,
  // Whether to replace, or to expand the current selection. E.g. when holding ctrl, expand it
  expand_selection: bool,

  prev_active_object: Option::<Rc<Shape>>,
}

// NOTE TO SELF: DON'T WORRY TOO MUCH ABOUT WHERE TO STORE THINGS LIKE SELECTED_STATE
// JUST MAKE SOMETHING COOL
// but for later:
// Selection state should only be modified by operators, not accessible directly

// for a "move" operator, how to keep it "active" until cancelled/ended?
// - put "status" on operator, check latest operator status every frame?
// - explicit "end" callbacks?

impl Operator for SelectObject {

  // fn new() {}
  fn invoke(&mut self, editor: &mut Editor) {}
  
  fn cancel(&mut self, e: &mut Editor) {
    todo!()
  }

  fn perform(&mut self, e: &mut Editor) {
    self.prev_active_object = e.scene_context.active_object.clone();

    e.scene_context.active_object = Some(Rc::clone(&self.object));
    e.scene_context.selected_objects.push(Rc::clone(&self.object));
  }

  fn undo(&mut self, e: &mut Editor) {
    // TODO: is cloning good here? Feel like it's better to hand over ownership
    // unclear whether this copies the whole shape, or just the reference
    e.scene_context.active_object = self.prev_active_object.clone();

    // ???
    if let Some(pos) = e.scene_context.selected_objects
      .iter()
      .position(|x| x.as_ref() as *const _ == self.object.as_ref() as *const _)
    {
      e.scene_context.selected_objects.remove(pos);
    }
  }
}
