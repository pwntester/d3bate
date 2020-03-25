use std::collections::{HashMap, HashSet};
use std::option::Option;

use actix::{Actor, StreamHandler};
use actix::prelude::*;

use actix_web_actions::ws;


#[derive(Message)]
#[rtype(result = "()")]
pub struct ChatMessage {
    pub text: String,
    // This should be the id in the database (not the hashmap)
    pub debate: usize,
}

#[derive(Message)]
#[rtype(usize)]
struct UserJoin {
    pub addr: Recipeient<Message>
}

#[derive(Message)]
#[rtype(usize)]
struct UserRequestDebateJoin {
    // id should correspond to the id of a column in the 'training_session' table
    pub debate: usize,
}

/// The face struct represents data from the client about a specific face. More than one person might
/// use a single device. All the fields are instances of the `Option` enum because the client will
/// only send data to the server if its value has changed.
/// The client is using expo's [face detection API](https://docs.expo.io/versions/latest/sdk/facedetector/)
/// which is where the fields of the struct come from.
struct Face {
    pub face_id: i32,
    pub bounds_origin: Option<(i32, i32)>,
    pub bounds_size: Option<(i32, i32)>,
    pub roll_angle: f64,
    pub yaw_angle: f64,
    pub smiling_probability: Option<f64>,
    pub left_ear_position: Option<(i32, i32)>,
    pub right_ear_position: Option<(i32, i32)>,
    pub left_eye_position: Option<(i32, i32)>,
    pub left_eye_open_prob: Option<f64>,
    pub right_eye_position: Option<(i32, i32)>,
    pub right_eye_open_prob: Option<f64>,
    pub left_cheek_position: Option<(i32, i32)>,
    pub right_cheek_position: Option<(i32, i32)>,
    pub mouth_position: Option<(i32, i32)>,
    pub left_mouth_position: Option<(i32, i32)>,
    pub right_mouth_position: Option<(i32, i32)>,
    pub nose_base_position: Option<(i32, i32)>,
}

#[derive(Message)]
#[rtype(usize)]
pub struct VideoSnapshot {
    pub people_count: u8,
    pub faces: Vec<Face>,
}

#[derive(Message)]
#[type (usize)]
pub struct AudioSnapshot {}


/// Represents a debate
/// The `session_id` field should correspond to a database entry
pub struct Debate {
    pub users: HashSet<usize>,
    pub session_id: usize,
}

pub struct PerformantWebsockets {
    sessions: HashMap<usize, Recipient<Message>>,
    debates: HashMap<usize, Debate>,
}


impl Actor for PerformantWebsockets {
    type Context = Context<Self>;
}

pub struct WsDebateSession {
    id: usize,
    hb: std::time::Instant,
    debate: usize,
    authenticated: bool,
    name: String,
    username: String,
    addr: Addr<PerformantWebsockets>,
    jwt: String,
}

impl Actor for WsDebateSession {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        let addr = ctx.address();
    }
}