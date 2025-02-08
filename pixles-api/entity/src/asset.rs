use chrono::{DateTime, Utc};
use nanoid::nanoid;
use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};

// TODO: Check
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "assets")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    #[sea_orm(nullable)]
    pub album_id: String, // TODO: foreign key
    pub dimension: Json,
    pub date: DateTime<Utc>, // TODO: Index
    #[sea_orm(
        column_type = "TimestampWithTimeZone",
        default_value = "CURRENT_TIMESTAMP"
    )]
    pub uploaded_at: DateTime<Utc>, // TODO: Index
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
        belongs_to = "super::album::Entity",
        from = "Column::AlbumId",
        to = "super::album::Column::Id"
    )]
    Album,
}

impl Related<super::album::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Album.def()
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
