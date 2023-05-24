use crate::Message;
use web_sys::HtmlLiElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub message: Option<Message>,
}

#[function_component]
pub fn MessageView(Props { message }: &Props) -> Html {
    let default_format = "source".to_owned();
    let default_message = &Message::default();
    let message = message.as_ref().unwrap_or(default_message);
    let selected_format = use_state(|| {
        message
            .formats
            .last()
            .unwrap_or_else(|| &default_format)
            .to_owned()
    });

    let iframe_src = match message.id {
        Some(id) => format!("{}{}/{}", "/api/messages/", id, *selected_format),
        None => "about:blank".to_owned(),
    };

    let onclick = {
        let selected_format = selected_format.clone();

        Callback::from(move |e: MouseEvent| {
            let element: HtmlLiElement = e.target_unchecked_into();
            let format = element
                .parent_element()
                .unwrap()
                .get_attribute("data-message-format")
                .unwrap()
                .parse::<String>()
                .unwrap();

            selected_format.set(format);
        })
    };

    let tab_classes = move |format: &str| -> String {
        let hidden = if message.formats.contains(&format.to_owned()) {
            ""
        } else {
            "hidden"
        };

        let selected = if format == *selected_format {
            "selected"
        } else {
            ""
        };

        format!("format tab {} {} {}", format, selected, hidden)
    };

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
              <li onclick={&onclick} class={tab_classes("html")} data-message-format="html"><a href="#">{ "HTML" }</a></li>
              <li onclick={&onclick} class={tab_classes("plain")} data-message-format="plain"><a href="#">{ "Plain Text" }</a></li>
              <li onclick={&onclick} class={tab_classes("source")} data-message-format="source"><a href="#">{ "Source" }</a></li>
              <li class="action download" data-message-format="html"><a href="#" class="button"><span>{ "Download" }</span></a></li>
            </ul>
          </nav>
        </header>
        <iframe class="body" src={ iframe_src }></iframe>
      </article>
    }
}
