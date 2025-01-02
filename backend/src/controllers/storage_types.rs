#![allow(unused)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::models::_entities::storage_types::{ActiveModel, Entity, Model};
use axum::debug_handler;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::storage_types::StorageTypes;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StorageTypeFormParams {
    pub name: String,
    pub description: Option<String>,
}

impl StorageTypeFormParams {
    fn update(&self, item: &mut ActiveModel) {
        item.name = Set(self.name.clone());
        item.description = Set(self.description.clone());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

// region:      --- CRUD for StorageTypes ---
#[debug_handler]
pub async fn list(State(_ctx): State<AppContext>) -> Result<Response> {
    let res = StorageTypes::find().all(&_ctx.db).await?;
    format::json(res)
}

#[debug_handler]
pub async fn add(
    State(ctx): State<AppContext>,
    Json(params): Json<StorageTypeFormParams>,
) -> Result<Response> {
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    let item = item.insert(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<StorageTypeFormParams>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    let mut item = item.into_active_model();
    params.update(&mut item);
    let item = item.update(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

#[debug_handler]
pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(load_item(&ctx, id).await?)
}
// endregion:   --- CRUD for StorageTypes ---

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/test_coverage")
        .add("/storage_types", get(list))
        .add("/storage_type", post(add))
        .add("/storage_type:id", get(get_one))
        .add("/storage_type:id", delete(remove))
        .add("/storage_type:id", put(update))
        .add("/storage_type:id", patch(update))
}
