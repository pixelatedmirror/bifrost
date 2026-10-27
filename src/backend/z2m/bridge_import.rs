use std::collections::HashSet;

use maplit::btreeset;
use uuid::Uuid;

use hue::api::{
    BridgeHome, Button, ColorGamut, ColorTemperature, ContentConfiguration,
    ContentConfigurationOrder, ContentConfigurationOrientation, ContentConfigurationStatusType,
    DeviceArchetype, DeviceProductData, Entertainment, EntertainmentSegment, EntertainmentSegments,
    GamutType, GroupArchetype, GroupMetadata, GroupedLight, Light, LightColor, LightEffects,
    LightEffectsV2, LightMetadata, LightTimedEffect, LightTimedEffects, Metadata, MirekSchema,
    OrderType, OrientationType, RType, Resource, ResourceLink, Room, Scene, SceneActive,
    SceneMetadata, ScenePalette, SceneRecall, SceneStatus, Stub, Taurus, ZigbeeConnectivity,
    ZigbeeConnectivityStatus, Zone,
};
use hue::devicedb::gradient_product_data;
use hue::scene_icons;
use hue::xy::XY;
use z2m::api::{ExposeLight, ExposeSwitch};
use z2m::convert::{
    ExtractColorTemperature, ExtractDeviceProductData, ExtractDimming, ExtractLightColor,
    ExtractLightGradient,
};

use crate::backend::z2m::Z2mBackend;
use crate::backend::z2m::button::Z2mButtonData;
use crate::error::{ApiError, ApiResult};
use crate::model::state::AuxData;
use crate::resource::Resources;

