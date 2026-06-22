use yew::prelude::*;
use gloo_net::websocket::futures::WebSocket;
use futures_util::{SinkExt, StreamExt};
use wasm_bindgen_futures::spawn_local;
use serde_json::json;

#[hook]
pub fn use_collab_websocket(notepad_id: &str) {
    let notepad_id = notepad_id.to_string();
    use_effect_with(notepad_id, move |nid| {
        let nid = nid.clone();
        let window = web_sys::window().unwrap();
        let protocol = if window.location().protocol().unwrap() == "https:" { "wss:" } else { "ws:" };
        let host = window.location().host().unwrap();
        let ws_url = format!("{}//{}/ws", protocol, host);
        
        if let Ok(ws) = WebSocket::open(&ws_url) {
            let (mut write, mut read) = ws.split();
            let user_id = format!("user_{}", chrono::Utc::now().timestamp_millis());
            let init_msg = json!({
                "type": "sync_request",
                "userId": user_id,
                "notepadId": nid
            }).to_string();
            
            spawn_local(async move {
                let _ = write.send(gloo_net::websocket::Message::Text(init_msg)).await;
            });
            
            spawn_local(async move {
                while let Some(Ok(msg)) = read.next().await {
                    if let gloo_net::websocket::Message::Text(_) = msg {}
                }
            });
        }
        
        || ()
    });
}
