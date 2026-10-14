use dioxus::prelude::*;

use hue::api::GroupArchetype;

use crate::hue_icons;

#[component]
pub fn SvgIcon(path: String) -> Element {
    rsx! {
        svg {
            fill: "none",
            view_box: "0 0 24 24",
            class: "h-6 w-6 shrink-0 stroke-current inline mr-2",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "2",
                d: "{path}"
            }
        }
    }
}

#[component]
pub fn SvgIconFilled(path: String) -> Element {
    rsx! {
        svg {
            fill: "currentColor",
            view_box: "0 0 24 24",
            class: "h-6 w-6 shrink-0 inline mr-2",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "2",
                d: "{path}"
            }
        }
    }
}

#[component]
pub fn IconInfo() -> Element {
    rsx! {
        SvgIcon {
            path: "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
        }
    }
}

#[component]
pub fn IconSuccess() -> Element {
    rsx! {
        SvgIcon {
            path: "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
        }
    }
}

#[component]
pub fn IconWarn() -> Element {
    rsx! {
        SvgIcon {
            path: "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
        }
    }
}

#[component]
pub fn IconError() -> Element {
    rsx! {
        SvgIcon {
            path: "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
        }
    }
}

// from flowbite.com (this svg is MIT-licensed)
#[component]
pub fn IconGithub() -> Element {
    rsx! {
        SvgIconFilled {
            path: "M12.006 2a9.847 9.847 0 0 0-6.484 2.44 10.32 10.32 0 0 0-3.393 6.17 10.48 10.48 0 0 0 1.317 6.955 10.045 10.045 0 0 0 5.4 4.418c.504.095.683-.223.683-.494 0-.245-.01-1.052-.014-1.908-2.78.62-3.366-1.21-3.366-1.21a2.711 2.711 0 0 0-1.11-1.5c-.907-.637.07-.621.07-.621.317.044.62.163.885.346.266.183.487.426.647.71.135.253.318.476.538.655a2.079 2.079 0 0 0 2.37.196c.045-.52.27-1.006.635-1.37-2.219-.259-4.554-1.138-4.554-5.07a4.022 4.022 0 0 1 1.031-2.75 3.77 3.77 0 0 1 .096-2.713s.839-.275 2.749 1.05a9.26 9.26 0 0 1 5.004 0c1.906-1.325 2.74-1.05 2.74-1.05.37.858.406 1.828.101 2.713a4.017 4.017 0 0 1 1.029 2.75c0 3.939-2.339 4.805-4.564 5.058a2.471 2.471 0 0 1 .679 1.897c0 1.372-.012 2.477-.012 2.814 0 .272.18.592.687.492a10.05 10.05 0 0 0 5.388-4.421 10.473 10.473 0 0 0 1.313-6.948 10.32 10.32 0 0 0-3.39-6.165A9.847 9.847 0 0 0 12.007 2Z"
        }
    }
}

// from flowbite.com (this svg is MIT-licensed)
#[component]
pub fn IconDiscord() -> Element {
    rsx! {
        SvgIconFilled {
            path: "M18.942 5.556a16.3 16.3 0 0 0-4.126-1.3 12.04 12.04 0 0 0-.529 1.1 15.175 15.175 0 0 0-4.573 0 11.586 11.586 0 0 0-.535-1.1 16.274 16.274 0 0 0-4.129 1.3 17.392 17.392 0 0 0-2.868 11.662 15.785 15.785 0 0 0 4.963 2.521c.41-.564.773-1.16 1.084-1.785a10.638 10.638 0 0 1-1.706-.83c.143-.106.283-.217.418-.331a11.664 11.664 0 0 0 10.118 0c.137.114.277.225.418.331-.544.328-1.116.606-1.71.832a12.58 12.58 0 0 0 1.084 1.785 16.46 16.46 0 0 0 5.064-2.595 17.286 17.286 0 0 0-2.973-11.59ZM8.678 14.813a1.94 1.94 0 0 1-1.8-2.045 1.93 1.93 0 0 1 1.8-2.047 1.918 1.918 0 0 1 1.8 2.047 1.929 1.929 0 0 1-1.8 2.045Zm6.644 0a1.94 1.94 0 0 1-1.8-2.045 1.93 1.93 0 0 1 1.8-2.047 1.919 1.919 0 0 1 1.8 2.047 1.93 1.93 0 0 1-1.8 2.045Z"
        }
    }
}

