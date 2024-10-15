use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Label::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Label::VisibleId)
                            .string()
                            .primary_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Label::Color).string().not_null())
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Label::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Label {
    Table,
    VisibleId,
    Color,
}
