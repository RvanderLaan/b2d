mod editor;

// based on blenkernel/intern/context.c
struct bContext {
  // TODO: Window context


  // Data context
  scene: Scene,
}

trait Operator {
  // Start the 
  pub fn invoke(&self, &mut c: bContext) -> void;
  pub fn apply(&self, &mut c: bContext) -> void;
  pub fn cancel(&self, &mut c: bContext) -> void;
}

struct SelectObject {
  id: u32,
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
  fn invoke(&self) {}
  fn apply(&self) {
    
  }
}