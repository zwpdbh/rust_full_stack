use super::_entities::storage_types::{ActiveModel, Entity};
use sea_orm::entity::prelude::*;
pub type StorageTypes = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)

    // async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    // where
    //     C: ConnectionTrait,
    // {
    //     if !insert && self.updated_at.is_unchanged() {
    //         let mut this = self;
    //         this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
    //         Ok(this)
    //     } else {
    //         Ok(self)
    //     }
    // }
}
