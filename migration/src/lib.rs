pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20250326_020445_create_user_table;
mod m20250427_200238_add_created_at__to_post;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20250326_020445_create_user_table::Migration),
            Box::new(m20250427_200238_add_created_at__to_post::Migration),
        ]
    }
}
