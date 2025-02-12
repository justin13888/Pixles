use async_graphql::{dataloader::*, FieldError};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::{collections::HashMap, sync::Arc};

use crate::schema::user::User;
use entity::user;

// TODO: FInish implementation for user and all other entities

pub struct UserLoader {
    pub conn: Arc<DatabaseConnection>,
}

impl Loader<String> for UserLoader {
    type Value = User;
    type Error = FieldError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let users = user::Entity::find()
            .filter(user::Column::Id.is_in(keys.to_vec()))
            .all(&*self.conn)
            .await
            .unwrap_or_default();

        let mut map = HashMap::new();
        for user in users {
            map.insert(user.id.clone(), user.into());
        }
        // map
        Ok(map)
    }
}

pub struct Loaders {
    pub user_loader: DataLoader<UserLoader>,
}

impl Loaders {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self {
            user_loader: DataLoader::new(UserLoader { conn }, tokio::spawn),
        }
    }
}
