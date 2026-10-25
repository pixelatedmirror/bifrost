use std::ops::{AddAssign, Sub};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::{
    ColorTemperatureUpdate, ColorUpdate, DimmingUpdate, LightEffect, LightGradientUpdate, On,
    ResourceLink,
};
use crate::date_format;

#[derive(Copy, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SceneActive {
    Inactive,
    Static,
    DynamicPalette,
}

#[derive(Copy, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct SceneStatus {
    pub active: SceneActive,
    #[serde(
        with = "date_format::utc_ms_opt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_recall: Option<DateTime<Utc>>,
}

#[derive(Copy, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SceneStatusEnum {
    Active,
    Static,
    DynamicPalette,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct PaletteColorTemperature {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirek: Option<u16>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct PaletteEffect {
    pub effect: LightEffect,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScenePaletteColor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<ColorUpdate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimming: Option<DimmingUpdate>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScenePaletteColorTemperature {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color_temperature: Option<PaletteColorTemperature>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimming: Option<DimmingUpdate>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ScenePalette {
    #[serde(default, alias = "colors")]
    pub color: Vec<ScenePaletteColor>,
    #[serde(default)]
    pub color_temperature: Vec<ScenePaletteColorTemperature>,
    #[serde(default)]
    pub dimming: Vec<DimmingUpdate>,
    #[serde(default)]
    pub effects: Vec<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effects_v2: Option<Vec<Value>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ScenePaletteUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Vec<ScenePaletteColor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_temperature: Option<Vec<ScenePaletteColorTemperature>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimming: Option<Vec<DimmingUpdate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effects: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effects_v2: Option<Vec<Value>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Scene {
    #[serde(default)]
    pub actions: Vec<SceneActionElement>,
    #[serde(default)]
    pub auto_dynamic: bool,
    pub group: ResourceLink,
    pub metadata: SceneMetadata,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub palette: Option<ScenePalette>,
    #[serde(default)]
    pub speed: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SceneStatus>,
    #[serde(default)]
    pub recall: SceneRecall,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SceneAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<ColorUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_temperature: Option<ColorTemperatureUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimming: Option<DimmingUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<On>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient: Option<LightGradientUpdate>,
    #[serde(default, skip_serializing_if = "Value::is_null")]
    pub effects: Value,
    #[serde(default, skip_serializing_if = "Value::is_null")]
    pub effects_v2: Value,
    #[serde(default, skip_serializing_if = "Value::is_null")]
    pub dynamics: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SceneActionElement {
    pub action: SceneAction,
    pub target: ResourceLink,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct SceneMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appdata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<ResourceLink>,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct SceneMetadataUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appdata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<ResourceLink>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SceneUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<SceneActionElement>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recall: Option<SceneRecall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<SceneMetadataUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub palette: Option<ScenePaletteUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_dynamic: Option<bool>,
}

impl SceneUpdate {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_actions(self, actions: Option<Vec<SceneActionElement>>) -> Self {
        Self { actions, ..self }
    }

    #[must_use]
    pub fn with_recall_action(self, action: Option<SceneStatus>) -> Self {
        Self {
            recall: Some(SceneRecall {
                action: match action.map(|a| a.active) {
                    Some(SceneActive::DynamicPalette) => Some(SceneStatusEnum::DynamicPalette),
                    Some(SceneActive::Static) => Some(SceneStatusEnum::Active),
                    Some(SceneActive::Inactive) | None => None,
                },
                duration: None,
                dimming: None,
            }),
            ..self
        }
    }
}

impl AddAssign<&ScenePaletteUpdate> for ScenePalette {
    fn add_assign(&mut self, rhs: &ScenePaletteUpdate) {
        if let Some(color) = &rhs.color {
            self.color.clone_from(color);
        }
        if let Some(color_temperature) = &rhs.color_temperature {
            self.color_temperature.clone_from(color_temperature);
        }
        if let Some(dimming) = &rhs.dimming {
            self.dimming.clone_from(dimming);
        }
        if let Some(effects) = &rhs.effects {
            self.effects.clone_from(effects);
        }
        if let Some(effects_v2) = &rhs.effects_v2 {
            self.effects_v2 = Some(effects_v2.clone());
        }
    }
}

impl AddAssign<&SceneUpdate> for Scene {
    fn add_assign(&mut self, upd: &SceneUpdate) {
        if let Some(actions) = &upd.actions {
            self.actions.clone_from(actions);
        }
        if let Some(md) = &upd.metadata {
            self.metadata += md;
        }
        if let Some(palette) = &upd.palette {
            if let Some(pal) = &mut self.palette {
                *pal += palette;
            } else {
                let mut pal = ScenePalette::default();
                pal += palette;
                self.palette = Some(pal);
            }
        }
        if let Some(speed) = upd.speed {
            self.speed = speed;
        }
        if let Some(auto_dynamic) = upd.auto_dynamic {
            self.auto_dynamic = auto_dynamic;
        }
    }
}

impl AddAssign<&SceneMetadataUpdate> for SceneMetadata {
    fn add_assign(&mut self, upd: &SceneMetadataUpdate) {
        if let Some(appdata) = &upd.appdata {
            self.appdata = Some(appdata.to_string());
        }
        if let Some(image) = &upd.image {
            self.image = Some(*image);
        }
        if let Some(name) = &upd.name {
            self.name.clone_from(name);
        }
    }
}

impl Sub<&SceneMetadata> for &SceneMetadata {
    type Output = SceneMetadataUpdate;

    fn sub(self, rhs: &SceneMetadata) -> Self::Output {
        let mut upd = Self::Output::default();

        if self != rhs {
            if self.appdata != rhs.appdata {
                upd.appdata.clone_from(&rhs.appdata);
            }
            if self.image != rhs.image {
                upd.image.clone_from(&rhs.image);
            }
            if self.name != rhs.name {
                upd.name = Some(rhs.name.clone());
            }
        }

        upd
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SceneRecall {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<SceneStatusEnum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimming: Option<DimmingUpdate>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deserialize_hue_app_scene_with_effects_v2() {
        let payload = json!({
            "actions": [
                {
                    "action": {
                        "color": {
                            "xy": {
                                "x": 0.6048986724785972,
                                "y": 0.33962998539643
                            }
                        },
                        "dimming": {
                            "brightness": 28.244504766949152
                        },
                        "on": {
                            "on": true
                        }
                    },
                    "target": {
                        "rid": "e9910a37-9f76-547f-b923-6e272b701834",
                        "rtype": "light"
                    }
                }
            ],
            "group": {
                "rid": "fb94f598-51b5-5d4f-b8e8-5cd7822ded5d",
                "rtype": "zone"
            },
            "metadata": {
                "image": {
                    "rid": "a93d1cb8-3ed3-444f-b727-e23936ab9e9d",
                    "rtype": "public_image"
                },
                "name": "Trick or treat"
            },
            "palette": {
                "color": [
                    {
                        "color": {
                            "xy": {
                                "x": 0.1543920758280876,
                                "y": 0.07251010879761448
                            }
                        },
                        "dimming": {
                            "brightness": 55.00000000000001
                        }
                    }
                ],
                "color_temperature": [
                    {
                        "color_temperature": {
                            "mirek": 434
                        },
                        "dimming": {
                            "brightness": 55.00000000000001
                        }
                    }
                ],
                "dimming": [],
                "effects_v2": [
                    {
                        "action": {
                            "effect": "cosmos",
                            "parameters": {
                                "color": {
                                    "xy": {
                                        "x": 0.25667436173377867,
                                        "y": 0.09771288539493972
                                    }
                                },
                                "speed": 0.5476190476190477
                            }
                        }
                    }
                ]
            },
            "speed": 0.7301587301587301
        });

        let scene: Scene = serde_json::from_value(payload).expect("Failed to deserialize scene");
        assert_eq!(scene.metadata.name, "Trick or treat");
        assert_eq!(scene.actions.len(), 1);
        let palette = scene.palette.expect("Expected palette");
        assert_eq!(palette.color.len(), 1);
        assert_eq!(palette.color_temperature.len(), 1);
        assert!(palette.effects_v2.is_some());
    }
}
