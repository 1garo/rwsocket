use crate::{Client, Clients};
use futures::{FutureExt, StreamExt};
use serde::Deserialize;
use serde_json::from_str;
use tokio::sync::mpsc;
use warp::ws::{Message, WebSocket};
#[derive(Deserialize, Debug)]
pub struct TopicsRequest {
	topics: Vec<String>,
}

// theoric reference https://blog.logrocket.com/how-to-build-a-websocket-server-with-rust/
// following https://github.com/zupzup/warp-websockets-example repo
// TODO: Implement the ws mod 