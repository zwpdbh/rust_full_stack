#![allow(unused)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::models::{
    _entities::{acstor_features, acstor_features_to_storage_types, storage_types},
    acstor_features::AcstorFeatures,
};
use axum::debug_handler;
use common::{CreateAcstorFeatureFormParams, CreatedAcstorFeature};
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct UpdateParams {
    name: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FeatureResponse {
    id: i32,
    name: String,
    description: Option<String>,
}

// region:      --- CRUD for acstor_features ---

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    let res = AcstorFeatures::find().all(&ctx.db).await?;
    format::json(res)
}

pub async fn add(
    State(ctx): State<AppContext>,
    Json(form): Json<CreateAcstorFeatureFormParams>,
) -> Result<Response> {
    // start a transaction
    let txn = ctx.db.begin().await?;

    let new_feature = acstor_features::ActiveModel {
        name: Set(form.name),
        description: Set(form.description),
        ..Default::default()
    };
    let feature = new_feature.insert(&txn).await?;

    // create associations with storage types
    for storage_type_id in form.storage_type_ids {
        let association = acstor_features_to_storage_types::ActiveModel {
            feature_id: Set(feature.id),
            storage_type_id: Set(storage_type_id),
            ..Default::default()
        };
        association.insert(&txn).await?;
    }
    // commit the transaction
    txn.commit().await?;

    // fetch associated storage types
    let storage_types = feature
        .find_related(storage_types::Entity)
        .all(&ctx.db)
        .await?;

    let response = CreatedAcstorFeature {
        name: feature.name,
        description: feature.description,
        storage_type_ids: storage_types
            .iter()
            .map(|storage_type| storage_type.id)
            .collect(),
    };

    format::json(response)
}

// endregion:   --- CRUD for acstor_features ---
