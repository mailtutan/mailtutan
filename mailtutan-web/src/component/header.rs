use yew::prelude::*;

#[function_component]
pub fn Header() -> Html {
    html! {
        <header>
        <h1>{ "Mailtutan" }</h1>
        <nav class="app">
          <ul>
            <li class="search hidden"><input type="search" name="search" placeholder="Search messages..." incremental="true" /></li>
            <li class="clear"><a href="#" title="Clear all messages">{ "Clear" }</a></li>
              <li class="quit"><a href="#" title="Quit Mailtutan">{ "Quit" }</a></li>
          </ul>
        </nav>
        </header>
    }
}
