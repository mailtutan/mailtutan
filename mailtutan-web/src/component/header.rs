use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub clear_onclick: Callback<MouseEvent>,
}

#[function_component]
pub fn Header(Props { clear_onclick }: &Props) -> Html {
    let mailtutan: String = "MAILTUTAN"
        .chars()
        .map(|f| format!("{}{}", f, "\u{00a0}".repeat(5)))
        .collect();

    html! {
        <header>
        <h1>{ mailtutan }</h1>
        <nav class="app">
          <ul>
            <li class="search hidden"><input type="search" name="search" placeholder="Search messages..." incremental="true" /></li>
            <li class="clear"><a href="#" onclick={clear_onclick} title="Clear all messages">{ "Clear" }</a></li>
          </ul>
        </nav>
        </header>
    }
}
