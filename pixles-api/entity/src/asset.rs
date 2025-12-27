use chrono::{DateTime, FixedOffset, Utc};
use nanoid::nanoid;
use sea_orm::Set;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

// TODO: Check
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "assets")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    #[sea_orm(indexed)]
    pub owner_id: String,
    #[sea_orm(nullable, indexed)]
    pub album_id: Option<String>,

    /// Width of asset
    pub width: i32,
    /// Height of asset
    pub height: i32,

    pub asset_type: AssetType,

    /// Original filename from client
    pub original_filename: String,
    /// File size in bytes
    pub file_size: i64,
    /// XXH3 hash of the file content
    /// We just need a 64-bit value but Postgres only has BigInt (8-byte signed integer).
    /// Just cast between u64 and i64 using `as` keyword.
    pub file_hash: i64,
    /// MIME type
    pub content_type: String,

    /// Date when the asset was taken
    #[sea_orm(indexed)]
    pub date: Option<DateTime<Utc>>,
    #[sea_orm(
        column_type = "TimestampWithTimeZone",
        default_value = "CURRENT_TIMESTAMP",
        indexed
    )]
    pub uploaded_at: DateTime<Utc>,
    #[sea_orm(
        column_type = "TimestampWithTimeZone",
        default_value = "CURRENT_TIMESTAMP",
        on_update = "CURRENT_TIMESTAMP",
        indexed
    )]
    pub modified_at: DateTime<FixedOffset>,
    /// Date when the album was deleted if not NULL
    #[sea_orm(column_type = "TimestampWithTimeZone", nullable, indexed)]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
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
        belongs_to = "super::owner::Entity",
        from = "Column::OwnerId",
        to = "super::owner::Column::Id"
    )]
    Owner,
    #[sea_orm(
        belongs_to = "super::album::Entity",
        from = "Column::AlbumId",
        to = "super::album::Column::Id"
    )]
    Album,
}

impl Related<super::owner::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Owner.def()
    }
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