// from flowbite.com (this svg is MIT-licensed)
#[component]
pub fn IconLightbulbFilled() -> Element {
    rsx! {
        SvgIconFilled {
            path: "M7.05 4.05A7 7 0 0 1 19 9c0 2.407-1.197 3.874-2.186 5.084l-.04.048C15.77 15.362 15 16.34 15 18a1 1 0 0 1-1 1h-4a1 1 0 0 1-1-1c0-1.612-.77-2.613-1.78-3.875l-.045-.056C6.193 12.842 5 11.352 5 9a7 7 0 0 1 2.05-4.95ZM9 21a1 1 0 0 1 1-1h4a1 1 0 1 1 0 2h-4a1 1 0 0 1-1-1Zm1.586-13.414A2 2 0 0 1 12 7a1 1 0 1 0 0-2 4 4 0 0 0-4 4 1 1 0 0 0 2 0 2 2 0 0 1 .586-1.414Z"
        }
    }
}

// from flowbite.com (this svg is MIT-licensed)
#[component]
pub fn IconLightbulb() -> Element {
    rsx! {
        SvgIcon {
            path: "M9 9a3 3 0 0 1 3-3m-2 15h4m0-3c0-4.1 4-4.9 4-9A6 6 0 1 0 6 9c0 4 4 5 4 9h4Z"
        }
    }
}

// public domain
#[component]
pub fn IconChevronUp() -> Element {
    rsx! {
        SvgIcon {
            path: "M11.4697 7.71967C11.7626 7.42678 12.2374 7.42678 12.5303 7.71967L20.0303 15.2197C20.3232 15.5126 20.3232 15.9874 20.0303 16.2803C19.7374 16.5732 19.2626 16.5732 18.9697 16.2803L12 9.31066L5.03033 16.2803C4.73744 16.5732 4.26256 16.5732 3.96967 16.2803C3.67678 15.9874 3.67678 15.5126 3.96967 15.2197L11.4697 7.71967Z",
        }
    }
}

// public domain
#[component]
pub fn IconChevronDown() -> Element {
    rsx! {
        SvgIcon {
            path: "M12.5303 16.2803C12.2374 16.5732 11.7626 16.5732 11.4697 16.2803L3.96967 8.78033C3.67678 8.48744 3.67678 8.01256 3.96967 7.71967C4.26256 7.42678 4.73744 7.42678 5.03033 7.71967L12 14.6893L18.9697 7.71967C19.2626 7.42678 19.7374 7.42678 20.0303 7.71967C20.3232 8.01256 20.3232 8.48744 20.0303 8.78033L12.5303 16.2803Z",
        }
    }
}

// Included with tailwind (MIT-licensed)
#[component]
pub fn Spinner() -> Element {
    rsx! {
        svg {
            width: "24",
            height: "24",
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            style {
                ".spinner{{transform-origin:center;animation:spinner1 .75s infinite linear}}@keyframes spinner1{{100%{{transform:rotate(360deg)}}}}"
            }
            path {
                opacity: ".25",
                d: "M12,1A11,11,0,1,0,23,12,11,11,0,0,0,12,1Zm0,19a8,8,0,1,1,8-8A8,8,0,0,1,12,20Z"
            }
            path {
                class: "spinner",
                d: "M12,4a8,8,0,0,1,7.89,6.7A1.53,1.53,0,0,0,21.38,12h0a1.5,1.5,0,0,0,1.48-1.75,11,11,0,0,0-21.72,0A1.5,1.5,0,0,0,2.62,12h0a1.53,1.53,0,0,0,1.49-1.3A8,8,0,0,1,12,4Z",
            }
        }
    }
}

