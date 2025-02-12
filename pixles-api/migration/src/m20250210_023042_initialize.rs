use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // TODO: Inspect the generated postgres schema for column types and indices

        let db = manager.get_connection();

        // Create the users table
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(char_len(Users::Id, 21).primary_key())
                    .col(string_len_uniq(Users::Username, 64))
                    .col(string(Users::Name))
                    .col(string_len_uniq(Users::Email, 255))
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
                    .index(
                        Index::create()
                            .name("idx_email")
                            .col(Users::Email)
                            .index_type(IndexType::Hash),
                    )
                    .index(
                        Index::create()
                            .name("idx_deleted_at")
                            .col(Users::DeletedAt)
                            .index_type(IndexType::BTree),
                    )
                    .index(
                        Index::create()
                            .name("idx_is_admin")
                            .col(Users::IsAdmin)
                            .index_type(IndexType::Hash),
                    )
                    .to_owned(),
            )
            .await?;

        // Create users.username index
        db.execute_unprepared(
            r#"CREATE UNIQUE INDEX idx_username_lower ON users (LOWER(username))"#,
        )
        .await?; // TODO: Verify index prevents usernames with difference casing to be inserted

        // Create albums table
        manager
            .create_table(
                Table::create()
                    .table(Albums::Table)
                    .if_not_exists()
                    .col(char_len(Albums::Id, 21).primary_key())
                    .col(char_len(Albums::OwnerId, 21))
                    .col(string(Albums::Name))
                    .col(string(Albums::Description))
                    .col(
                        timestamp_with_time_zone(Albums::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Albums::ModifiedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(timestamp_with_time_zone_null(Albums::DeletedAt))
                    .index(
                        Index::create()
                            .name("idx_owner_id")
                            .col(Albums::OwnerId)
                            .index_type(IndexType::Hash),
                    )
                    .index(
                        Index::create()
                            .name("idx_name")
                            .col(Albums::Name)
                            .index_type(IndexType::BTree),
                    )
                    .index(
                        Index::create()
                            .name("idx_created_at")
                            .col(Albums::CreatedAt)
                            .index_type(IndexType::BTree),
                    )
                    .index(
                        Index::create()
                            .name("idx_modified_at")
                            .col(Albums::ModifiedAt)
                            .index_type(IndexType::BTree),
                    )
                    .index(
                        Index::create()
                            .name("idx_deleted_at")
                            .col(Albums::DeletedAt)
                            .index_type(IndexType::BTree),
                    )
                    .to_owned(),
            )
            .await?;

        // Add albums.user_id foreign key constraint
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
                    .index(
                        Index::create()
                            .name("idx_owner_id")
                            .col(Assets::OwnerId)
                            .index_type(IndexType::Hash),
                    )
                    .index(
                        Index::create()
                            .name("idx_album_id")
                            .col(Assets::AlbumId)
                            .index_type(IndexType::Hash),
                    )
                    .index(
                        Index::create()
                            .name("idx_date")
                            .col(Assets::Date)
                            .index_type(IndexType::BTree),
                    )
                    .index(
                        Index::create()
                            .name("idx_uploaded_at")
                            .col(Assets::UploadedAt)
                            .index_type(IndexType::BTree),
                    )
                    .index(
                        Index::create()
                            .name("idx_modified_at")
                            .col(Assets::ModifiedAt)
                            .index_type(IndexType::BTree),
                    )
                    .index(
                        Index::create()
                            .name("idx_deleted_at")
                            .col(Assets::DeletedAt)
                            .index_type(IndexType::BTree),
                    )
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
