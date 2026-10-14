use chrono::{DateTime, Duration, Utc};
use dioxus::prelude::*;
use futures_util::StreamExt;
use gloo_timers::future::IntervalStream;

use crate::icons::{IconError, IconInfo, IconSuccess, IconWarn};
use crate::traits::{HasClass, HasIcon};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToastLevel {
    Info,
    Success,
    Warning,
    Error,
}

impl HasClass for ToastLevel {
    #[must_use]
    fn class(&self) -> &'static str {
        match self {
            Self::Info => "alert alert-info",
            Self::Success => "alert alert-success",
            Self::Warning => "alert alert-warning",
            Self::Error => "alert alert-error",
        }
    }
}

impl HasIcon for ToastLevel {
    fn icon(&self) -> Element {
        match self {
            Self::Info => rsx! { IconInfo { } },
            Self::Success => rsx! { IconSuccess { } },
            Self::Warning => rsx! { IconWarn { } },
            Self::Error => rsx! { IconError { } },
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Toast {
    elm: Element,
    level: ToastLevel,
    deadline: Option<DateTime<Utc>>,
}

impl Toast {
    #[must_use]
    pub const fn new(elm: Element, level: ToastLevel, deadline: Option<DateTime<Utc>>) -> Self {
        Self {
            elm,
            level,
            deadline,
        }
    }

    #[must_use]
    pub const fn info(elm: Element) -> Self {
        Self::new(elm, ToastLevel::Info, None)
    }

    #[must_use]
    pub const fn success(elm: Element) -> Self {
        Self::new(elm, ToastLevel::Success, None)
    }

    #[must_use]
    pub const fn warning(elm: Element) -> Self {
        Self::new(elm, ToastLevel::Warning, None)
    }

    #[must_use]
    pub const fn error(elm: Element) -> Self {
        Self::new(elm, ToastLevel::Error, None)
    }

    #[must_use]
    pub fn with_timeout(self, timeout: Duration) -> Self {
        Self {
            deadline: Some(Utc::now() + timeout),
            ..self
        }
    }

    #[must_use]
    pub fn with_timeout_secs(self, seconds: i64) -> Self {
        Self {
            deadline: Some(Utc::now() + Duration::seconds(seconds)),
            ..self
        }
    }

    #[must_use]
    pub fn with_deadline(self, deadline: DateTime<Utc>) -> Self {
        Self {
            deadline: Some(deadline),
            ..self
        }
    }
}

#[derive(Clone, PartialEq, Default)]
pub struct ToastMaster {
    toasts: Vec<Toast>,
}

impl ToastMaster {
    #[must_use]
    pub const fn new() -> Self {
        Self { toasts: vec![] }
    }

    pub fn add(&mut self, toast: Toast) {
        self.toasts.push(toast);
    }
}

#[component]
pub fn ToastFrame(master: Signal<ToastMaster>) -> Element {
    let _ = use_resource(move || async move {
        let mut tick = IntervalStream::new(100);
        loop {
            let now = Utc::now();
            tick.next().await;
            master
                .write()
                .toasts
                .retain(|toast| toast.deadline.is_none_or(|dl| now < dl));
        }
    });

    rsx! {
        div {
            class: "toast",
            for toast in &master.read().toasts {
                div {
                    role: "alert",
                    class: format!("duration-500 {}", toast.level.class()),
                    { toast.level.icon() }
                    span {
                        {toast.elm.clone()}
                    }
                }
            }
        }
    }
}
