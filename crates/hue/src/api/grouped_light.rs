use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::{
    ColorGamut, ColorTemperature, ColorTemperatureUpdate, ColorUpdate, DimmingDeltaAction,
    DimmingDeltaUpdate, DimmingUpdate, GamutType, LightAlert, LightColor, LightSignal,
    LightSignaling, MirekSchema, On, ResourceLink, Stub,
};
use crate::hs::HS;
use crate::legacy_api::ApiLightStateUpdate;
use crate::xy::XY;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupedLight {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert: Option<LightAlert>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimming: Option<DimmingUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<LightColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_temperature: Option<ColorTemperature>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_temperature_delta: Option<Stub>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimming_delta: Option<Stub>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamics: Option<Stub>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<On>,
    pub owner: ResourceLink,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signaling: Option<LightSignaling>,
}

impl GroupedLight {
    #[must_use]
    pub fn new(room: ResourceLink) -> Self {
        Self {
            alert: Some(LightAlert {
                action_values: BTreeSet::from([String::from("breathe")]),
            }),
            dimming: Some(DimmingUpdate { brightness: 0.0 }),
            color: Some(LightColor {
                gamut: Some(ColorGamut::GAMUT_C),
                gamut_type: GamutType::C,
                xy: XY {
                    x: 0.3151,
                    y: 0.3224,
                },
            }),
            color_temperature: Some(ColorTemperature {
                mirek: Some(366),
                mirek_schema: MirekSchema::DEFAULT,
                mirek_valid: true,
            }),
            color_temperature_delta: Some(Stub),
            dimming_delta: Some(Stub),
            dynamics: Some(Stub),
            on: Some(On::new(true)),
            owner: room,
            signaling: Some(LightSignaling {
                signal_values: vec![
                    LightSignal::Alternating,
                    LightSignal::NoSignal,
                    LightSignal::OnOff,
                    LightSignal::OnOffColor,
                ],
                status: Value::Null,
            }),
        }
    }

    #[must_use]
    pub fn as_brightness_opt(&self) -> Option<f64> {
        self.dimming.as_ref().map(|br| br.brightness)
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct GroupedLightDynamicsUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,
}

impl GroupedLightDynamicsUpdate {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_duration(self, duration: Option<impl Into<u32>>) -> Self {
        Self {
            duration: duration.map(Into::into),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GroupedLightUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<On>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimming: Option<DimmingUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimming_delta: Option<DimmingDeltaUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<ColorUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_temperature: Option<ColorTemperatureUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<ResourceLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamics: Option<GroupedLightDynamicsUpdate>,
}

impl GroupedLightUpdate {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_brightness(self, brightness: Option<f64>) -> Self {
        Self {
            dimming: brightness.map(DimmingUpdate::new),
            ..self
        }
    }

    #[must_use]
    pub fn with_on(self, on: impl Into<Option<On>>) -> Self {
        Self {
            on: on.into(),
            ..self
        }
    }

    #[must_use]
    pub fn with_color_temperature(self, mirek: impl Into<Option<u16>>) -> Self {
        Self {
            color_temperature: mirek.into().map(ColorTemperatureUpdate::new),
            ..self
        }
    }

    #[must_use]
    pub fn with_color_hs(self, hs: impl Into<Option<HS>>) -> Self {
        Self {
            color: hs.into().map(|hs| XY::from_hs(hs).0).map(ColorUpdate::new),
            ..self
        }
    }

    #[must_use]
    pub fn with_color_xy(self, val: impl Into<Option<XY>>) -> Self {
        Self {
            color: val.into().map(ColorUpdate::new),
            ..self
        }
    }

    #[must_use]
    pub const fn with_dynamics(self, dynamics: Option<GroupedLightDynamicsUpdate>) -> Self {
        Self { dynamics, ..self }
    }

    pub fn with_dimming_delta(self, dimming_delta: Option<DimmingDeltaUpdate>) -> Self {
        Self {
            dimming_delta,
            ..self
        }
    }

    pub fn merge(&mut self, other: Self) {
        if other.on.is_some() {
            self.on = other.on;
        }
        if other.dimming.is_some() {
            self.dimming = other.dimming;
            self.dimming_delta = None;
        } else if let Some(other_delta) = other.dimming_delta {
            if let Some(existing_delta) = &mut self.dimming_delta {
                let existing_val = match existing_delta.action {
                    DimmingDeltaAction::Up => existing_delta.brightness_delta,
                    DimmingDeltaAction::Down => -existing_delta.brightness_delta,
                };
                let other_val = match other_delta.action {
                    DimmingDeltaAction::Up => other_delta.brightness_delta,
                    DimmingDeltaAction::Down => -other_delta.brightness_delta,
                };
                let total = existing_val + other_val;
                if total >= 0.0 {
                    existing_delta.action = DimmingDeltaAction::Up;
                    existing_delta.brightness_delta = total;
                } else {
                    existing_delta.action = DimmingDeltaAction::Down;
                    existing_delta.brightness_delta = -total;
                }
            } else {
                self.dimming_delta = Some(other_delta);
            }
        }
        if other.color.is_some() {
            self.color = other.color;
            self.color_temperature = None;
        }
        if other.color_temperature.is_some() {
            self.color_temperature = other.color_temperature;
            self.color = None;
        }
        if other.owner.is_some() {
            self.owner = other.owner;
        }
        if other.dynamics.is_some() {
            self.dynamics = other.dynamics;
        }
    }
}

/* conversion from v1 api */
impl From<&ApiLightStateUpdate> for GroupedLightUpdate {
    fn from(upd: &ApiLightStateUpdate) -> Self {
        Self::new()
            .with_on(upd.on.map(On::new))
            .with_brightness(upd.bri.map(|b| f64::from(b) / 2.54))
            .with_color_hs(upd.hs.map(Into::into))
            .with_color_xy(upd.xy.map(XY::from))
            .with_color_temperature(upd.ct)
            .with_dynamics(
                upd.transitiontime.map(|t| {
                    GroupedLightDynamicsUpdate::new().with_duration(Some(u32::from(t) * 100))
                }),
            )
    }
}
