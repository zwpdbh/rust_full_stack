#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;

use crate::models::storage_types::StorageTypes;

#[debug_handler]
pub async fn index(State(_ctx): State<AppContext>) -> Result<Response> {
    let res = StorageTypes::find().all(&_ctx.db).await?;
    format::json(res)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/storage_types/")
        .add("/", get(index))
}
