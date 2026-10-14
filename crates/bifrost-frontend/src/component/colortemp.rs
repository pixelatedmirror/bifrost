use dioxus::prelude::*;

use hue::api::MirekSchema;
use hue::clamp::Clamp;
use hue::colorspace::SRGB;
use hue::colortemp::cct_to_xy;

use crate::traits::Optional;

#[must_use]
pub const fn mirek_to_kelvin(mirek: u32) -> u32 {
    1_000_000 / mirek
}

#[allow(
    clippy::suboptimal_flops,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
fn css_color_temperature_steps(range: MirekSchema) -> String {
    const STEPS: u32 = 7;

    let mut parts = vec![];

    let span = f64::from(range.mirek_maximum - range.mirek_minimum);
    let base = f64::from(range.mirek_minimum);
    let step = span / f64::from(STEPS);

    for x in 0..=STEPS {
        let pct = f64::from(x) / f64::from(STEPS) * 100.0;
        let kelvin = mirek_to_kelvin((base + step * f64::from(x)) as u32);
        let xy = cct_to_xy(f64::from(kelvin));

        /* let [x, y, z] = WIDE.xyy_to_xyz(xy.x, xy.y, 10.0); */
        /* parts.push(format!("color(xyz-d50 {x:.3} {y:.3} {z:.3}) {pct:.0}%")); */
        let rgb = SRGB
            .xy_to_rgb_color(xy.x, xy.y, 255.0)
            .map(Clamp::unit_to_u8_clamped);
        parts.push(format!("rgb({}, {}, {}) {pct:.0}%", rgb[0], rgb[1], rgb[2]));
    }

    let gradient = parts.join(",");

    format!("background: linear-gradient(90deg, {gradient})")
}

#[component]
pub fn ColorTemp(
    value: Option<u32>,
    range: MirekSchema,
    onchange: Option<EventHandler<FormEvent>>,
    oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    let middle = range.mirek_maximum.midpoint(range.mirek_minimum);
    rsx! {
        div {
            class: "grow",
            div { class: "flex justify-between px-2.5 mt-2 text-xs",
                  span { "{mirek_to_kelvin(range.mirek_minimum)}K" }
                  span { "{mirek_to_kelvin(middle)}K" }
                  span { "{mirek_to_kelvin(range.mirek_maximum)}K" }
            }
            div { class: "flex justify-between px-2.5 mt-2 text-xs",
                  span { "|" }
                  span { "|" }
                  span { "|" }
            }
            input {
                type: "range",
                class: "range",
                class: "range-xs",
                class: "w-full",
                class: "[--range-fill:0]",
                style: css_color_temperature_steps(range),
                min: "{range.mirek_minimum}",
                max: "{range.mirek_maximum}",
                onchange: move |evt| onchange.call_if_some(evt),
                oninput: move |evt| oninput.call_if_some(evt),
                value,
            }
            div { class: "flex justify-between px-2.5 mt-2 text-xs",
                  span { "|" }
                  span { "|" }
                  span { "|" }
            }
            div { class: "flex justify-between px-2.5 mt-2 text-xs gap-3",
                  span { "{range.mirek_minimum}mirek" }
                  span { "{middle}mirek" }
                  span { "{range.mirek_maximum}mirek" }
            }
        }
    }
}