impl Z2mBackend {
    pub async fn add_light(
        &mut self,
        apidev: &z2m::api::Device,
        expose: &ExposeLight,
    ) -> ApiResult<()> {
        let name = &apidev.friendly_name;

        let link_device = RType::Device.deterministic(&apidev.ieee_address);
        let link_light = RType::Light.deterministic(&apidev.ieee_address);
        let link_enttm = RType::Entertainment.deterministic(&apidev.ieee_address);
        let link_taurus = RType::Taurus.deterministic(&apidev.ieee_address);
        let link_zigcon = RType::ZigbeeConnectivity.deterministic(&apidev.ieee_address);

        let product_data = DeviceProductData::guess_from_device(apidev);
        let metadata = LightMetadata::new(product_data.product_archetype.clone(), name);

        let effects =
            apidev.manufacturer.as_deref() == Some(DeviceProductData::SIGNIFY_MANUFACTURER_NAME);
        let gradient = apidev.expose_gradient();
        let gradient_product_data = gradient_product_data(&product_data.model_id);

        let dev = hue::api::Device {
            product_data,
            metadata: metadata.clone().into(),
            services: btreeset![link_zigcon, link_light, link_enttm, link_taurus],
            identify: Some(Stub),
            usertest: None,
        };

        if let Some(old_name) = self.rmap.get(&link_light).cloned() {
            if old_name != *name {
                log::info!(
                    "[{}] Light {:?} renamed: [{old_name}] -> [{name}]",
                    self.name,
                    apidev.ieee_address
                );
                self.map.remove(&old_name);
            }
        }

        self.map.insert(name.clone(), link_light);
        self.rmap.insert(link_device, name.clone());
        self.rmap.insert(link_light, name.clone());

        let mut light = Light::new(link_device, metadata);

        light.dimming = expose
            .feature("brightness")
            .and_then(ExtractDimming::extract_from_expose);
        log::trace!("Detected dimming: {:?}", &light.dimming);

        light.color_temperature = expose
            .feature("color_temp")
            .and_then(ExtractColorTemperature::extract_from_expose);
        log::trace!("Detected color temperature: {:?}", &light.color_temperature);

        light.color = expose
            .feature("color_xy")
            .and_then(ExtractLightColor::extract_from_expose);
        log::trace!("Detected color: {:?}", &light.color);

        light.gradient = gradient.and_then(|gradient| {
            ExtractLightGradient::extract_from_expose(gradient, &gradient_product_data)
        });
        log::trace!("Detected gradient support: {:?}", &light.gradient);

        if effects {
            log::trace!("Detected Hue light: enabling effects");
            light.effects = Some(LightEffects::all());
            light.effects_v2 = Some(LightEffectsV2::all());
            light.timed_effects = Some(LightTimedEffects {
                status_values: Vec::from(LightTimedEffect::ALL),
                status: LightTimedEffect::NoEffect,
                effect_values: Vec::from(LightTimedEffect::ALL),
            });
        }

        if gradient.is_some() {
            light.content_configuration = Some(ContentConfiguration {
                orientation: Some(ContentConfigurationOrientation {
                    configurable: true,
                    orientation: OrientationType::Horizontal,
                    status: ContentConfigurationStatusType::Set,
                }),
                order: Some(ContentConfigurationOrder {
                    configurable: true,
                    order: OrderType::Forward,
                    status: ContentConfigurationStatusType::Set,
                }),
            });
        }

        let segments = if gradient.is_some() {
            EntertainmentSegments {
                configurable: false,
                max_segments: 10,
                segments: gradient_product_data.entertainment_segments.to_vec(),
            }
        } else {
            EntertainmentSegments {
                configurable: false,
                max_segments: 1,
                segments: vec![EntertainmentSegment {
                    start: 0,
                    length: 1,
                }],
            }
        };

        // FIXME: This should be feature-detected, not always enabled
        let enttm = Entertainment {
            equalizer: true,
            owner: link_device,
            proxy: true,
            renderer: true,
            max_streams: None,
            renderer_reference: Some(link_light),
            segments: Some(segments),
        };

        // FIXME: The Taurus objects are seen on Hue Entertainment devices on a
        // real hue bridge, but nobody knows what it does. Some clients seem to
        // want them present, though.
        let taurus = Taurus {
            capabilities: vec![
                "sensor".to_string(),
                "collector".to_string(),
                "sync".to_string(),
            ],
            owner: link_device,
        };

        let zigcon = ZigbeeConnectivity {
            channel: None,
            extended_pan_id: None,
            mac_address: apidev.ieee_address.as_mac(),
            owner: link_device,
            status: ZigbeeConnectivityStatus::Connected,
        };

        let mut res = self.state.lock().await;
        res.aux_set(
            &link_light,
            AuxData::new().with_topic(name).with_backend(&self.name),
        );
        if res.contains_id(&link_device.rid) {
            let _ = res.update::<hue::api::Device>(&link_device.rid, |device| {
                if device.metadata.name != *name {
                    device.metadata.name.clone_from(name);
                }
            });
        } else {
            res.add(&link_device, Resource::Device(dev))?;
        }

        if res.contains_id(&link_light.rid) {
            let _ = res.update::<Light>(&link_light.rid, |l| {
                if l.metadata.name != *name {
                    l.metadata.name.clone_from(name);
                }
            });
        } else {
            res.add(&link_light, Resource::Light(Box::new(light)))?;
        }

        res.add(&link_enttm, Resource::Entertainment(enttm))?;
        res.add(&link_taurus, Resource::Taurus(taurus))?;
        res.add(&link_zigcon, Resource::ZigbeeConnectivity(zigcon))?;
        drop(res);

        Ok(())
    }

