use crate::loaders::Loaders;
use crate::schema::album::{AlbumMutation, AlbumQuery};
use crate::schema::asset::{AssetMutation, AssetQuery, AssetSubscription};
use crate::schema::user::{UserMutation, UserQuery};
use async_graphql::MergedSubscription;
use async_graphql::{extensions::Logger, Object, Schema};

pub mod album;
pub mod asset;
mod types;
pub mod user;

pub use types::*;

pub struct QueryRoot {
    pub user: UserQuery,
    pub album: AlbumQuery,
    pub asset: AssetQuery,
}
pub struct MutationRoot {
    pub user: UserMutation,
    pub album: AlbumMutation,
    pub asset: AssetMutation,
}

#[Object]
impl QueryRoot {
    async fn user(&self) -> &UserQuery {
        &self.user
    }

    async fn album(&self) -> &AlbumQuery {
        &self.album
    }

    async fn asset(&self) -> &AssetQuery {
        &self.asset
    }
}

#[Object]
impl MutationRoot {
    async fn user(&self) -> &UserMutation {
        &self.user
    }

    async fn album(&self) -> &AlbumMutation {
        &self.album
    }

    async fn asset(&self) -> &AssetMutation {
        &self.asset
    }
}

#[derive(MergedSubscription, Default)]
pub struct SubscriptionRoot(AssetSubscription);

pub type AppSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

pub fn create_schema(loaders: Loaders) -> AppSchema {
    let schema = Schema::build(
        QueryRoot {
            user: UserQuery,
            album: AlbumQuery,
            asset: AssetQuery,
        },
        MutationRoot {
            user: UserMutation,
            album: AlbumMutation,
            asset: AssetMutation,
        },
        SubscriptionRoot::default(),
    );

    #[cfg(debug_assertions)]
    let schema = schema.extension(Logger);

    schema.data(loaders).finish()
}
