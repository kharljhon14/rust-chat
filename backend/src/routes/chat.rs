use rocket::futures::{SinkExt, StreamExt};
use rocket_ws::{Channel, WebSocket};

#[rocket::get("/")]
pub fn chat(ws: WebSocket) -> Channel<'static> {
    ws.channel(move |mut stream| {
        Box::pin(async move {
            while let Some(message) = stream.next().await {
                let _ = stream.send(message?).await;
            }

            Ok(())
        })
    })
}
