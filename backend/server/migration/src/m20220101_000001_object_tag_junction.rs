use crate::m20220101_000001_object::Object;
use crate::m20220101_000001_tag::Tag;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ObjectTagJunction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ObjectTagJunction::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ObjectTagJunction::ObjectId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ObjectTagJunction::Table, ObjectTagJunction::ObjectId)
                            .to(Object::Table, Object::Id),
                    )
                    .col(
                        ColumnDef::new(ObjectTagJunction::TagId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ObjectTagJunction::Table, ObjectTagJunction::TagId)
                            .to(Tag::Table, Tag::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ObjectTagJunction::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ObjectTagJunction {
    Table,
    Id,
    ObjectId,
    TagId,
}