    pub async fn add_switch_plug(
        &mut self,
        apidev: &z2m::api::Device,
        _expose: &ExposeSwitch,
    ) -> ApiResult<()> {
        let name = &apidev.friendly_name;

        let link_device = RType::Device.deterministic(&apidev.ieee_address);
        let link_light = RType::Light.deterministic(&apidev.ieee_address);
        let link_zigcon = RType::ZigbeeConnectivity.deterministic(&apidev.ieee_address);

        let mut product_data = DeviceProductData::guess_from_device(apidev);
        if product_data.product_archetype == DeviceArchetype::UnknownArchetype {
            product_data.product_archetype = DeviceArchetype::Plug;
        }
        if product_data.product_name == "<unknown>" {
            if let Some(def) = &apidev.definition {
                product_data.product_name.clone_from(&def.description);
            } else {
                product_data.product_name = "Smart plug".to_string();
            }
        }

        let metadata = LightMetadata::new(product_data.product_archetype.clone(), name);

        let dev = hue::api::Device {
            product_data,
            metadata: metadata.clone().into(),
            services: btreeset![link_zigcon, link_light],
            identify: Some(Stub),
            usertest: None,
        };

        if let Some(old_name) = self.rmap.get(&link_light).cloned() {
            if old_name != *name {
                log::info!(
                    "[{}] Switch/plug {:?} renamed: [{old_name}] -> [{name}]",
                    self.name,
                    apidev.ieee_address
                );
                self.map.remove(&old_name);
            }
        }

        self.map.insert(name.clone(), link_light);
        self.rmap.insert(link_device, name.clone());
        self.rmap.insert(link_light, name.clone());

        let mut light = Light::new(link_device, metadata);
        light.dimming = None;
        light.dimming_delta = None;
        light.color = None;
        light.color_temperature = None;
        light.color_temperature_delta = None;
        light.powerup = None;

        let zigcon = ZigbeeConnectivity {
            channel: None,
            extended_pan_id: None,
            mac_address: apidev.ieee_address.as_mac(),
            owner: link_device,
            status: ZigbeeConnectivityStatus::Connected,
        };

        let mut res = self.state.lock().await;
        res.aux_set(
            &link_light,
            AuxData::new().with_topic(name).with_backend(&self.name),
        );
        if res.contains_id(&link_device.rid) {
            let _ = res.update::<hue::api::Device>(&link_device.rid, |device| {
                if device.metadata.name != *name {
                    device.metadata.name.clone_from(name);
                }
            });
        } else {
            res.add(&link_device, Resource::Device(dev))?;
        }

        if res.contains_id(&link_light.rid) {
            let _ = res.update::<Light>(&link_light.rid, |l| {
                if l.metadata.name != *name {
                    l.metadata.name.clone_from(name);
                }
            });
        } else {
            res.add(&link_light, Resource::Light(Box::new(light)))?;
        }

        res.add(&link_zigcon, Resource::ZigbeeConnectivity(zigcon))?;
        drop(res);

        Ok(())
    }

    pub async fn add_switch(&mut self, apidev: &z2m::api::Device) -> ApiResult<Option<()>> {
        let Some(model_id) = &apidev.model_id else {
            return Ok(None);
        };

        let Some(button_device) = Z2mButtonData::from_model_id(model_id) else {
            return Ok(None);
        };

        let name = &apidev.friendly_name;
        let link_device = RType::Device.deterministic(&apidev.ieee_address);

        if let Some(old_name) = self.rmap.get(&link_device).cloned() {
            if old_name != *name {
                log::info!(
                    "[{}] Switch {:?} renamed: [{old_name}] -> [{name}]",
                    self.name,
                    apidev.ieee_address
                );
                self.map.remove(&old_name);
            }
        }

        self.map.insert(name.to_owned(), link_device);
        self.rmap.insert(link_device, name.to_owned());

        let link_zbc = RType::ZigbeeConnectivity.deterministic(&apidev.ieee_address);

        // missing services:
        // device_power
        // device_software_update

        let mut services = btreeset![link_zbc];
        let mut buttons = vec![];

        for button in &button_device.buttons {
            let link_button = RType::Button.deterministic((&apidev.ieee_address, &button.name));
            let button = Button {
                owner: link_device,
                metadata: button.metadata.clone(),
                button: button.data.clone(),
            };
            buttons.push((link_button, button));
            services.insert(link_button);
        }

        let dev = hue::api::Device {
            product_data: DeviceProductData::guess_from_device(apidev),
            metadata: Metadata::new(DeviceArchetype::UnknownArchetype, name),
            services,
            identify: None,
            usertest: None,
        };

        let mut res = self.state.lock().await;

        let zigcon = ZigbeeConnectivity {
            channel: None,
            extended_pan_id: None,
            mac_address: apidev.ieee_address.as_mac(),
            owner: link_device,
            status: ZigbeeConnectivityStatus::Connected,
        };

        let mut aux_data = AuxData::new().with_topic(name).with_backend(&self.name);
        if let Some(model_id) = &apidev.model_id {
            // needed to look up button mappings when handling actions
            aux_data = aux_data.with_model_id(model_id);
        }
        res.aux_set(&link_device, aux_data);
        if res.contains_id(&link_device.rid) {
            let _ = res.update::<hue::api::Device>(&link_device.rid, |device| {
                if device.metadata.name != *name {
                    device.metadata.name.clone_from(name);
                }
            });
        } else {
            res.add(&link_device, Resource::Device(dev))?;
        }
        for (link_button, button) in buttons {
            res.add(&link_button, Resource::Button(button))?;
        }
        res.add(&link_zbc, Resource::ZigbeeConnectivity(zigcon))?;
        drop(res);

        Ok(Some(()))
    }

