pub use sea_orm_migration::prelude::*;

mod m20220101_000001_item;
mod m20220101_000001_label;
mod m20220101_000001_object;

pub struct Migrator;

#[rustfmt::skip]
#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_label::Migration),
            Box::new(m20220101_000001_item::Migration),
            Box::new(m20220101_000001_object::Migration),
        ]
    }
}
