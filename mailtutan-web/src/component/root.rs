use yew::prelude::*;

use crate::api;
use crate::component::header::Header;
use crate::component::message_list::MessageList;
use crate::component::message_view::MessageView;
use crate::component::resizer::Resizer;
use crate::ws;
use crate::Message;

use web_sys::HtmlTableCellElement;

#[function_component]
pub fn Root() -> Html {
    let messages: UseStateHandle<Vec<Message>> = use_state(|| vec![]);
    let selected_message = use_state(|| None);

    let temp_message = selected_message.clone();
    let temp_messages = messages.clone();

    let onclick = Callback::from(move |e: MouseEvent| {
        log::info!("just clicked");

        let element: HtmlTableCellElement = e.target_unchecked_into();
        let id = element
            .parent_element()
            .unwrap()
            .get_attribute("data-message-id")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let msg: Message = (*temp_messages.get(id - 1).unwrap()).clone();
        temp_message.set(Some(msg));
    });

    {
        let messages = messages.clone();

        use_effect_with_deps(
            move |_| {
                api::fetch_messages(messages);
                || ()
            },
            (),
        );
    }
    // ws::listen(messages.clone());

    html! {
        <>
            <Header/>
            <MessageList messages={(*messages).clone()} onclick={onclick} />
            <Resizer/>
            <MessageView message={(*selected_message).clone()} />
        </>
    }
}
