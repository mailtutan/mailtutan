use crate::Message;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub message: Option<Message>,
}

#[function_component]
pub fn MessageView(Props { message }: &Props) -> Html {
    let default_message = &Message::default();
    let message = message.as_ref().unwrap_or(default_message);

    let iframe_src = match message.id {
        Some(id) => format!("{}{}{}", "/api/messages/", id, "/plain"),
        None => "about:blank".to_owned(),
    };

    log::info!("src : {}", iframe_src);

    html! {
      <article id="message">
        <header>
          <dl class="metadata">
            <dt class="created_at">{ "Received" }</dt>
            <dd class="created_at">{ &message.created_at }</dd>
            <dt class="from">{ "From" }</dt>
            <dd class="from">{ &message.sender }</dd>
            <dt class="to">{ "To" }</dt>
            <dd class="to">{ &message.recipients.join(", ") }</dd>
            <dt class="subject">{ "Subject" }</dt>
            <dd class="subject">{ &message.subject }</dd>
            <dt class="attachments">{ "Attachments" }</dt>
            <dd class="attachments"></dd>
          </dl>
          <nav class="views">
            <ul>
              <li class="format tab html selected" data-message-format="html"><a href="#">{ "HTML" }</a></li>
              <li class="format tab plain" data-message-format="plain"><a href="#">{ "Plain Text" }</a></li>
              <li class="format tab source" data-message-format="source"><a href="#">{ "Source" }</a></li>
              <li class="action download" data-message-format="html"><a href="#" class="button"><span>{ "Download" }</span></a></li>
            </ul>
          </nav>
        </header>
        <iframe class="body" src={ iframe_src }></iframe>
      </article>
    }
}
