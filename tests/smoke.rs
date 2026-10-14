use serde_json::Value;
use std::collections::BTreeMap;

use crate::common::{TestBridge, TestResult};

pub mod common;

const fn z2m_state() -> BTreeMap<String, Value> {
    BTreeMap::new()
}

#[tokio::test]
async fn read_config() -> TestResult<()> {
    let test_bridge = TestBridge::start(z2m_state()).await?;

    let response = reqwest::get(test_bridge.hue_url.join("/api/config").unwrap())
        .await
        .unwrap();

    assert!(response.status().is_success());
    Ok(())
}

#[tokio::test]
async fn unsupported_device_update_does_not_error() -> TestResult<()> {
    common::init();
    let mut switch = common::fixture::hue::dimmer_switch(&common::fixture::FixtureDevice {
        id: 2,
        friendly_name: "unsupported_switch",
    });
    switch.bridge_device["model_id"] = serde_json::json!("UnsupportedModel123");
    if let Some(def) = switch.bridge_device.get_mut("definition") {
        def["model"] = serde_json::json!("UnsupportedModel123");
    }

    let light_fixture = common::fixture::misc::dimmer_light(&common::fixture::FixtureDevice {
        id: 1,
        friendly_name: "test_light",
    });

    let mut z2m_state = BTreeMap::new();
    z2m_state.insert(
        "bridge/devices".to_string(),
        serde_json::json!([switch.bridge_device, light_fixture.bridge_device]),
    );
    z2m_state.insert("bridge/groups".to_string(), serde_json::json!([]));
    z2m_state.insert(
        "unsupported_switch".to_string(),
        serde_json::json!({
            "battery": 40,
            "linkquality": 127
        }),
    );
    z2m_state.insert(
        "test_light".to_string(),
        serde_json::json!({
            "state": "ON",
            "brightness": 254
        }),
    );

    let mut test_bridge = TestBridge::start(z2m_state).await?;
    let _ = test_bridge
        .wait_for_light(common::fixture::FixtureDevice {
            id: 1,
            friendly_name: "test_light",
        })
        .await?;

    // Publishing state update to the unsupported device should not cause errors
    test_bridge.z2m.publish_topic(
        "unsupported_switch".to_string(),
        serde_json::json!({
            "battery": 39,
            "linkquality": 120
        }),
    )?;

    // Publishing an action on <topic>/action should also be safely ignored
    test_bridge.z2m.publish_topic(
        "unsupported_switch/action".to_string(),
        serde_json::json!("press_1"),
    )?;

    let response = reqwest::get(test_bridge.hue_url.join("/api/config").unwrap())
        .await
        .unwrap();
    assert!(response.status().is_success());

    Ok(())
}
