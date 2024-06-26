use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

use rocket::{
    futures::{stream::SplitSink, SinkExt, StreamExt},
    tokio::sync::Mutex,
    State,
};
use rocket_ws::{stream::DuplexStream, Channel, Message, WebSocket};

static USER_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(Default)]
pub struct ChatRoom {
    connections: Mutex<HashMap<usize, SplitSink<DuplexStream, Message>>>,
}

impl ChatRoom {
    pub async fn add(&self, id: usize, sink: SplitSink<DuplexStream, Message>) {
        let mut connections = self.connections.lock().await;
        connections.insert(id, sink);
    }

    pub async fn remove(&self, id: usize) {
        let mut connections = self.connections.lock().await;
        connections.remove(&id);
    }

    pub async fn broadcast_message(&self, message: Message) {
        let mut connections = self.connections.lock().await;

        for (_id, sink) in connections.iter_mut() {
            let _ = sink.send(message.clone()).await;
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

            while let Some(message) = ws_stream.next().await {
                // Send message
                state.broadcast_message(message?).await;
            }

            // Remove connection
            state.remove(user_id).await;

            Ok(())
        })
    })
}
