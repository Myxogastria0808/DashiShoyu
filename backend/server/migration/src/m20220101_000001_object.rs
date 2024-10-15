use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Object::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Object::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Object::Name).string().not_null())
                    .col(
                        ColumnDef::new(Object::PhotoUrl)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Object::MimeType).string().not_null())
                    .col(ColumnDef::new(Object::License).string().not_null())
                    .col(ColumnDef::new(Object::Description).string().not_null())
                    .col(ColumnDef::new(Object::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Object::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Object::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Object {
    Table,
    Id,
    Name,
    PhotoUrl,
    MimeType,
    License,
    Description,
    CreatedAt,
    UpdatedAt,
}
