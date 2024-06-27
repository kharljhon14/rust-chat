use common::chat::chat::ChatMessage;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub messages: Vec<ChatMessage>,
}

#[function_component(MessageList)]
pub fn message_list(props: &Props) -> Html {
    html!({
        props
            .messages
            .iter()
            .map(|chat_message| {
                html! {
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
                }
            })
            .collect::<Html>()
    })
}
