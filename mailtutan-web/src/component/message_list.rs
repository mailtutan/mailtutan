use crate::State;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onclick: Callback<MouseEvent>,
}

#[function_component]
pub fn MessageList(Props { onclick }: &Props) -> Html {
    let (state, _) = use_store::<State>();

    let list = state
        .messages
        .iter()
        .map(|(_id, message)| {
            let class = if state.selected_message.is_none() {
                ""
            } else {
                if state.selected_message.as_ref().unwrap().id.unwrap() == message.id.unwrap() {
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
