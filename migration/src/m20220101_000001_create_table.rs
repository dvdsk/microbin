use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager.create_table(
            Table::create().if_not_exists()
                .table(Pasta::Table)
                .col(big_unsigned(Pasta::Id).not_null())
                .col(string(Pasta::Content).not_null())
                .col(string_null(Pasta::Filename))
                .col(big_unsigned_null(Pasta::FileSize))
                .col(string(Pasta::Extension).not_null())
                .col(boolean(Pasta::ReadOnly).not_null())
                .col(boolean(Pasta::Private).not_null())
                .col(integer(Pasta::Editable).not_null())
                .col(integer(Pasta::EncryptServer).not_null())
                .col(integer(Pasta::EncryptClient).not_null())
                .col(string_null(Pasta::EncryptedKey))
                .col(big_unsigned(Pasta::Created).not_null())
                .col(big_unsigned(Pasta::Expiration).not_null())
                .col(big_unsigned(Pasta::LastRead).not_null())
                .col(big_unsigned(Pasta::ReadCount).not_null())
                .col(big_unsigned(Pasta::BurnAfterReads).not_null())
                .col(string(Pasta::PastaType).not_null())
                .col(boolean(Pasta::HideReadCount).not_null().default(0))
                .primary_key(Index::create().col(Pasta::Id))
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Pasta::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Pasta {
    Table,
    #[sea_orm(iden = "id")]
    #[sea_orm(primary_key)]
    Id,
    #[sea_orm(iden = "content")]
    Content,
    #[sea_orm(iden = "file_name")]
    Filename,
    #[sea_orm(iden = "file_size")]
    FileSize,
    #[sea_orm(iden = "extension")]
    Extension,
    #[sea_orm(iden = "read_only")]
    ReadOnly,
    #[sea_orm(iden = "private")]
    Private,
    #[sea_orm(iden = "editable")]
    Editable,
    #[sea_orm(iden = "encrypt_server")]
    EncryptServer,
    #[sea_orm(iden = "encrypt_client")]
    EncryptClient,
    #[sea_orm(iden = "encrypted_key")]
    EncryptedKey,
    #[sea_orm(iden = "created")]
    Created,
    #[sea_orm(iden = "expiration")]
    Expiration,
    #[sea_orm(iden = "last_read")]
    LastRead,
    #[sea_orm(iden = "read_count")]
    ReadCount,
    #[sea_orm(iden = "burn_after_reads")]
    BurnAfterReads,
    #[sea_orm(iden = "pasta_type")]
    PastaType,
    #[sea_orm(iden = "hide_read_count")]
    HideReadCount,
}
