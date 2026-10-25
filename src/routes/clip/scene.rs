use serde_json::Value;

use bifrost_api::backend::BackendRequest;
use hue::api::{RType, Resource, ResourceLink, Scene, SceneActive, SceneStatus, SceneUpdate};

use crate::model::state::AuxData;
use crate::routes::clip::{ApiV2Result, V2Reply};
use crate::server::appstate::AppState;

pub async fn post_scene(state: &AppState, req: Value) -> ApiV2Result {
    let mut scene: Scene = serde_json::from_value(req)?;

    let mut lock = state.res.lock().await;

    let sid = lock.get_next_scene_id(&scene.group)?;

    let link_scene = RType::Scene.deterministic((scene.group.rid, sid));

    if scene.status.is_none() {
        scene.status = Some(SceneStatus {
            active: SceneActive::Inactive,
            last_recall: None,
        });
    }

    let backend = match scene.group.rtype {
        RType::Room => lock.room_backend(&scene.group).ok().flatten(),
        RType::Zone => lock.zone_backend(&scene.group).ok().flatten(),
        _ => None,
    };
    let mut aux = AuxData::new().with_index(sid);
    if let Some(backend) = &backend {
        aux = aux.with_backend(backend);
    }
    if let Ok(group_aux) = lock.aux_get(&scene.group) {
        if let Some(topic) = &group_aux.topic {
            aux = aux.with_topic(topic);
        }
    }

    lock.aux_set(&link_scene, aux);
    lock.add(&link_scene, Resource::Scene(scene.clone()))?;

    lock.backend_request(BackendRequest::SceneCreate(link_scene, sid, scene))?;

    drop(lock);

    V2Reply::ok(link_scene)
}

pub async fn put_scene(state: &AppState, rlink: ResourceLink, put: Value) -> ApiV2Result {
    let mut lock = state.res.lock().await;

    let upd: SceneUpdate = serde_json::from_value(put)?;

    lock.update::<Scene>(&rlink.rid, |scn| *scn += &upd)?;

    let _scene = lock.get::<Scene>(&rlink)?;

    lock.backend_request(BackendRequest::SceneUpdate(rlink, upd))?;
    drop(lock);

    V2Reply::ok(rlink)
}
