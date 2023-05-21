use yew::prelude::*;

#[function_component]
pub fn MessageView() -> Html {
    html! {
      <article id="message">
        <header>
          <dl class="metadata">
            <dt class="created_at">{ "Received" }</dt>
            <dd class="created_at"></dd>
            <dt class="from">{ "From" }</dt>
            <dd class="from"></dd>
            <dt class="to">{ "To" }</dt>
            <dd class="to"></dd>
            <dt class="subject">{ "Subject" }</dt>
            <dd class="subject"></dd>
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
        <iframe class="body"></iframe>
      </article>
    }
}
