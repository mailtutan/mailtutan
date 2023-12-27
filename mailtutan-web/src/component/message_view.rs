use crate::State;
use web_sys::{HtmlIFrameElement, HtmlLiElement};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component]
pub fn MessageView() -> Html {
    let (state, _) = use_store::<State>();

    if state.selected_message.is_none() {
        return html!();
    }

    let message = state.selected_message.as_ref().unwrap();
    let default_format = "source".to_owned();

    let selected_format = use_state(|| {
        for format in ["html", "plain", "source"].into_iter() {
            let format = format.to_string();
            if message.formats.contains(&format) {
                return format;
            }
        }

        default_format
    });

    let iframe_src = match message.id {
        Some(id) => format!("{}{}/{}", "/api/messages/", id, *selected_format),
        None => "about:blank".to_owned(),
    };

    let onclick = {
        let selected_format = selected_format.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
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

    let mut attachments: Vec<Html> = vec![];

    for attachment in &message.attachments {
        let link = format!(
            "/api/messages/{}/parts/{}",
            message.id.unwrap(),
            attachment.cid
        );

        attachments.push(html! {
            <a href={ link }>{ &attachment.filename }</a>
        });
    }

    let download_link = format!("/api/messages/{}/eml", message.id.unwrap());

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
            <dd class="attachments">
                { attachments }
            </dd>
          </dl>
        </header>
        <nav class="views">
          <ul>
            <li onclick={&onclick} class={tab_classes("html")} data-message-format="html"><a href="#">{ "HTML" }</a></li>
            <li onclick={&onclick} class={tab_classes("plain")} data-message-format="plain"><a href="#">{ "Plain Text" }</a></li>
            <li onclick={&onclick} class={tab_classes("source")} data-message-format="source"><a href="#">{ "Source" }</a></li>
            <li class="format tab" data-message-format="html"><a href={ download_link } target="_new"><span>{ "Download" }</span></a></li>
          </ul>
        </nav>
        <iframe class="body" src={ iframe_src }></iframe>
      </article>
    }
}
