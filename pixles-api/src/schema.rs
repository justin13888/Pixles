use crate::graphql::admin::{AdminMutation, AdminQuery};
use crate::graphql::media::{MediaMutation, MediaQuery};
use crate::graphql::user::{UserMutation, UserQuery};
use async_graphql::{EmptySubscription, Object, Schema};

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

pub fn create_schema() -> AppSchema {
    Schema::build(
        QueryRoot(UserQuery, MediaQuery, AdminQuery),
        MutationRoot(UserMutation, MediaMutation, AdminMutation),
        EmptySubscription,
    )
    .finish()
}
