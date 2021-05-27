use macroquad::{models::*, prelude::*};

// use crate::scene::web_rtc_manager::*;

use std::str;
use std::{
  hash::{self, Hash},
  rc::Rc,
};

// Encoding/decoding messages with Serde: https://serde.rs/
// TODO: look into the how's and why's
// use base64;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct Transform {
  pub location: Vec2,
  pub rotation: f32,
  pub scale: Vec2,
}

impl Default for Transform {
  fn default() -> Transform {
    Transform {
      location: Vec2::default(),
      rotation: 0.,
      scale: vec2(1., 1.),
    }
  }
}

#[derive(Clone, Debug)]
pub struct Shape {
  id: u64,
  transform: Transform,
  pub name: String,
  points: Vec<Vec2>,
  //   edges/faces?
  // layer ?
}

impl Shape {
  pub fn get_mesh(&self) -> Mesh {
    // TODO: should cache this
    Mesh {
      vertices: self
        .points
        .iter()
        .map(|p| macroquad::models::Vertex {
          position: [p.x, p.y, 0.].into(),
          uv: [0., 0.].into(),
          color: [255, 255, 255, 255].into(),
        })
        .collect(),
      indices: (0..(self.points.len()))
        .map(|x| x as u16)
        .collect::<Vec<u16>>(),
      texture: None,
    }
  }
}

pub fn create_default_square() -> Shape {
  return Shape {
    id: 0, // TODO:: RANDOM HASH
    transform: Default::default(),
    name: "Square".to_string(),
    points: vec![
      vec2(-0.5, 0.5),
      vec2(0.5, 0.5),
      vec2(0.5, -0.5),
      vec2(-0.5, -0.5),
    ],
  };
}

#[derive(Clone)]
pub struct Camera {
  _camera: Camera2D,
  pub transform: Transform,
  pub zoom: f32,
  cursor_grab: bool,
  prev_mouse_pos: Vec2,
}

impl Default for Camera {
  fn default() -> Camera {
    Camera {
      _camera: Camera2D::default(),
      transform: Transform::default(),
      zoom: 1.,
      cursor_grab: true,
      prev_mouse_pos: Vec2::default(),
    }
  }
}

impl Camera {
  pub fn set_resolution(&mut self, rect: Rect) -> &Camera2D {
    // self._camera.
    self._camera = Camera2D::from_display_rect(rect);
    self._camera.target = Vec2::default();
    return &self._camera;
  }

  pub fn cam2d(&self) -> &Camera2D {
    return &self._camera;
  }
}

trait Updatable {
  fn update(&mut self);
}

impl Updatable for Camera {
  // would be nice to have initialize() for cursor grab

  fn update(&mut self) {
    // Based on
    // https://github.com/not-fl3/macroquad/blob/master/examples/camera_transformations.rs
    let mut local_translation = Vec2::default();
    if is_key_down(KeyCode::Up) {
      local_translation.y -= 10.;
    }
    if is_key_down(KeyCode::Down) {
      local_translation.y += 10.;
    }
    if is_key_down(KeyCode::Left) {
      local_translation.x -= 10.;
    }
    if is_key_down(KeyCode::Right) {
      local_translation.x += 10.;
    }
    // TODO: Rotation
    self.transform.location += local_translation / self.zoom;

    let mouse_pos: Vec2 = mouse_position().into();

    // Move with Shift + middle mouse button
    if is_key_down(KeyCode::LeftShift) && is_mouse_button_down(MouseButton::Middle) {
      let p1 = self._camera.screen_to_world(self.prev_mouse_pos);
      let p2 = self._camera.screen_to_world(mouse_pos);
      let delta = p2 - p1;

      self.transform.location -= delta;
    }

    if is_key_pressed(KeyCode::Escape) {
      self.cursor_grab = !self.cursor_grab;
      set_cursor_grab(self.cursor_grab);
    }

    match mouse_wheel() {
      (_x, y) if y != 0.0 => {
        // Normalize mouse wheel values is browser (chromium: 53, firefox: 3)
        #[cfg(target_arch = "wasm32")]
        let y = if y < 0.0 {
          -1.0
        } else if y > 0.0 {
          1.0
        } else {
          0.0
        };
        // if is_key_down(KeyCode::LeftControl) {
        // TODO: Zoom to cursor
        self.zoom *= 1.1f32.powf(y);
        // } else {
        // self.transform.rotation += 10.0 * y;
        // self.transform.rotation = match self.transform.rotation {
        //     angle if angle >= 360.0 => angle - 360.0,
        //     angle if angle < 0.0 => angle + 360.0,
        //     angle => angle,
        // }
        // }
      }
      _ => (),
    }

    // Deal with window resizing
    let rect = Rect {
      x: 0.,
      y: 0.,
      w: screen_width(),
      h: screen_height(),
    };
    self._camera = Camera2D::from_display_rect(rect);
    self._camera.target = self.transform.location;
    self._camera.rotation = self.transform.rotation;
    self._camera.zoom = vec2(1. / rect.w * 2., -1. / rect.h * 2.) * self.zoom;

    set_camera(&self._camera);

    self.prev_mouse_pos = mouse_pos
  }
}

