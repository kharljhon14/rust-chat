use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::{
    function_component, html, use_state, Callback, Event, Html, MouseEvent, Properties, TargetCast,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub send_message_callback: Callback<String>,
    pub change_username_callback: Callback<String>,
    pub username: String,
}

#[function_component(SendDialog)]
pub fn send_dialog(props: &Props) -> Html {
    let new_message_handle = use_state(String::default);
    let new_message = (*new_message_handle).clone();

    // State for editing username
    let is_editing_username_handle = use_state(bool::default);
    let is_editing_username = (*is_editing_username_handle).clone();

    let new_username_handle = use_state(|| props.username.clone());
    let new_usename = (*new_username_handle).clone();

    let cloned_is_editing_username_handle = is_editing_username_handle.clone();
    let update_username_change_click = Callback::from(move |_: MouseEvent| {
        cloned_is_editing_username_handle.set(true);
    });

    let cloned_is_editing_username_handle = is_editing_username_handle.clone();
    let update_username_edit_cancel = Callback::from(move |_: MouseEvent| {
        cloned_is_editing_username_handle.set(false);
    });

    let cloned_change_username_callback = props.change_username_callback.clone();
    let cloned_new_username = new_usename.clone();
    let udate_username_apply = Callback::from(move |_: MouseEvent| {
        cloned_change_username_callback.emit(cloned_new_username.clone());
        is_editing_username_handle.set(false);
    });

    let on_username_change = Callback::from(move |event: Event| {
        let target = event.target_dyn_into::<HtmlInputElement>();

        if let Some(input) = target {
            new_username_handle.set(input.value());
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
    let callback = props.send_message_callback.clone();
    let on_submit = Callback::from(move |_: MouseEvent| {
        callback.emit(cloned_new_msg.clone());
        new_message_handle.set("".to_string());
    });

    html! {
    <div class="input-group">
        if is_editing_username{
            <input type="text" value={props.username.clone()} onchange={on_username_change} />
            <button class="btn btn-danger" onclick={update_username_edit_cancel}>{"Cancel"}</button>
            <button class="btn btn-secondary" onclick={udate_username_apply}>{"Apply"}</button>
        }
        else{
            <button class="btn btn-secondary" onclick={update_username_change_click}>{props.username.clone()}</button>
        }
        <span class="input-group-text">{"Your Message:"}</span>
        <textarea class="form-control" placeholder="Enter your message" value={new_message} onchange={on_message_change}>
        </textarea>
        <button class="btn btn-primary" type="submit" onclick={on_submit}>{"Send"}</button>
    </div>
    }
}
