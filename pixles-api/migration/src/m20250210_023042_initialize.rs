use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // TODO: Inspect the generated postgres schema for column types and indices

        let db = manager.get_connection();

        db.execute_unprepared("CREATE EXTENSION IF NOT EXISTS pg_trgm;")
            .await?;
        db.execute_unprepared("CREATE EXTENSION IF NOT EXISTS btree_gin;")
            .await?;

        // Create the users table
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(char_len(Users::Id, 21).primary_key())
                    .col(string_len(Users::Username, 64))
                    .col(string(Users::Name))
                    .col(string_len(Users::Email, 255))
                    .col(boolean(Users::AccountVerified))
                    .col(boolean(Users::NeedsOnboarding))
                    .col(string(Users::HashedPassword))
                    .col(boolean(Users::IsAdmin))
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
            r#"CREATE UNIQUE INDEX IF NOT EXISTS idx_username_lower ON users (LOWER(username))"#,
        )
        .await?; // TODO: Verify index prevents usernames with difference casing to be inserted
        db.execute_unprepared(
            r#"CREATE UNIQUE INDEX IF NOT EXISTS idx_email_lower ON users (LOWER(email))"#,
        )
        .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_deleted_at")
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
                    .name("idx_is_admin")
                    .table(Users::Table)
                    .col(Users::IsAdmin)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;

        // Create albums table
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
                    .to_owned(),
            )
            .await?;

        // Create albums indices
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_owner_id")
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
                    .name("idx_name")
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
                    .name("idx_description")
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
                    .name("idx_created_at")
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
                    .name("idx_modified_at")
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
                    .name("idx_deleted_at")
                    .table(Albums::Table)
                    .col(Albums::DeletedAt)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await?;

        // // Add albums.user_id foreign key constraint
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_albums_owner_id")
                    .from(Albums::Table, Albums::OwnerId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .to_owned(),
            )
            .await?;

        // Create assets table
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
                    .col(timestamp_with_time_zone(Assets::Date))
                    .col(timestamp_with_time_zone(Assets::UploadedAt))
                    .col(timestamp_with_time_zone(Assets::ModifiedAt))
                    .col(timestamp_with_time_zone_null(Assets::DeletedAt))
                    .to_owned(),
            )
            .await?;

        // Create assets indices
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_owner_id")
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
                    .name("idx_album_id")
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
                    .name("idx_date")
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
                    .name("idx_uploaded_at")
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
                    .name("idx_modified_at")
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
                    .name("idx_deleted_at")
                    .table(Assets::Table)
                    .col(Assets::DeletedAt)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order
        manager
            .drop_table(Table::drop().table(Assets::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Albums::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    Name,
    Email,
    AccountVerified,
    NeedsOnboarding,
    HashedPassword,
    IsAdmin,
    CreatedAt,
    ModifiedAt,
    DeletedAt,
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
enum Assets {
    Table,
    Id,
    OwnerId,
    AlbumId,
    Width,
    Height,
    Date,
    UploadedAt,
    ModifiedAt,
    DeletedAt,
}
