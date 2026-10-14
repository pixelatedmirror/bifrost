use dioxus::prelude::*;

#[derive(Clone, Props, PartialEq)]
pub struct CardProps {
    class: Option<&'static str>,

    #[props(extends = button, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(default = None)]
    children: Element,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    let mut cls = vec!["card"];
    if let Some(class) = props.class {
        cls.push(class);
    }

    rsx! {
        div {
            class: cls.join(" "),
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct CardBodyProps {
    class: Option<&'static str>,

    #[props(extends = button, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(default = None)]
    children: Element,
}

#[component]
pub fn CardBody(props: CardBodyProps) -> Element {
    let mut cls = vec!["card-body"];
    if let Some(class) = props.class {
        cls.push(class);
    }

    rsx! {
        div {
            class: cls.join(" "),
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct CardTitleProps {
    class: Option<&'static str>,

    #[props(extends = button, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(default = None)]
    children: Element,
}

#[component]
pub fn CardTitle(props: CardTitleProps) -> Element {
    let mut cls = vec!["card-title"];
    if let Some(class) = props.class {
        cls.push(class);
    }

    rsx! {
        h2 {
            class: cls.join(" "),
            ..props.attributes,
            {props.children}
        }
    }
}
