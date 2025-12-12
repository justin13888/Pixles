use ::entity::{user, user::Entity as User};
use sea_orm::*;

// TODO: Finish
pub struct Mutation;

impl Mutation {
    pub async fn create_user(
        db: &DbConn,
        username: String,
        name: String,
        email: String,
        password_hash: String,
    ) -> Result<user::Model, DbErr> {
        user::ActiveModel {
            username: Set(username),
            name: Set(name),
            email: Set(email),
            password_hash: Set(password_hash),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    pub async fn update_user(
        db: &DbConn,
        id: String,
        username: Option<String>,
        name: Option<String>,
        email: Option<String>,
        password_hash: Option<String>,
    ) -> Result<user::Model, DbErr> {
        let user: user::ActiveModel = User::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("User not found".to_string()))?
            .into();

        let mut user = user;

        if let Some(username) = username {
            user.username = Set(username);
        }
        if let Some(name) = name {
            user.name = Set(name);
        }
        if let Some(email) = email {
            user.email = Set(email);
        }
        if let Some(password_hash) = password_hash {
            user.password_hash = Set(password_hash);
        }

        user.update(db).await
    }
}
