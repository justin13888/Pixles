use chrono::{DateTime, FixedOffset, Utc};
use nanoid::nanoid;
use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};

// TODO: Check
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "assets")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    #[sea_orm(indexed)]
    pub owner_id: String,
    #[sea_orm(nullable)]
    pub album_id: Option<String>,
    /// Width of asset
    pub width: i32,
    /// Height of asset
    pub height: i32,
    /// Date when the asset was taken
    pub date: DateTime<Utc>,
    #[sea_orm(
        column_type = "TimestampWithTimeZone",
        default_value = "CURRENT_TIMESTAMP"
    )]
    pub uploaded_at: DateTime<Utc>,
    #[sea_orm(
        column_type = "TimestampWithTimeZone",
        default_value = "CURRENT_TIMESTAMP",
        on_update = "CURRENT_TIMESTAMP"
    )]
    pub modified_at: DateTime<FixedOffset>,
    /// Date when the album was deleted if not NULL
    #[sea_orm(column_type = "TimestampWithTimeZone", nullable)]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(2))")]
pub enum AssetType {
    #[sea_orm(string_value = "ph")]
    Photo,
    #[sea_orm(string_value = "vi")]
    Video,
    #[sea_orm(string_value = "mp")]
    MotionPhoto,
    #[sea_orm(string_value = "sc")]
    Sidecar,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::OwnerId",
        to = "super::user::Column::Id"
    )]
    Owner,
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
    pub fn find_by_owner_id(owner_id: &str) -> Select<Entity> {
        Self::find().filter(Column::OwnerId.eq(owner_id))
    }

    pub fn find_by_album_id(album_id: &str) -> Select<Entity> {
        Self::find().filter(Column::AlbumId.eq(album_id))
    }

    pub fn find_by_owner_id_and_album_id(owner_id: &str, album_id: &str) -> Select<Entity> {
        Self::find()
            .filter(Column::OwnerId.eq(owner_id))
            .filter(Column::AlbumId.eq(album_id))
    }
}