#[allow(clippy::match_same_arms)]
const fn room_to_icon_path(archetype: GroupArchetype) -> &'static str {
    match archetype {
        GroupArchetype::LivingRoom => hue_icons::ROOM_LIVING,
        GroupArchetype::Kitchen => hue_icons::ROOM_KITCHEN,
        GroupArchetype::Dining => hue_icons::ROOM_DINING,
        GroupArchetype::Bedroom => hue_icons::ROOM_BEDROOM,
        GroupArchetype::KidsBedroom => hue_icons::ROOM_KIDS,
        GroupArchetype::Bathroom => hue_icons::ROOM_BATHROOM,
        GroupArchetype::Nursery => hue_icons::ROOM_NURSERY,
        GroupArchetype::Recreation => hue_icons::ROOM_RECREATION,
        GroupArchetype::Office => hue_icons::ROOM_OFFICE,
        GroupArchetype::Gym => hue_icons::ROOM_GYM,
        GroupArchetype::Hallway => hue_icons::ROOM_HALLWAY,
        GroupArchetype::Toilet => hue_icons::ROOM_TOILET,
        GroupArchetype::FrontDoor => hue_icons::ROOM_FRONT_DOOR,
        GroupArchetype::Garage => hue_icons::ROOM_GARAGE,
        GroupArchetype::Terrace => hue_icons::ROOM_TERRACE,
        GroupArchetype::Garden => hue_icons::ROOM_OUTDOORS,
        GroupArchetype::Driveway => hue_icons::ROOM_DRIVEWAY,
        GroupArchetype::Carport => hue_icons::ROOM_CARPORT,
        GroupArchetype::Home => hue_icons::HOME,
        GroupArchetype::Downstairs => hue_icons::DOWNSTAIRS,
        GroupArchetype::Upstairs => hue_icons::UPSTAIRS,
        GroupArchetype::TopFloor => hue_icons::UPSTAIRS, // ROOM_TOPFLOOR,
        GroupArchetype::Attic => hue_icons::ROOM_ATTIC,
        GroupArchetype::GuestRoom => hue_icons::ROOM_GUESTROOM,
        GroupArchetype::Staircase => hue_icons::ROOM_STAIRS,
        GroupArchetype::Lounge => hue_icons::ROOM_LOUNGE,
        GroupArchetype::ManCave => hue_icons::ROOM_GAMES,
        GroupArchetype::Computer => hue_icons::ROOM_COMPUTER,
        GroupArchetype::Studio => hue_icons::ROOM_STUDIO,
        GroupArchetype::Music => hue_icons::SYNC_MUSIC,
        GroupArchetype::Tv => hue_icons::ROOM_COMPUTER, // ROOM_TV,
        GroupArchetype::Reading => hue_icons::HOME,     // ROOM_READING,
        GroupArchetype::Closet => hue_icons::ROOM_CLOSET,
        GroupArchetype::Storage => hue_icons::ROOM_STORAGE,
        GroupArchetype::LaundryRoom => hue_icons::ROOM_LAUNDRY,
        GroupArchetype::Balcony => hue_icons::ROOM_BALCONY,
        GroupArchetype::Porch => hue_icons::ROOM_PORCH,
        GroupArchetype::Barbecue => hue_icons::ROOM_BBQ,
        GroupArchetype::Pool => hue_icons::ROOM_POOL,
        GroupArchetype::Other => hue_icons::ROOM_OTHER,
    }
}

#[component]
pub fn RoomIcon(archetype: GroupArchetype) -> Element {
    rsx! {
        SvgIconFilled {
            path: room_to_icon_path(archetype)
        }
    }
}
