use chrono::Utc;
use common::chat::chat::{ChatMessage, WebSocketMessage, WebSocketMessageType};

use serde_json::json;
use yew::prelude::*;
use yew_hooks::use_websocket;

use super::{message_list::MessageList, send_dialog::SendDialog, users_list::UsersList};

#[function_component]
pub fn App() -> Html {
    let messages_handle = use_state(Vec::default);
    let messages = (*messages_handle).clone();

    let users_handle = use_state(Vec::default);
    let users = (*users_handle).clone();

    let username_handle = use_state(String::default);
    let username = (*username_handle).clone();

    let ws = use_websocket("ws://127.0.0.1:8000".to_string());

    let mut cloned_messages = messages.clone();

    use_effect_with(ws.message.clone(), move |ws_message| {
        if let Some(ws_msg) = &**ws_message {
            let websocket_message = serde_json::from_str::<WebSocketMessage>(&ws_msg).unwrap();

            match websocket_message.message_type {
                WebSocketMessageType::NewMessage => {
                    let msg = websocket_message.message.expect("Missing message payload");
                    cloned_messages.push(msg);
                    messages_handle.set(cloned_messages);
                }
                WebSocketMessageType::UsersList => {
                    let users = websocket_message.users.expect("Missing users payload");
                    users_handle.set(users);
                }
                WebSocketMessageType::UpdateUsername => {
                    let username = websocket_message
                        .username
                        .expect("Missing username payload");
                    username_handle.set(username);
                }
            }
        }
    });

    let cloned_username = username.clone();
    let cloned_ws: yew_hooks::UseWebSocketHandle = ws.clone();
    let send_message_callback = Callback::from(move |msg: String| {
        let websocket_message = WebSocketMessage {
            message_type: WebSocketMessageType::NewMessage,
            message: Some(ChatMessage {
                message: msg,
                author: cloned_username.clone(),
                created_at: Utc::now().naive_utc(),
            }),
            username: None,
            users: None,
        };
        cloned_ws.send(json!(websocket_message).to_string());
    });

    let cloned_ws: yew_hooks::UseWebSocketHandle = ws.clone();
    let change_username_callback = Callback::from(move |username: String| {
        let websocket_message = WebSocketMessage {
            message_type: WebSocketMessageType::UpdateUsername,
            message: None,
            username: Some(username),
            users: None,
        };
        cloned_ws.send(json!(websocket_message).to_string());
    });

    html! {
        <div class="container">
            <div class="row">
                <div class="col-sm-3">
                    <UsersList users={users}/>
                </div>
                <div class="col-sm-9">
                    <MessageList messages={messages}/>
                </div>
            </div>
            <div class="row mt-5">
            if username.len() > 0{
                <SendDialog
                    send_message_callback={send_message_callback}
                    change_username_callback={change_username_callback}
                    username={username}/>
            }
            </div>
        </div>
    }
}
