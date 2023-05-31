use crate::Message;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub messages: Vec<Message>,
    pub selected_message: Option<Message>,
    pub onclick: Callback<MouseEvent>,
}

#[function_component]
pub fn MessageList(
    Props {
        messages,
        onclick,
        selected_message,
    }: &Props,
) -> Html {
    let list = messages
        .iter()
        .map(|message| {
            let class = if selected_message.is_none() {
                ""
            } else {
                if selected_message.as_ref().unwrap().id.unwrap() == message.id.unwrap() {
                    "selected"
                } else {
                    ""
                }
            };

            html! {
                <tr {onclick} data-message-id={ message.id.unwrap().to_string() } class={ class }>
                    <td name={ "mohsen" }>{ &message.sender }</td>
                    <td>{ &message.recipients.join(",") }</td>
                    <td>{ &message.subject }</td>
                    <td>{ &message.created_at }</td>
                </tr>
            }
        })
        .collect::<Html>();

    html! {
        <nav id="messages" style="height: 210px;">
            <table>
              <thead>
                <tr>
                  <th>{ "From" }</th>
                  <th>{ "To" }</th>
                  <th>{ "Subject" }</th>
                  <th>{ "Received" }</th>
                </tr>
              </thead>
              <tbody>
                { list }
              </tbody>
            </table>
        </nav>
    }
}
