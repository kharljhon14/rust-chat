use yew::prelude::*;
use yew_hooks::use_websocket;

#[function_component]
pub fn App() -> Html {
    let messages_handle = use_state(Vec::default);
    let messages = (*messages_handle).clone();

    let ws = use_websocket("ws://127.0.0.1:8000".to_string());
    let mut cloned_messages = messages.clone();

    use_effect_with(ws.message, move |ws_message| {
        if let Some(ws_msg) = &**ws_message {
            cloned_messages.push(ws_msg.clone());
            messages_handle.set(cloned_messages);
        }
    });

    html! {
        <ul>
            {
                messages.iter().map(|message| html!{
                    <li>{message}</li>
                }).collect::<Html>()
            }
        </ul>
    }
}