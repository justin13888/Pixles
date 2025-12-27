use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Install required extensions
        db.execute_unprepared("CREATE EXTENSION IF NOT EXISTS pg_trgm;")
            .await?;
        db.execute_unprepared("CREATE EXTENSION IF NOT EXISTS btree_gin;")
            .await?;

        // ========================================
        // 1. Create Owners Table
        // ========================================
        manager
            .create_table(
                Table::create()
                    .table(Owners::Table)
                    .if_not_exists()
                    .col(char_len(Owners::Id, 21).primary_key())
                    .col(
                        timestamp_with_time_zone(Owners::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // ========================================
        // 2. Create Users Table
        // ========================================
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(char_len(Users::Id, 21).primary_key())
                    .col(string_len(Users::Username, 64))
                    .col(string(Users::Name))
                    .col(string_len(Users::Email, 255))
                    .col(string_len_null(Users::ProfileImageUrl, 255))
                    .col(boolean(Users::AccountVerified).default(false))
                    .col(boolean(Users::NeedsOnboarding).default(true))
                    .col(string(Users::PasswordHash))
                    .col(string_null(Users::PasswordResetToken).unique_key())
                    .col(timestamp_with_time_zone_null(Users::PasswordResetExpiresAt))
                    .col(timestamp_with_time_zone_null(Users::LastLoginAt))
                    .col(integer(Users::FailedLoginAttempts).default(0))
                    .col(boolean(Users::IsAdmin).default(false))
                    .col(
                        timestamp_with_time_zone(Users::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Users::ModifiedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(timestamp_with_time_zone_null(Users::DeletedAt))
                    .to_owned(),
            )
            .await?;

        // Create users indices
        db.execute_unprepared(
            r#"CREATE UNIQUE INDEX IF NOT EXISTS idx_users_username_lower ON users (LOWER(username))"#,
        )
        .await?;
        db.execute_unprepared(
            r#"CREATE UNIQUE INDEX IF NOT EXISTS idx_users_email_lower ON users (LOWER(email))"#,
        )
        .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_users_deleted_at")
                    .table(Users::Table)
                    .col(Users::DeletedAt)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_users_is_admin")
                    .table(Users::Table)
                    .col(Users::IsAdmin)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;

        // ========================================
        // 3. Create Owner Members Table
        // ========================================
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
                    .col(char_len(OwnerMembers::OwnerId, 21))
                    .col(char_len(OwnerMembers::UserId, 21))
                    .col(
                        timestamp_with_time_zone(OwnerMembers::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_owner_members_owner_id")
                            .from(OwnerMembers::Table, OwnerMembers::OwnerId)
                            .to(Owners::Table, Owners::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_owner_members_user_id")
                            .from(OwnerMembers::Table, OwnerMembers::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx_owner_members_owner_user_unique")
                            .table(OwnerMembers::Table)
                            .col(OwnerMembers::OwnerId)
                            .col(OwnerMembers::UserId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        // ========================================
        // 4. Create Albums Table
        // ========================================
        manager
            .create_table(
                Table::create()
                    .table(Albums::Table)
                    .if_not_exists()
                    .col(char_len(Albums::Id, 21).primary_key())
                    .col(char_len(Albums::OwnerId, 21))
                    .col(string(Albums::Name))
                    .col(text(Albums::Description))
                    .col(
                        timestamp_with_time_zone(Albums::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Albums::ModifiedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(timestamp_with_time_zone_null(Albums::DeletedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_albums_owner_id")
                            .from(Albums::Table, Albums::OwnerId)
                            .to(Owners::Table, Owners::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create albums indices
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_albums_owner_id")
                    .table(Albums::Table)
                    .col(Albums::OwnerId)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_albums_name")
                    .table(Albums::Table)
                    .col(Albums::Name)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_albums_description")
                    .table(Albums::Table)
                    .col(Albums::Description)
                    .index_type(IndexType::FullText)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_albums_created_at")
                    .table(Albums::Table)
                    .col(Albums::CreatedAt)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_albums_modified_at")
                    .table(Albums::Table)
                    .col(Albums::ModifiedAt)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_albums_deleted_at")
                    .table(Albums::Table)
                    .col(Albums::DeletedAt)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await?;

        // ========================================
        // 5. Create Album Shares Table
        // ========================================
        manager
            .create_table(
                Table::create()
                    .table(AlbumShares::Table)
                    .if_not_exists()
                    .col(char_len(AlbumShares::Id, 21).primary_key())
                    .col(char_len(AlbumShares::AlbumId, 21))
                    .col(char_len(AlbumShares::UserId, 21))
                    .col(string_len(AlbumShares::Permission, 10))
                    .col(
                        timestamp_with_time_zone(AlbumShares::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_album_shares_album_id")
                            .from(AlbumShares::Table, AlbumShares::AlbumId)
                            .to(Albums::Table, Albums::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_album_shares_user_id")
                            .from(AlbumShares::Table, AlbumShares::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // ========================================
        // 6. Create Assets Table
        // ========================================
        manager
            .create_table(
                Table::create()
                    .table(Assets::Table)
                    .if_not_exists()
                    .col(char_len(Assets::Id, 21).primary_key())
                    .col(char_len(Assets::OwnerId, 21))
                    .col(char_len_null(Assets::AlbumId, 21))
                    .col(unsigned(Assets::Width))
                    .col(unsigned(Assets::Height))
                    .col(string_len(Assets::AssetType, 2))
                    .col(string(Assets::OriginalFilename))
                    .col(big_integer(Assets::FileSize))
                    .col(big_integer(Assets::FileHash)) // i64 hash stored as bigint
                    .col(string(Assets::ContentType))
                    .col(timestamp_with_time_zone_null(Assets::Date)) // Nullable as per entity
                    .col(
                        timestamp_with_time_zone(Assets::UploadedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Assets::ModifiedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(timestamp_with_time_zone_null(Assets::DeletedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_assets_owner_id")
                            .from(Assets::Table, Assets::OwnerId)
                            .to(Owners::Table, Owners::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_assets_album_id")
                            .from(Assets::Table, Assets::AlbumId)
                            .to(Albums::Table, Albums::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        // Create assets indices
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_assets_owner_id")
                    .table(Assets::Table)
                    .col(Assets::OwnerId)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_assets_album_id")
                    .table(Assets::Table)
                    .col(Assets::AlbumId)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_assets_date")
                    .table(Assets::Table)
                    .col(Assets::Date)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_assets_uploaded_at")
                    .table(Assets::Table)
                    .col(Assets::UploadedAt)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_assets_modified_at")
                    .table(Assets::Table)
                    .col(Assets::ModifiedAt)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_assets_deleted_at")
                    .table(Assets::Table)
                    .col(Assets::DeletedAt)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order of creation (respecting foreign keys)
        manager
            .drop_table(Table::drop().table(Assets::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(AlbumShares::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Albums::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(OwnerMembers::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Owners::Table).to_owned())
            .await?;

        Ok(())
    }
}

// ========================================
// Table Identifiers
// ========================================

#[derive(DeriveIden)]
enum Owners {
    Table,
    Id,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    Name,
    Email,
    ProfileImageUrl,
    AccountVerified,
    NeedsOnboarding,
    PasswordHash,
    PasswordResetToken,
    PasswordResetExpiresAt,
    LastLoginAt,
    FailedLoginAttempts,
    IsAdmin,
    CreatedAt,
    ModifiedAt,
    DeletedAt,
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
enum Albums {
    Table,
    Id,
    OwnerId,
    Name,
    Description,
    CreatedAt,
    ModifiedAt,
    DeletedAt,
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

#[derive(DeriveIden)]
enum Assets {
    Table,
    Id,
    OwnerId,
    AlbumId,
    Width,
    Height,
    AssetType,
    OriginalFilename,
    FileSize,
    FileHash,
    ContentType,
    Date,
    UploadedAt,
    ModifiedAt,
    DeletedAt,
}
