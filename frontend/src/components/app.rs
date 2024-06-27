use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yew_hooks::use_websocket;

#[function_component]
pub fn App() -> Html {
    let messages_handle = use_state(Vec::default);
    let messages = (*messages_handle).clone();

    let new_message_handle = use_state(String::default);
    let new_message = (*new_message_handle).clone();

    let ws = use_websocket("ws://127.0.0.1:8000".to_string());
    let mut cloned_messages = messages.clone();

    use_effect_with(ws.message.clone(), move |ws_message| {
        if let Some(ws_msg) = &**ws_message {
            cloned_messages.push(ws_msg.clone());
            messages_handle.set(cloned_messages);
        }
    });

    let cloned_new_message_handle = new_message_handle.clone();
    let on_message_change = Callback::from(move |event: Event| {
        let target = event.target_dyn_into::<HtmlTextAreaElement>();

        if let Some(textarea) = target {
            cloned_new_message_handle.set(textarea.value());
        }
    });

    let cloned_new_msg = new_message.clone();
    let cloned_ws = ws.clone();
    let on_submit = Callback::from(move |_| {
        cloned_ws.send(cloned_new_msg.clone());
        new_message_handle.set("".to_string());
    });

    html! {
        <>
        <ul>
            {
                messages.iter().map(|message| html!{
                    <li>{message}</li>
                }).collect::<Html>()
            }
        </ul>
            <textarea placeholder="Enter your message" value={new_message} onchange={on_message_change}>
            </textarea>
            <button type="submit" onclick={on_submit}>{"Send"}</button>
        </>
    }
}
