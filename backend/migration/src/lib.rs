#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;

mod m20241113_065545_posts;
mod m20241113_132948_files;
mod m20241216_123238_acstor_storage_types;
mod m20241223_095243_acstor_features;
mod m20241223_100314_storage_type_storage_feature;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // inject-below (do not remove this comment)
            Box::new(m20241113_065545_posts::Migration),
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20241113_132948_files::Migration),
            Box::new(m20241216_123238_acstor_storage_types::Migration),
            Box::new(m20241223_095243_acstor_features::Migration),
            Box::new(m20241223_100314_storage_type_storage_feature::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
