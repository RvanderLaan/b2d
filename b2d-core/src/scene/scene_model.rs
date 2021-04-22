// "Inspired" by https://github.com/codec-abc/Yew-WebRTC-Chat/blob/master/src/chat/chat_model.rs
use web_sys::*;

// use crate::scene::web_rtc_manager::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::str;

// Encoding/decoding messages with Serde: https://serde.rs/
// TODO: look into the how's and why's
// use base64;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default)]
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

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            location: Vec2D::default(),
            rotation: 0.,
            scale: Vec2D{x: 1., y: 1.},
        }
   }
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

#[derive(Clone, Debug, Default)]
pub struct Camera {
  transform: Transform,
  zoom: f32,
}

#[derive(Clone, Debug, Default)]
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
#[derive(Clone, Debug)]
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
            scene: Scene{
                main_camera: Camera::default(),
                shapes: vec!(),
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
