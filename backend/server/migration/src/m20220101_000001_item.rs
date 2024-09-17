use crate::m20220101_000001_grand_parent_label_junction::GrandParentLabelJunction;
use crate::m20220101_000001_label::Label;
use crate::m20220101_000001_parent_label_junction::ParentLabelJunction;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Item::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Item::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Item::LabelId)
                            .integer()
                            .unique_key()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Item::Table, Item::LabelId)
                            .to(Label::Table, Label::Id),
                    )
                    .col(ColumnDef::new(Item::ParentId).integer().not_null())
                    .col(
                        ColumnDef::new(Item::ParentLabelId)
                            .integer()
                            .unique_key()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Item::Table, Item::ParentLabelId)
                            .to(ParentLabelJunction::Table, ParentLabelJunction::Id),
                    )
                    .col(ColumnDef::new(Item::GrandParentId).integer().not_null())
                    .col(
                        ColumnDef::new(Item::GrandParentLabelId)
                            .integer()
                            .unique_key()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Item::Table, Item::GrandParentLabelId)
                            .to(
                                GrandParentLabelJunction::Table,
                                GrandParentLabelJunction::Id,
                            ),
                    )
                    .col(ColumnDef::new(Item::Name).string().not_null())
                    .col(ColumnDef::new(Item::ProductNumber).string().not_null())
                    .col(ColumnDef::new(Item::PhotoUrl).string().not_null())
                    .col(ColumnDef::new(Item::Record).string().not_null())
                    .col(ColumnDef::new(Item::Description).string().not_null())
                    .col(ColumnDef::new(Item::YearPurchased).integer())
                    .col(ColumnDef::new(Item::Connector).json().not_null())
                    .col(ColumnDef::new(Item::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Item::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Item::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Item {
    Table,
    Id,
    LabelId,
    ParentId,
    ParentLabelId,
    GrandParentId,
    GrandParentLabelId,
    Name,
    ProductNumber,
    PhotoUrl,
    Record,
    Description,
    YearPurchased,
    Connector,
    CreatedAt,
    UpdatedAt,
}
