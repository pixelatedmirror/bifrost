use dioxus::prelude::*;

use crate::daisyui::Level;

#[derive(Clone, Props, PartialEq)]
pub struct BadgeProps {
    #[props(default = false)]
    soft: bool,

    #[props(default = false)]
    dash: bool,

    #[props(default = false)]
    outline: bool,

    level: Option<Level>,

    class: Option<&'static str>,

    #[props(extends = button, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(default = None)]
    children: Element,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let mut cls = vec!["badge"];

    if let Some(level) = props.level {
        let base = match level {
            Level::Primary => "badge-primary",
            Level::Secondary => "badge-secondary",
            Level::Accent => "badge-accent",
            Level::Info => "badge-info",
            Level::Success => "badge-success",
            Level::Warning => "badge-warning",
            Level::Error => "badge-error",
        };
        cls.push(base);
    }

    if props.dash {
        cls.push("badge-dash");
    }

    if props.outline {
        cls.push("badge-outline");
    }

    if props.soft {
        cls.push("badge-soft");
    }

    cls.extend(props.class);

    rsx! {
        div {
            class: cls.join(" "),
            ..props.attributes,
            {props.children}
        }
    }
}
