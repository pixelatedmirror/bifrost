use std::collections::VecDeque;

use bifrost_api::logging::LogRecord;
use dioxus::prelude::*;
use log::Level;

use crate::use_context_signal;

#[must_use]
const fn log_level_to_color(level: Level) -> &'static str {
    match level {
        Level::Error => "text-red-600",
        Level::Warn => "text-yellow-400",
        Level::Info => "text-white",
        Level::Debug => "text-blue-500",
        Level::Trace => "text-fuchsia-500",
    }
}

#[component]
pub fn LogsView() -> Element {
    let logs = use_context_signal::<VecDeque<LogRecord>>();
    rsx! {
        table {
            class: "w-full",
            class: "font-mono",
            class: "**:px-2 **:text-xs",
            for log in &*logs.read() {
                tr {
                    td {
                        class: "text-nowrap",
                        class: "text-gray-400",
                        { log.ts.format("%Y-%m-%d %H:%M:%S.%3fZ").to_string() }
                    }
                    td {
                        class: log_level_to_color(log.metadata.level),
                        "{log.metadata.level}"
                    }
                    td { "{log.metadata.target}" }
                    td { "{log.msg}" }
                }
            }
        }
    }
}