    #[allow(clippy::too_many_lines)]
    pub async fn add_group(&mut self, grp: &z2m::api::Group) -> ApiResult<()> {
        let topic = grp.friendly_name.clone();
        let room_name;
        if let Some(ref prefix) = self.server.group_prefix {
            if let Some(name) = grp.friendly_name.strip_prefix(prefix) {
                room_name = name;
            } else {
                log::debug!(
                    "[{}] Ignoring room outside our prefix: {}",
                    self.name,
                    grp.friendly_name
                );
                return Ok(());
            }
        } else {
            room_name = &grp.friendly_name;
        }

        let mut lock = self.state.lock().await;
        let link_glight = self
            .find_grouped_light(&lock, grp.id, &topic)?
            .unwrap_or_else(|| RType::GroupedLight.deterministic((&self.name, grp.id)));

        let mut glight = if let Ok(glight) = lock.get::<GroupedLight>(&link_glight).cloned() {
            glight
        } else {
            let link_room = RType::Room.deterministic(link_glight.rid);
            let glight = GroupedLight::new(link_room);
            lock.add(&link_glight, Resource::GroupedLight(glight.clone()))?;
            glight
        };
        let mut needs_update = false;
        if glight.color_temperature.is_none() {
            glight.color_temperature = Some(ColorTemperature {
                mirek: Some(366),
                mirek_schema: MirekSchema::DEFAULT,
                mirek_valid: true,
            });
            needs_update = true;
        }
        if glight.color.is_none() {
            glight.color = Some(LightColor {
                gamut: Some(ColorGamut::GAMUT_C),
                gamut_type: GamutType::C,
                xy: XY {
                    x: 0.3151,
                    y: 0.3224,
                },
            });
            needs_update = true;
        }
        if needs_update {
            let ct = glight.color_temperature.clone();
            let col = glight.color.clone();
            lock.update(&link_glight.rid, |g: &mut GroupedLight| {
                g.color_temperature = ct;
                g.color = col;
            })?;
        }
        let owner_link = glight.owner;
        self.set_group_aux(&mut lock, link_glight, grp.id, Some(&topic));
        drop(lock);

        let children = grp
            .members
            .iter()
            .map(|f| match owner_link.rtype {
                RType::Zone => RType::Light.deterministic(&f.ieee_address),
                _ => RType::Device.deterministic(&f.ieee_address),
            })
            .collect();

        let mut res = self.state.lock().await;

        let mut scenes_new = HashSet::new();

        for scn in &grp.scenes {
            let scene = Scene {
                actions: vec![],
                auto_dynamic: false,
                group: owner_link,
                metadata: SceneMetadata {
                    appdata: None,
                    image: guess_scene_icon(&scn.name),
                    name: scn.name.clone(),
                },
                palette: Some(ScenePalette::default()),
                speed: 0.5,
                recall: SceneRecall {
                    action: None,
                    dimming: None,
                    duration: None,
                },
                status: Some(SceneStatus {
                    active: SceneActive::Inactive,
                    last_recall: None,
                }),
            };

            let link_scene = RType::Scene.deterministic((owner_link.rid, scn.id));

            res.aux_set(
                &link_scene,
                AuxData::new()
                    .with_topic(&topic)
                    .with_backend(&self.name)
                    .with_index(scn.id),
            );

            scenes_new.insert(link_scene.rid);
            res.add(&link_scene, Resource::Scene(scene))?;
        }

        let group_metadata = match res.get_resource(&owner_link) {
            Ok(group) => match group.obj {
                Resource::Room(room) => Some(room.metadata),
                Resource::Zone(zone) => Some(zone.metadata),
                _ => None,
            },
            Err(_) => None,
        };

        if let Some(group_metadata) = group_metadata.as_ref() {
            log::info!(
                "[{}] {owner_link:?} ({}) known, updating..",
                self.name,
                group_metadata.name
            );

            let scenes_old: HashSet<Uuid> =
                HashSet::from_iter(res.get_scenes_for_group(&owner_link.rid));

            log::trace!("[{}] old scenes: {scenes_old:?}", self.name);
            log::trace!("[{}] new scenes: {scenes_new:?}", self.name);
            let gone = scenes_old.difference(&scenes_new);
            log::trace!("[{}]   deleted: {gone:?}", self.name);
            for uuid in gone {
                log::debug!(
                    "[{}] Deleting orphaned {uuid:?} in {owner_link:?}",
                    self.name
                );
                let _ = res.delete(&RType::Scene.link_to(*uuid));
            }
        } else {
            log::debug!(
                "[{}] {owner_link:?} ({}) is new, adding..",
                self.name,
                room_name
            );
        }

        let mut metadata = GroupMetadata::new(GroupArchetype::Home, room_name);
        if let Some(room_conf) = self.config.rooms.get(&topic) {
            if let Some(name) = &room_conf.name {
                metadata.name.clone_from(name);
            }
            if let Some(icon) = &room_conf.icon {
                metadata.archetype = *icon;
            }
        }

        if let Some(old_topic) = self.rmap.get(&link_glight).cloned() {
            if old_topic != topic {
                log::info!(
                    "[{}] Group {:?} renamed: [{old_topic}] -> [{topic}]",
                    self.name,
                    grp.id
                );
                self.map.remove(&old_topic);
            }
        }

        self.map.insert(topic.clone(), link_glight);
        self.rmap.insert(link_glight, topic.clone());
        self.rmap.insert(owner_link, topic.clone());

        if owner_link.rtype == RType::Room {
            for id in &res.get_resource_ids_by_type(RType::BridgeHome) {
                res.update(id, |bh: &mut BridgeHome| {
                    bh.children.insert(owner_link);
                })?;
            }
        }

        match owner_link.rtype {
            RType::Room => {
                let room = Room {
                    children,
                    metadata: metadata.clone(),
                    services: btreeset![link_glight],
                };
                if res.get::<Room>(&owner_link).is_ok() {
                    res.update::<Room>(&owner_link.rid, |r| {
                        r.services = room.services;
                        r.children = room.children;
                        if r.metadata.name != metadata.name {
                            r.metadata.name.clone_from(&metadata.name);
                        }
                    })?;
                } else {
                    res.add(&owner_link, Resource::Room(room))?;
                }
            }
            RType::Zone => {
                let zone = Zone {
                    children,
                    metadata: metadata.clone(),
                    services: btreeset![link_glight],
                };
                if res.get::<Zone>(&owner_link).is_ok() {
                    res.update::<Zone>(&owner_link.rid, |z| {
                        z.services = zone.services;
                        z.children = zone.children;
                        if z.metadata.name != metadata.name {
                            z.metadata.name.clone_from(&metadata.name);
                        }
                    })?;
                } else {
                    res.add(&owner_link, Resource::Zone(zone))?;
                }
            }
            _ => {}
        }

        drop(res);

        Ok(())
    }

