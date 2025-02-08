use crate::loaders::Loaders;
use crate::schema::admin::{AdminMutation, AdminQuery};
use crate::schema::media::{MediaMutation, MediaQuery};
use crate::schema::user::{UserMutation, UserQuery};
use async_graphql::{extensions::Logger, EmptySubscription, Object, Schema};

pub mod admin;
pub mod media;
pub mod user;

pub struct QueryRoot(UserQuery, MediaQuery, AdminQuery);
pub struct MutationRoot(UserMutation, MediaMutation, AdminMutation);

#[Object]
impl QueryRoot {
    async fn user(&self) -> &UserQuery {
        &self.0
    }

    async fn media(&self) -> &MediaQuery {
        &self.1
    }

    async fn admin(&self) -> &AdminQuery {
        &self.2
    }
}

#[Object]
impl MutationRoot {
    async fn user(&self) -> &UserMutation {
        &self.0
    }

    async fn media(&self) -> &MediaMutation {
        &self.1
    }

    async fn admin(&self) -> &AdminMutation {
        &self.2
    }
}

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema(loaders: Loaders) -> AppSchema {
    let schema = Schema::build(
        QueryRoot(UserQuery, MediaQuery, AdminQuery),
        MutationRoot(UserMutation, MediaMutation, AdminMutation),
        EmptySubscription,
    );

    #[cfg(debug_assertions)]
    let schema = schema.extension(Logger);

    schema.data(loaders).finish()
}