#[derive(Clone, Default)]
pub struct Scene {
  pub shapes: Vec<Shape>,
  pub main_camera: Camera,
}

impl Scene {
  pub fn update(&mut self) {
    self.main_camera.update();
  }
}

pub struct ActionHandler {
  // undoStack Vec<Action>
// TODO: take in actions, store undo/redo
// Also accept middleware: push actions over webRTC (or just send the whole scene for now)
// maybe call it Operations instead of actions?
}

impl ActionHandler {
  fn runAction() {}
  fn undo() {}
  fn redo() {}
}

// the "main" manager
pub struct SceneModel {
  //   web_rtc_manager: Rc<RefCell<WebRTCManager>>,
  pub scene: Scene, // shouldn't be public, but, yeah.

                    //   value: String, // TODO: what is "value"?
}

//
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MessageSender {
  Me,
  Other,
}

#[derive(Clone, Debug)]
pub struct Message {
  sender: MessageSender,
  content: String,
}

impl Message {
  pub fn new(content: String, sender: MessageSender) -> Message {
    Message {
      content: content,
      sender: sender,
    }
  }
}

// All action types (at least, those sent over WebRTC)
#[derive(Clone)]
pub enum Msg {
  StartAsServer,
  ConnectToServer,
  // UpdateWebRTCState(State),
  Disconnect,
  Send,
  UpdateScene(Scene), // TODO: temporary msg
  // UpdateInputValue(String),
  // UpdateInputChatValue(String),
  // OnKeyUp(KeyboardEvent),
  // CopyToClipboard,
  ValidateOffer,
  ResetWebRTC,
}

impl SceneModel {
  pub fn create() -> Self {
    // let mut web_rtc_manager = WebRTCManager::create_default();

    // let rc = Rc::new(RefCell::new(web_rtc_manager));

    let model = SceneModel {
      // web_rtc_manager: rc.clone(),
      scene: Scene {
        main_camera: Camera::default(),
        shapes: vec![],
      },
      // value: "".into(),
      // chat_value: "".into(),
      // node_ref: NodeRef::default(),
    };

    // let mut model_callback = |msg: Msg| {
    //     model.update(msg);
    // };

    // web_rtc_manager.set_model_callback(model_callback);

    model
  }

  pub fn get_scene(self) -> Rc<Scene> {
    Rc::new(self.scene)
  }

  pub fn update(&mut self) {
    self.scene.update();
  }

  //   pub fn update(&mut self, msg: Msg) -> bool {
  //     match msg {
  //       Msg::StartAsServer => {
  //         self.web_rtc_manager
  //             .borrow_mut()
  //             .set_state(State::Server(ConnectionState::new()));
  //         WebRTCManager::start_web_rtc(self.web_rtc_manager.clone());
  //         let re_render = true;
  //         return re_render;
  //     }

  //     Msg::ConnectToServer => {
  //         self.web_rtc_manager
  //             .borrow_mut()
  //             .set_state(State::Client(ConnectionState::new()));
  //         WebRTCManager::start_web_rtc(self.web_rtc_manager.clone());
  //         let re_render = true;
  //         return re_render;
  //     }

  //     Msg::UpdateWebRTCState(web_rtc_state) => {
  //       self.value = "".into();
  //       let debug = SceneModel::get_debug_state_string(&web_rtc_state);
  //       console::log_1(&debug.into());

