use yew::prelude::*;

use crate::api;
use crate::component::header::Header;
use crate::component::message_list::MessageList;
use crate::component::message_view::MessageView;
use crate::ws;
use crate::Message;
use gloo_dialogs::confirm;

use web_sys::HtmlTableCellElement;

#[function_component]
pub fn Root() -> Html {
    let messages: UseStateHandle<Vec<Message>> = use_state(|| vec![]);
    let selected_message = use_state(|| None);

    let clear_onclick = {
        let messages = messages.clone();

        Callback::from(move |_: MouseEvent| {
            if confirm("Do you want to clear all messages?") {
                api::delete_messages();

                // use_effect_with_deps(
                //     move |_| {
                //         api::fetch_messages(messages);
                //     },
                //     (),
                // );
                messages.set(vec![]);
            }
        })
    };

    let onclick = {
        let selected_message = selected_message.clone();
        let messages = messages.clone();

        Callback::from(move |e: MouseEvent| {
            let element: HtmlTableCellElement = e.target_unchecked_into();
            let id = element
                .parent_element()
                .unwrap()
                .get_attribute("data-message-id")
                .unwrap()
                .parse::<usize>()
                .unwrap();

            let msg: Message = (*messages.get(id - 1).unwrap()).clone();
            selected_message.set(Some(msg));
        })
    };

    {
        let messages = messages.clone();

        use_effect_with_deps(
            move |_| {
                api::fetch_messages(messages);
            },
            (),
        );
    }
    ws::listen(messages.clone());

    html! {
        <>
            <Header clear_onclick={clear_onclick} />
            <div class="main">
                <MessageList messages={(*messages).clone()} selected_message={{(*selected_message).clone()}} onclick={onclick} />
                <MessageView message={(*selected_message).clone()} />
            </div>
        </>
    }
}
