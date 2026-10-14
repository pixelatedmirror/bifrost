/* use std::collections::HashMap; */
use std::num::NonZeroU32;

use dioxus::prelude::*;
use reqwest::Url;

use bifrost_api::config::Z2mServer;

use crate::component::button::SpinnerButton;
use crate::component::enable_input::EnableInput;
use crate::toast::{Toast, ToastMaster};
use crate::{CLIENT, use_context_signal};

#[allow(clippy::option_if_let_else)]
#[component]
pub fn Z2mServerView(name: String, server: Z2mServer) -> Element {
    rsx! {
        div {
            div {
                class: "bg-base-200 w-full mt-4 p-5 border-b-2 border-primary/40 rounded-t-xl",
                class: "flex gap-4 *:text-nowrap",
                span { class: "badge badge-soft font-mono", "{name}" }
            }
            div {
                class: "bg-base-300 p-5 rounded-b-xl font-mono grid",
                style: "grid-template-columns: 1fr 3fr;",
                span { "url:" } span { "{server.url}" },
                span { "group_prefix:" }
                {
                    if let Some(gp) = &server.group_prefix {
                        rsx! {
                            span { "{gp}" }
                        }
                    } else {
                        rsx! {
                            span {
                                class: "text-base-content/[50%]",
                                "<no prefix>"
                            }
                        }
                    }
                }
                span { "streaming_fps:" } span { "{server.streaming_fps:?}" },
            }
        }
    }
}

#[component]
pub fn Z2mServerAdd(show: Signal<bool>) -> Element {
    let mut name = use_signal(String::new);
    let mut url = use_signal(|| "ws://127.0.0.1:8080/api?token=your-secret-token".to_string());
    let parsed_url = use_memo(move || Url::try_from(url().as_str()));
    let mut group_prefix = use_signal(String::new);
    let mut streaming_fps = use_signal(|| None::<u32>);

    let mut toast = use_context_signal::<ToastMaster>();

    let mut updating = use_signal(|| false);

    let fps_enabled = use_signal(|| false);
    let prefix_enabled = use_signal(|| false);

    rsx! {
        div {
            class: "rounded-md",
            class: "ease-in-out",
            class: "duration-500",
            class: "transition-all",
            class: if show() { "h-auto" } else { "h-0 overflow-hidden" },

            form {
                class: "fieldset",
                class: "lg:w-180 bg-base-200 border border-base-300 p-4 rounded-box",
                onsubmit: move |event| {
                    tracing::info!("onsubmit: {event:?}");

                    async move {
                        updating.set(true);
                        /* let mut form: Z2mForm = parse_form_data(&event.data)?; */
                        /* if form.server.group_prefix.as_ref().is_some_and(String::is_empty) { */
                        /*     form.server.group_prefix = None; */
                        /* } */
                        let name = event.data.values()["name"].as_value();
                        let server = Z2mServer {
                            url: Url::parse(&url.read())?,
                            group_prefix: if group_prefix.read().is_empty() { None } else { Some(group_prefix.read().to_string())},
                            disable_tls_verify: None,
                            streaming_fps: (*streaming_fps.read()).and_then(|fps| NonZeroU32::try_from(fps).ok())
                        };

                        /* if let Err(err) = CLIENT.post_backend(&form.name, form.server).await { */
                        if let Err(err) = CLIENT.post_backend(&name, server).await {
                            toast.write().add(Toast::error(rsx! { "Error: {err}" }).with_timeout_secs(3));
                        }

                        let config = CLIENT.config().await?;

                        CLIENT.put_config(config).await?;

                        show.set(false);
                        updating.set(false);
                        Ok(())
                    }
                },

                legend {
                    class: "fieldset-legend",
                    "Add new z2m backend"
                }

                hr {}

                fieldset {
                    class: "fieldset",

                    label {
                        class: "fieldset-label",
                        "Name"
                    }
                    input {
                        name: "name",
                        pattern: "[A-Za-z_][A-Za-z0-9_]+",
                        class: "input w-full font-mono validator",
                        placeholder: "z2m",
                        value: "{name}",
                        oninput: move |e| name.set(e.value()),
                    }
                    p {
                        class: "validator-hint font-bold font-mono",
                        "Allowed characters: A-Z, a-z, 0-9, _ (and cannot start with a digit)"
                    }

                    label {
                        class: "fieldset-label",
                        "URL"
                    }
                    input {
                        name: "url",
                        class: "input w-full font-mono validator",
                        placeholder: "ws://...",
                        value: "{url}",
                        oninput: move |e| url.set(e.value()),
                    }

                    match parsed_url() {
                        Ok(_url) => rsx! {},
                        Err(error) => rsx! {
                            div {
                                class: "font-bold",
                                class: "text-red-500",
                                "Url is invalid: {error}"
                            }
                        }
                    }

                    EnableInput {
                        enabled: prefix_enabled,
                        label: rsx! { "Group prefix" },

                        input {
                            class: "input w-full",
                            name: "group_prefix",
                            placeholder: "example: bifrost_",
                            value: "{group_prefix}",
                            disabled: !prefix_enabled(),
                            oninput: move |e| group_prefix.set(e.value()),
                        }
                    }

                    EnableInput {
                        enabled: fps_enabled,
                        label: rsx! { "Streaming FPS" },
                        input {
                            class: "input validator w-full",
                            name: "streaming_fps",
                            type: "number",
                            placeholder: "example: 30",
                            disabled: !fps_enabled(),
                            value: streaming_fps,
                            min: "1",
                            max: "120",
                            oninput: move |e| {
                                let v = e.value();
                                if let Ok(fps) = v.parse() {
                                    streaming_fps.set(Some(fps));
                                } else if v.is_empty() {
                                    streaming_fps.set(None);
                                }
                            },
                        }
                        p {
                            class: "validator-hint font-bold",
                            "Must be between be 1 and 120 (recommended range: 5-60)"
                        }
                    }

                    SpinnerButton {
                        class: "btn btn-primary mt-4 justify-self-end min-w-30",
                        onclick: move |_| {},
                        type: "submit",
                        updating,
                        "Add"
                    }
                }
            }
        }
    }
}
