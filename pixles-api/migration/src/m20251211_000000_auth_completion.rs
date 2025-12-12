use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(Users::PasswordResetToken)
                            .string()
                            .unique_key(),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(Users::PasswordResetExpiresAt).timestamp_with_time_zone(),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(Users::LastLoginAt).timestamp_with_time_zone(),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(Users::FailedLoginAttempts)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .drop_column(Users::PasswordResetToken)
                    .drop_column(Users::PasswordResetExpiresAt)
                    .drop_column(Users::LastLoginAt)
                    .drop_column(Users::FailedLoginAttempts)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    PasswordResetToken,
    PasswordResetExpiresAt,
    LastLoginAt,
    FailedLoginAttempts,
}