    fn find_grouped_light(
        &self,
        resources: &Resources,
        group_id: u32,
        friendly_name: &str,
    ) -> ApiResult<Option<ResourceLink>> {
        for grouped_light in resources.get_resources_by_type(RType::GroupedLight) {
            let link = grouped_light.link();
            match resources.aux_get(&link) {
                Ok(aux) => {
                    if let Some(aux_backend) = &aux.backend
                        && aux_backend == &self.name
                        && aux.index == Some(group_id)
                    {
                        return Ok(Some(link));
                    }
                }
                Err(ApiError::AuxNotFound(_)) => {
                    // match rooms that haven't been migrated yet
                    let link_room = RType::Room.deterministic(friendly_name);
                    let link_glight = RType::GroupedLight.deterministic((link_room.rid, group_id));
                    if link_glight == link {
                        return Ok(Some(link_glight));
                    }
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Ok(None)
    }
}

#[allow(clippy::match_same_arms)]
fn guess_scene_icon(name: &str) -> Option<ResourceLink> {
    let icon = match name {
        /* Built-in names */
        "Bright" => scene_icons::BRIGHT,
        "Relax" => scene_icons::RELAX,
        "Night Light" => scene_icons::NIGHT_LIGHT,
        "Rest" => scene_icons::REST,
        "Concentrate" => scene_icons::CONCENTRATE,
        "Dimmed" => scene_icons::DIMMED,
        "Energize" => scene_icons::ENERGIZE,
        "Read" => scene_icons::READ,
        "Cool Bright" => scene_icons::COOL_BRIGHT,

        /* Aliases */
        "Night" => scene_icons::NIGHT_LIGHT,
        "Cool" => scene_icons::COOL_BRIGHT,
        "Dim" => scene_icons::DIMMED,

        _ => return None,
    };

    Some(ResourceLink {
        rid: icon,
        rtype: RType::PublicImage,
    })
}
