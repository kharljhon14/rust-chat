use common::chat::chat::ChatMessage;
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
            let chat_message = serde_json::from_str::<ChatMessage>(&ws_msg).unwrap();

            cloned_messages.push(chat_message);
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
        <div class="container">
            <div class="row">
                <div class="list-group">
                    {
                        messages.iter().map(|chat_message| html!{
                            <div class="list-group-item list-group-item-action">
                                <div class="d-flex w-100 justify-content-between">
                                    <h5>
                                        {chat_message.author.clone()}
                                    </h5>
                                    <small>
                                        {chat_message.created_at.format("%Y-%m-%d %H:%M").to_string()}
                                    </small>
                                </div>
                                <p>
                                    {chat_message.message.clone()}
                                </p>
                            </div>
                        }).collect::<Html>()
                    }
                </div>
            </div>
            <div class="row mt-5">
                <div class="input-group">
                    <textarea class="form-control" placeholder="Enter your message" value={new_message} onchange={on_message_change}>
                    </textarea>
                    <button class="btn btn-secondary" type="submit" onclick={on_submit}>{"Send"}</button>
                </div>
            </div>
        </div>
    }
}
