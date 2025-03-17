use chrono::{DateTime, Utc};
use nanoid::nanoid;
use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};

// TODO: Check
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Char(Some(21))")]
    pub id: String,
    #[sea_orm(unique, column_type = "String(StringLen::N(64))", indexed)]
    pub username: String,
    pub name: String,
    #[sea_orm(unique, column_type = "String(StringLen::N(255))", indexed)]
    pub email: String, // TODO: Ensure uniqueness after normalization (see if standard disregards certain character)
    pub account_verified: bool,
    pub needs_onboarding: bool,
    pub hashed_password: String, // TODO: Rename to `password_hash`
    // pub external_id: String,
    #[sea_orm(indexed)]
    pub is_admin: bool,
    #[sea_orm(
        column_type = "TimestampWithTimeZone",
        default_value = "CURRENT_TIMESTAMP"
    )]
    pub created_at: DateTime<Utc>,
    #[sea_orm(
        column_type = "TimestampWithTimeZone",
        default_value = "CURRENT_TIMESTAMP",
        on_update = "CURRENT_TIMESTAMP"
    )]
    pub modified_at: DateTime<Utc>,
    /// Date when the user was deleted if not NULL
    #[sea_orm(column_type = "TimestampWithTimeZone", nullable, indexed)]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::album::Entity")]
    Albums,
}

impl Related<super::album::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Albums.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(nanoid!()),
            ..ActiveModelTrait::default()
        }
    }
}

impl Entity {
    pub fn find_by_username(username: &str) -> Select<Entity> {
        Self::find().filter(Column::Username.eq(username))
    }

    pub fn find_by_email(email: &str) -> Select<Entity> {
        Self::find().filter(Column::Email.eq(email))
    }
}
