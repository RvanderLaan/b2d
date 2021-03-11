// "Inspired" by https://github.com/codec-abc/Yew-WebRTC-Chat/blob/master/src/chat/chat_model.rs
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::*;

use crate::scene::web_rtc_manager::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::str;


use wasm_bindgen::prelude::*;

// Encoding/decoding messages with Serde: https://serde.rs/
// TODO: look into the how's and why's
// use base64;
// #[allow(unused_imports)]
// use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct Vec2D {
  x: f32,
  y: f32,
}


#[derive(Clone, Debug)]
pub struct Transform {
    location: Vec2D,
    rotation: f32,
    scale: Vec2D,
}

#[derive(Clone, Debug)]
pub struct Shape {
  id: i32,
  transform: Transform,
//   name: str,
  points: Vec<Vec2D>,
//   edges/faces?
  // layer ?
}

#[derive(Clone, Debug)]
pub struct Camera {
  transform: Transform,
  zoom: f32,
}

#[derive(Clone, Debug)]
pub struct Scene {
  shapes: Vec<Shape>,
  main_camera: Camera,
}

pub struct ActionHandler {
  // undoStack Vec<Action>
  // TODO: take in actions, store undo/redo
  // Also accept middleware: push actions over webRTC (or just send the whole scene for now)
  // maybe call it Operations instead of actions?
}

impl ActionHandler {
  fn runAction() {

  }
  fn undo() {

  }
  fn redo() {

  }
}
 
// the "main" manager
pub struct SceneModel {
  web_rtc_manager: Rc<RefCell<WebRTCManager>>,
  scene: Scene,
}

// All action types (at least, those sent over WebRTC)
#[derive(Clone, Debug)]
pub enum Msg {
    StartAsServer,
    ConnectToServer,
    UpdateWebRTCState(State),
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
        let web_rtc_manager = WebRTCManager::create_default(link.clone());

        let rc = Rc::new(RefCell::new(web_rtc_manager));

        let model = SceneModel {
            web_rtc_manager: rc.clone(),
            scene: Scene{
                main_camera: Camera{}
            },
            link,
            value: "".into(),
            chat_value: "".into(),
            node_ref: NodeRef::default(),
        };

        model
  }

  fn update(&mut self, msg: Self::Message) {
    match msg {
      Msg::StartAsServer => {
        self.web_rtc_manager
            .borrow_mut()
            .set_state(State::Server(ConnectionState::new()));
        WebRTCManager::start_web_rtc(self.web_rtc_manager.clone());
        let re_render = true;
        return re_render;
    }

    Msg::ConnectToServer => {
        self.web_rtc_manager
            .borrow_mut()
            .set_state(State::Client(ConnectionState::new()));
        WebRTCManager::start_web_rtc(self.web_rtc_manager.clone());
        let re_render = true;
        return re_render;
    }

    Msg::UpdateWebRTCState(web_rtc_state) => {
      self.value = "".into();
      let debug = ChatModel::get_debug_state_string(&web_rtc_state);
      console::log_1(&debug.into());

      // let debug = self.get_serialized_offer_and_candidates();
      // let hash = hmac_sha256::Hash::hash(debug.as_bytes());
      // let hash_as_string = hex::encode(hash);
      // console::log_1(&hash_as_string.into());

      let re_render = true;
      return re_render;
  }

  Msg::ResetWebRTC => {
      let web_rtc_manager = WebRTCManager::create_default(self.link.clone());
      let rc = Rc::new(RefCell::new(web_rtc_manager));
      self.web_rtc_manager = rc;
      self.messages = vec![];
      self.chat_value = "".into();
      self.value = "".into();

      let re_render = true;
      return re_render;
  }

  Msg::UpdateInputValue(val) => {
      self.value = val;
      let re_render = true;
      return re_render;
  }

  Msg::ValidateOffer => {
    let state = self.web_rtc_manager.borrow().get_state();

    match state {
        State::Server(_connection_state) => {
            let result = WebRTCManager::validate_answer(
                self.web_rtc_manager.clone(),
                &self.value,
            );

            if result.is_err() {
                web_sys::Window::alert_with_message(
                    &web_sys::window().unwrap(),
                    &format!(
                        "Cannot use answer. Failure reason: {:?}",
                        result.err().unwrap()
                    ),
                )
                .expect("alert should work");
            }
        }
        _ => {
            let result = WebRTCManager::validate_offer(
                self.web_rtc_manager.clone(),
                &self.value,
            );

            if result.is_err() {
                web_sys::Window::alert_with_message(
                    &web_sys::window().unwrap(),
                    &format!(
                        "Cannot use offer. Failure reason: {:?}",
                        result.err().unwrap()
                    ),
                )
                .expect("alert should work");
            }
        }
    };

    let re_render = true;
    return re_render;
}

Msg::NewMessage(message) => {
    self.messages.push(message);
    self.scroll_top();
    let re_render = true;
    return re_render;
}

Msg::Send => {
    let content = self.chat_value.clone();
    let my_message = Message::new(content.clone(), MessageSender::Me);
    self.messages.push(my_message);
    self.web_rtc_manager.borrow().send_message(content);
    self.chat_value = "".into();
    self.scroll_top();
    let re_render = true;
    return re_render;
}

Msg::Disconnect => {
    let web_rtc_manager = WebRTCManager::create_default(self.link.clone());
    let rc = Rc::new(RefCell::new(web_rtc_manager));
    self.web_rtc_manager = rc;
    self.messages = vec![];
    self.chat_value = "".into();
    self.value = "".into();
    let re_render = true;
    return re_render;
}
    }
  }




}


// TODO:
// - Step 0: Set up connection. Looks fairly simple in YewWebRTC chat repo
// - Step 1: Send entire serialized scene to peers on any change
// - Step 2: Only send actions, modify own state (look into CFRDT)


// Copied here onwards
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConnectionString {
    pub ice_candidates: Vec<IceCandidate>,
    pub offer: String, // TODO : convert as JsValue using Json.Parse
}
