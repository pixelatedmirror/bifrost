use std::collections::BTreeMap;

use dioxus::prelude::*;
use uuid::Uuid;

use hue::api::{Group, GroupedLight, Resource, ResourceRecord};

use crate::component::group::{GroupOnIcon, GroupView};
use crate::daisyui::Level;
use crate::daisyui::badge::Badge;
use crate::icons::RoomIcon;
use crate::use_context_signal;

#[component]
pub fn GroupsView() -> Element {
    let rres = use_context_signal::<BTreeMap<Uuid, ResourceRecord>>();
    let res = &*rres.read();

    let groups: Vec<_> = res
        .iter()
        .filter_map(|(uuid, item)| match &item.obj {
            Resource::Room(room) => Some((
                *uuid,
                item.id_v1.as_deref().unwrap_or("-").to_string(),
                room.metadata.clone(),
                room.grouped_light_service().copied(),
                room.children.iter().copied().collect::<Vec<_>>(),
                "Room",
            )),
            Resource::Zone(zone) => Some((
                *uuid,
                item.id_v1.as_deref().unwrap_or("-").to_string(),
                zone.metadata.clone(),
                zone.grouped_light_service().copied(),
                zone.children.iter().copied().collect::<Vec<_>>(),
                "Zone",
            )),
            _ => None,
        })
        .collect();

    rsx! {
        div {
            class: "grid gap-4 max-w-200",
            for (uuid, id_v1, metadata, grouped_light, members, kind) in groups {
                div {
                    key: "{uuid}",
                    div {
                        class: "bg-base-200 w-full p-5 border-b-2 border-primary/40 rounded-t-xl",
                        div {
                            class: "flex flex-col lg:flex-row *:text-nowrap items-center gap-2",
                            RoomIcon { archetype: metadata.archetype }

                            if let Some(gl) = grouped_light {
                                if let Some(ResourceRecord {
                                    obj: Resource::GroupedLight(GroupedLight { on: Some(on), ..}),
                                    ..
                                }) = res.get(&gl.rid) {
                                    GroupOnIcon { id: gl.rid, on: *on }
                                }
                            }

                            div {
                                class: "grow",
                                Badge {
                                    level: Level::Info,
                                    soft: true,
                                    class: "font-mono mr-2",
                                    "{metadata.name}"
                                }
                                Badge {
                                    level: Level::Secondary,
                                    soft: true,
                                    class: "font-mono text-xs",
                                    "{kind}"
                                }
                            }

                            Badge {
                                level: Level::Info,
                                soft: true,
                                class: "font-mono",
                                "{id_v1}"
                            }
                            "|",
                            Badge {
                                level: Level::Info,
                                soft: true,
                                class: "font-mono",
                                "{uuid}"
                            }
                        }
                    }
                    div {
                        class: "bg-base-300 p-5 rounded-b-xl",
                        GroupView { res: rres, id: uuid, grouped_light, members }
                    }
                }
            }
        }
    }
}
