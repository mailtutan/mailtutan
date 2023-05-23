use crate::Message;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EmailsListProps {
    pub messages: Vec<Message>,
    pub onclick: Callback<MouseEvent>,
}

#[function_component]
pub fn EmailsList(EmailsListProps { messages, onclick }: &EmailsListProps) -> Html {
    let list = messages
        .iter()
        .map(|message| {
            html! {
                <tr {onclick} data-message-id={ message.id.unwrap().to_string() }>
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
