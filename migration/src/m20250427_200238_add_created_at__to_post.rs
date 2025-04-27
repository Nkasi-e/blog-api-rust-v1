use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .alter_table(Table::alter().table(Post::Table)
            .add_column(ColumnDef::new(Post::CreatedAt)
            .date_time().not_null().default(Expr::current_timestamp()))
            .to_owned())
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
        .alter_table(
            Table::alter()
                .table(Post::Table)
                .drop_column(Post::CreatedAt)
                .to_owned()
        )
        .await
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    CreatedAt
}
