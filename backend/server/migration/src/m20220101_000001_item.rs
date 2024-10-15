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
                        ColumnDef::new(Item::VisibleId)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Item::Table, Item::VisibleId)
                            .to(Label::Table, Label::VisibleId),
                    )
                    .col(ColumnDef::new(Item::Name).string().not_null())
                    .col(ColumnDef::new(Item::ProductNumber).string().not_null())
                    .col(
                        ColumnDef::new(Item::PhotoUrl)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
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
    VisibleId,
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
