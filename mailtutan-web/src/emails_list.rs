use crate::Message;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EmailsListProps {
    pub messages: Vec<Message>,
}

#[function_component]
pub fn EmailsList(EmailsListProps { messages }: &EmailsListProps) -> Html {
    let list = messages
        .iter()
        .map(|message| {
            html! {
                <tr>
                    <td>{ &message.sender }</td>
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