  //       // let debug = self.get_serialized_offer_and_candidates();
  //       // let hash = hmac_sha256::Hash::hash(debug.as_bytes());
  //       // let hash_as_string = hex::encode(hash);
  //       // console::log_1(&hash_as_string.into());

  //       let re_render = true;
  //       return re_render;
  //   }

  //   Msg::ResetWebRTC => {
  //       let web_rtc_manager = WebRTCManager::create_default();
  //       let rc = Rc::new(RefCell::new(web_rtc_manager));
  //       self.web_rtc_manager = rc;
  //       self.scene = Scene::default();
  //       self.value = "".into();

  //       let re_render = true;
  //       return re_render;
  //   }

  // //   Msg::UpdateInputValue(val) => {
  // //       self.value = val;
  // //       let re_render = true;
  // //       return re_render;
  // //   }

  //   Msg::ValidateOffer => {
  //     let state = self.web_rtc_manager.borrow().get_state();

  //     match state {
  //         State::Server(_connection_state) => {
  //             let result = WebRTCManager::validate_answer(
  //                 self.web_rtc_manager.clone(),
  //                 &self.value,
  //             );

  //             if result.is_err() {
  //                 web_sys::Window::alert_with_message(
  //                     &web_sys::window().unwrap(),
  //                     &format!(
  //                         "Cannot use answer. Failure reason: {:?}",
  //                         result.err().unwrap()
  //                     ),
  //                 )
  //                 .expect("alert should work");
  //             }
  //         }
  //         _ => {
  //             let result = WebRTCManager::validate_offer(
  //                 self.web_rtc_manager.clone(),
  //                 &self.value,
  //             );

  //             if result.is_err() {
  //                 web_sys::Window::alert_with_message(
  //                     &web_sys::window().unwrap(),
  //                     &format!(
  //                         "Cannot use offer. Failure reason: {:?}",
  //                         result.err().unwrap()
  //                     ),
  //                 )
  //                 .expect("alert should work");
  //             }
  //         }
  //     };

  //     let re_render = true;
  //     return re_render;
  // }

  // // Msg::Send => {
  // //     let content = self.chat_value.clone();
  // //     let my_message = Message::new(content.clone(), MessageSender::Me);
  // //     self.messages.push(my_message);
  // //     self.web_rtc_manager.borrow().send_message(content);
  // //     self.chat_value = "".into();
  // //     self.scroll_top();
  // //     let re_render = true;
  // //     return re_render;
  // // }

  // Msg::Disconnect => {
  //     // TODO: Re-construct the scene?
  //     let web_rtc_manager = WebRTCManager::create_default();
  //     let rc = Rc::new(RefCell::new(web_rtc_manager));
  //     self.web_rtc_manager = rc;
  //     self.value = "".into();
  //     let re_render = true;
  //     return re_render;
  // }
  //         Msg::UpdateScene(_) => {
  //             return false
  //         }
  //         Msg::Send => {
  //             return false
  //         }
  //     }
  //   }

  //   fn get_debug_state_string(state: &State) -> String {
  //     match state {
  //         State::DefaultState => "Default State".into(),
  //         State::Server(connection_state) => format!(
  //             "{}\nice gathering: {:?}\nice connection: {:?}\ndata channel: {:?}\n",
  //             "Server",
  //             connection_state.ice_gathering_state,
  //             connection_state.ice_connection_state,
  //             connection_state.data_channel_state,
  //         ),

  //         State::Client(connection_state) => format!(
  //             "{}\nice gathering: {:?}\nice connection: {:?}\ndata channel: {:?}\n",
  //             "Client",
  //             connection_state.ice_gathering_state,
  //             connection_state.ice_connection_state,
  //             connection_state.data_channel_state,
  //         ),
  //     }
  // }
}

// TODO:
// - Step 0: Set up connection. Looks fairly simple in YewWebRTC chat repo
// - Step 1: Send entire serialized scene to peers on any change
// - Step 2: Only send actions, modify own state (look into CFRDT)

// Copied here onwards
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct ConnectionString {
//     pub ice_candidates: Vec<IceCandidate>,
//     pub offer: String, // TODO : convert as JsValue using Json.Parse
// }
