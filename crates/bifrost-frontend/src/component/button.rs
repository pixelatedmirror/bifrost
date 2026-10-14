use dioxus::prelude::*;

use crate::icons::Spinner;
use crate::traits::Optional;

#[derive(Props, PartialEq, Clone)]
pub struct ButtonProps {
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)]
    class: &'static [&'static str],
    #[props(extends = GlobalAttributes, extends = button)]
    attributes: Vec<Attribute>,
    #[props(optional, default = None)]
    children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    /* let clss = props.class.join(" "); */
    rsx! {
        button {
            class: [
                &[
                    "rounded-md",
                    "rounded-tl-2xl",
                    "rounded-br-2xl",
                    "inset-shadow-sm",
                    "duration-200",
                    "bg-sky-600",
                    "px-6",
                    "py-2",
                    "hover:ease-in-out",
                    "hover:bg-sky-500",
                    "active:inset-shadow-sky-800",
                    "active:duration-0",
                    "active:translate-y-1/25",
                    "disabled:bg-gray-500",
                ],
                props.class
            ].concat().join(" "),
            onclick: move |evt| props.onclick.call_if_some(evt),
            ..props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn SpinnerButton(
    onclick: EventHandler<MouseEvent>,
    updating: Signal<bool>,
    children: Element,
    #[props(extends = button, extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        button {
            class: "rounded-md rounded-tl-2xl rounded-br-2xl inset-shadow-sm duration-200 bg-sky-600 px-6 py-2 hover:ease-in-out hover:bg-sky-500 active:inset-shadow-sky-800 active:duration-0 active:translate-y-1/25 disabled:bg-gray-500",
            onclick: move |evt| onclick.call(evt),
            ..attributes,

            if updating() {
                Spinner {}
            } else {
                {children}
            }
        }
    }
}

#[component]
pub fn Submit(children: Element, value: Option<String>) -> Element {
    rsx! {
        button {
            class: "rounded-md rounded-tl-2xl rounded-br-2xl inset-shadow-sm duration-200 bg-sky-600 px-6 py-2 hover:ease-in-out hover:bg-sky-500 active:inset-shadow-sky-800 active:duration-0 active:translate-y-1/25 disabled:bg-gray-500 col-2",
            r#type: "submit",
            value,
            {children}
        }
    }
}
