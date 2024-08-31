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
                            .unique_key()
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
                    .col(ColumnDef::new(Item::ParentId).integer().not_null())
                    .col(ColumnDef::new(Item::ParentVisibleId).string().not_null())
                    .col(ColumnDef::new(Item::GrandParentId).integer().not_null())
                    .col(
                        ColumnDef::new(Item::GrandParentVisibleId)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Item::Name).string().not_null())
                    .col(ColumnDef::new(Item::ProductNumber).string().not_null())
                    .col(ColumnDef::new(Item::PhotoUrl).string().not_null())
                    .col(ColumnDef::new(Item::Record).string().not_null())
                    .col(ColumnDef::new(Item::Color).string().not_null())
                    .col(ColumnDef::new(Item::Description).string().not_null())
                    .col(ColumnDef::new(Item::YearPurchased).integer())
                    .col(ColumnDef::new(Item::IsDiscarded).boolean().not_null())
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
    ParentId,
    ParentVisibleId,
    GrandParentId,
    GrandParentVisibleId,
    Name,
    ProductNumber,
    PhotoUrl,
    Record,
    Color,
    Description,
    YearPurchased,
    IsDiscarded,
    Connector,
    CreatedAt,
    UpdatedAt,
}
