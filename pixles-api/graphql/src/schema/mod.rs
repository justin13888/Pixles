use activity::ActivityQuery;
use async_graphql::extensions::Logger;
use async_graphql::extensions::apollo_persisted_queries::{
    ApolloPersistedQueries, LruCacheStorage,
};
use async_graphql::{MergedSubscription, Object, Schema};

use crate::loaders::Loaders;
use crate::schema::album::{AlbumMutation, AlbumQuery};
use crate::schema::asset::{AssetMutation, AssetQuery, AssetSubscription};
use crate::schema::user::{UserMutation, UserQuery};
pub mod activity;
pub mod album;
pub mod asset;
mod types;
pub mod user;

pub use types::*;
use user::statistics::UserStatisticsQuery;

pub struct QueryRoot {
    pub user: UserQuery,
    pub album: AlbumQuery,
    pub asset: AssetQuery,
    pub activity: ActivityQuery,
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

    async fn activity(&self) -> &ActivityQuery {
        &self.activity
    }
}

#[Object]
impl MutationRoot {
    // async fn user(&self) -> &UserMutation {
    //     &self.user
    // }

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
            user: UserQuery {
                statistics: UserStatisticsQuery,
            },
            album: AlbumQuery,
            asset: AssetQuery,
            activity: ActivityQuery,
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

    // TODO: Setup twemproxy+memcached for distributed cache w/ r2d2-memcache
    let schema = schema.extension(ApolloPersistedQueries::new(LruCacheStorage::new(1024)));

    schema.data(loaders).finish()
}
