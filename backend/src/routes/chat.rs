use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

use common::chat::chat::{ChatMessage, WebSocketMessage, WebSocketMessageType};
use rocket::{
    futures::{stream::SplitSink, SinkExt, StreamExt},
    tokio::sync::Mutex,
    State,
};
use rocket_ws::{stream::DuplexStream, Channel, Message, WebSocket};
use serde_json::json;

static USER_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(Default)]
pub struct ChatRoom {
    connections: Mutex<HashMap<usize, ChatRoomConnection>>,
}

pub struct ChatRoomConnection {
    username: String,
    sink: SplitSink<DuplexStream, Message>,
}

impl ChatRoom {
    pub async fn add(&self, id: usize, sink: SplitSink<DuplexStream, Message>) {
        let mut connections = self.connections.lock().await;

        let connection = ChatRoomConnection {
            username: format!("User #{}", id),
            sink,
        };

        connections.insert(id, connection);
    }

    pub async fn remove(&self, id: usize) {
        let mut connections = self.connections.lock().await;
        connections.remove(&id);
    }

    pub async fn broadcast_message(&self, message: ChatMessage) {
        let mut connections = self.connections.lock().await;

        let websocket_message = WebSocketMessage {
            message_type: WebSocketMessageType::NewMessage,
            message: Some(message),
            users: None,
            username: None,
        };

        for (_id, connection) in connections.iter_mut() {
            let _ = connection
                .sink
                .send(Message::Text(json!(websocket_message).to_string()))
                .await;
        }
    }

    pub async fn broadcast_users(&self) {
        let mut connections = self.connections.lock().await;
        let mut users = vec![];

        for (_id, connection) in connections.iter() {
            users.push(connection.username.clone());
        }

        let websocket_message = WebSocketMessage {
            message_type: WebSocketMessageType::UsersList,
            message: None,
            username: None,
            users: Some(users),
        };

        for (_id, connection) in connections.iter_mut() {
            let _ = connection
                .sink
                .send(Message::Text(json!(websocket_message).to_string()))
                .await;
        }
    }

    pub async fn send_username(&self, id: usize) {
        let mut connections = self.connections.lock().await;

        if let Some(connection) = connections.get_mut(&id) {
            let websocket_message = WebSocketMessage {
                message_type: WebSocketMessageType::UpdateUsername,
                username: Some(connection.username.clone()),
                users: None,
                message: None,
            };

            let _ = connection
                .sink
                .send(Message::Text(json!(websocket_message).to_string()))
                .await;
        }
    }

    pub async fn update_username(&self, new_username: String, id: usize) {
        let mut connections = self.connections.lock().await;
        if let Some(connection) = connections.get_mut(&id) {
            connection.username = new_username;
        }
    }
}

pub async fn handle_incoming_message(
    message_contents: Message,
    state: &State<ChatRoom>,
    connection_id: usize,
) {
    match message_contents {
        Message::Text(json) => {
            if let Ok(websocket_message) = serde_json::from_str::<WebSocketMessage>(&json) {
                match websocket_message.message_type {
                    WebSocketMessageType::NewMessage => {
                        if let Some(ws_msg) = websocket_message.message {
                            state.broadcast_message(ws_msg).await;
                        }
                    }
                    WebSocketMessageType::UpdateUsername => {
                        if let Some(ws_username) = websocket_message.username {
                            state.update_username(ws_username, connection_id).await;
                            state.broadcast_users().await;
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {
            // Unsupported
        }
    }
}

#[rocket::get("/")]
pub fn chat<'r>(ws: WebSocket, state: &'r State<ChatRoom>) -> Channel<'r> {
    ws.channel(move |stream| {
        Box::pin(async move {
            let user_id = USER_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
            let (ws_sink, mut ws_stream) = stream.split();
            // Add connection
            state.add(user_id, ws_sink).await;
            state.broadcast_users().await;
            state.send_username(user_id).await;

            while let Some(message) = ws_stream.next().await {
                if let Ok(message_contents) = message {
                    handle_incoming_message(message_contents, state, user_id).await;
                }
            }

            // Remove connection
            state.remove(user_id).await;
            state.broadcast_users().await;

            Ok(())
        })
    })
}
