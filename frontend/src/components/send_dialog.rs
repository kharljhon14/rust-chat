use web_sys::HtmlTextAreaElement;
use yew::{
    function_component, html, use_state, Callback, Event, Html, MouseEvent, Properties, TargetCast,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub send_message_callback: Callback<String>,
}

#[function_component(SendDialog)]
pub fn send_dialog(props: &Props) -> Html {
    let new_message_handle = use_state(String::default);
    let new_message = (*new_message_handle).clone();

    let cloned_new_message_handle = new_message_handle.clone();
    let on_message_change = Callback::from(move |event: Event| {
        let target = event.target_dyn_into::<HtmlTextAreaElement>();

        if let Some(textarea) = target {
            cloned_new_message_handle.set(textarea.value());
        }
    });

    let cloned_new_msg = new_message.clone();
    let callback = props.send_message_callback.clone();
    let on_submit = Callback::from(move |_: MouseEvent| {
        callback.emit(cloned_new_msg.clone());
        new_message_handle.set("".to_string());
    });

    html! {
    <div class="input-group">
        <textarea class="form-control" placeholder="Enter your message" value={new_message} onchange={on_message_change}>
        </textarea>
        <button class="btn btn-secondary" type="submit" onclick={on_submit}>{"Send"}</button>
    </div>
    }
}
