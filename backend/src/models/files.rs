use sea_orm::entity::prelude::*;
use super::_entities::files::{ActiveModel, Entity};
pub type Files = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
