use crate::m20220101_000001_label::Label;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ParentLabelJunction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ParentLabelJunction::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ParentLabelJunction::LabelId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ParentLabelJunction::Table, ParentLabelJunction::LabelId)
                            .to(Label::Table, Label::Id),
                    )
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ParentLabelJunction::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ParentLabelJunction {
    Table,
    Id,
    LabelId,
}
