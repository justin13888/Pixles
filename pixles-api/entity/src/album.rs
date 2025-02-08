use chrono::{DateTime, Utc};
use nanoid::nanoid;
use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};

// TODO: Check
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "albums")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub user_id: String, // TODO: foreign key
    #[sea_orm(indexed)]
    pub name: String,
    pub description: String, // TODO: make this full-text searchable
    #[sea_orm(
        column_type = "TimestampWithTimeZone",
        default_value = "CURRENT_TIMESTAMP"
    )]
    pub created_at: DateTime<Utc>, // TODO: Index
    #[sea_orm(
        column_type = "TimestampWithTimeZone",
        default_value = "CURRENT_TIMESTAMP",
        on_update = "CURRENT_TIMESTAMP"
    )]
    pub modified_at: DateTime<Utc>, // TODO: Index
    /// Date when the album was deleted if not NULL
    #[sea_orm(column_type = "TimestampWithTimeZone", nullable)]
    pub deleted_at: Option<DateTime<Utc>>, // TODO: Index
}
// TODO: Index username, email

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
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
    // pub fn find_by_username(username: &str) -> Select<Entity> {
    //     Self::find().filter(Column::Username.eq(username))
    // }

    // pub fn find_by_email(email: &str) -> Select<Entity> {
    //     Self::find().filter(Column::Email.eq(email))
    // }
}
