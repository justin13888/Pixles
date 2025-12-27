use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. Create Owners Table
        manager
            .create_table(
                Table::create()
                    .table(Owners::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Owners::Id).string().not_null().primary_key())
                    .col(
                        ColumnDef::new(Owners::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // 2. Create Owner Members Table
        manager
            .create_table(
                Table::create()
                    .table(OwnerMembers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OwnerMembers::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(OwnerMembers::OwnerId).string().not_null())
                    // User ID is UUID string in Users table?
                    // initialize.rs uses char_len(21) for Users.Id (nanoid-like).
                    // But OwnerMembers.UserId should probably match.
                    // Let's check Users.Id type in initialize.rs -> char_len(21).
                    // So we should use string() or char_len(21).
                    .col(ColumnDef::new(OwnerMembers::UserId).string().not_null())
                    .col(
                        ColumnDef::new(OwnerMembers::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-owner_members-owner_id")
                            .from(OwnerMembers::Table, OwnerMembers::OwnerId)
                            .to(Owners::Table, Owners::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-owner_members-user_id")
                            .from(OwnerMembers::Table, OwnerMembers::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx-owner_members-owner-user")
                            .table(OwnerMembers::Table)
                            .col(OwnerMembers::OwnerId)
                            .col(OwnerMembers::UserId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        // 3. Create Album Shares Table
        manager
            .create_table(
                Table::create()
                    .table(AlbumShares::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AlbumShares::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AlbumShares::AlbumId).string().not_null())
                    .col(ColumnDef::new(AlbumShares::UserId).string().not_null())
                    .col(ColumnDef::new(AlbumShares::Permission).string().not_null())
                    .col(
                        ColumnDef::new(AlbumShares::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-album_shares-album_id")
                            .from(AlbumShares::Table, AlbumShares::AlbumId)
                            .to(Albums::Table, Albums::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-album_shares-user_id")
                            .from(AlbumShares::Table, AlbumShares::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 4. Alter Albums Table (OwnerId FK change)
        // Drop old FK to Users
        manager
            .alter_table(
                Table::alter()
                    .table(Albums::Table)
                    .drop_foreign_key(Alias::new("fk_albums_owner_id")) // Name from initialize.rs
                    .to_owned(),
            )
            .await?;

        // Add new FK to Owners
        manager
            .alter_table(
                Table::alter()
                    .table(Albums::Table)
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk-albums-owner_id-v2")
                            .from_tbl(Albums::Table)
                            .from_col(Albums::OwnerId)
                            .to_tbl(Owners::Table)
                            .to_col(Owners::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 5. Alter Assets Table
        // Add columns and FKs
        manager
            .alter_table(
                Table::alter()
                    .table(Assets::Table)
                    .add_column(
                        ColumnDef::new(Assets::AssetType)
                            .string()
                            .not_null()
                            .default("ph"),
                    ) // Default needed for existing rows
                    .add_column(
                        ColumnDef::new(Assets::OriginalFilename)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .add_column(
                        ColumnDef::new(Assets::FileSize)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .add_column(
                        ColumnDef::new(Assets::FileHash)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .add_column(
                        ColumnDef::new(Assets::ContentType)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk-assets-owner_id")
                            .from_tbl(Assets::Table)
                            .from_col(Assets::OwnerId)
                            .to_tbl(Owners::Table)
                            .to_col(Owners::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk-assets-album_id")
                            .from_tbl(Assets::Table)
                            .from_col(Assets::AlbumId)
                            .to_tbl(Albums::Table)
                            .to_col(Albums::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Reverse changes

        // Assets: remove columns and FKs
        manager
            .alter_table(
                Table::alter()
                    .table(Assets::Table)
                    .drop_foreign_key(Alias::new("fk-assets-album_id"))
                    .drop_foreign_key(Alias::new("fk-assets-owner_id"))
                    .drop_column(Assets::AssetType)
                    .drop_column(Assets::OriginalFilename)
                    .drop_column(Assets::FileSize)
                    .drop_column(Assets::FileHash)
                    .drop_column(Assets::ContentType)
                    .to_owned(),
            )
            .await?;

        // Albums: revert FK
        manager
            .alter_table(
                Table::alter()
                    .table(Albums::Table)
                    .drop_foreign_key(Alias::new("fk-albums-owner_id-v2"))
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_albums_owner_id")
                            .from_tbl(Albums::Table)
                            .from_col(Albums::OwnerId)
                            .to_tbl(Users::Table)
                            .to_col(Users::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(AlbumShares::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(OwnerMembers::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Owners::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Owners {
    Table,
    Id,
    CreatedAt,
}

#[derive(DeriveIden)]
enum OwnerMembers {
    Table,
    Id,
    OwnerId,
    UserId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Assets {
    Table,
    Id,
    OwnerId,
    AlbumId,
    AssetType,
    OriginalFilename,
    FileSize,
    FileHash,
    ContentType,
}

#[derive(DeriveIden)]
enum Albums {
    Table,
    Id,
    OwnerId,
}

#[derive(DeriveIden)]
enum AlbumShares {
    Table,
    Id,
    AlbumId,
    UserId,
    Permission,
    CreatedAt,
}
