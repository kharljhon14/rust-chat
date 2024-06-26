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

#[rocket::get("/")]
pub fn chat<'r>(ws: WebSocket, state: &'r State<ChatRoom>) -> Channel<'r> {
    ws.channel(move |stream| {
        Box::pin(async move {
            let user_id = USER_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
            let (ws_sink, mut ws_stream) = stream.split();

            {
                let mut connections = state.connections.lock().await;
                connections.insert(user_id, ws_sink);
            }

            while let Some(message) = ws_stream.next().await {
                {
                    let mut connections = state.connections.lock().await;

                    let msg = message?;
                    for (_id, sink) in connections.iter_mut() {
                        let _ = sink.send(msg.clone()).await;
                    }
                }
            }

            {
                let mut connections = state.connections.lock().await;
                connections.remove(&user_id);
            }

            Ok(())
        })
    })
}
