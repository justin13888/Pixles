use ::entity::{user, user::Entity as User};
use sea_orm::*;

// TODO: Finish
pub struct Mutation;

impl Mutation {
    pub async fn create_user(
        db: &DbConn,
        username: String,
        email: String,
        hashed_password: String,
    ) -> Result<user::ActiveModel, DbErr> {
        user::ActiveModel {
            username: Set(username),
            email: Set(email),
            hashed_password: Set(hashed_password),
            ..Default::default()
        }
        .save(db)
        .await
    }

    // pub async fn update_user_by_id(
    //     db: &DbConn,
    //     id: i32,
    //     form_data: user::Model,
    // ) -> Result<user::Model, DbErr> {
    //     let user: user::ActiveModel = User::find_by_id(id)
    //         .one(db)
    //         .await?
    //         .ok_or(DbErr::Custom("Cannot find user.".to_owned()))
    //         .map(Into::into)?;

    //     user::ActiveModel {
    //         id: user.id,
    //         title: Set(form_data.title.to_owned()),
    //         text: Set(form_data.text.to_owned()),
    //     }
    //     .update(db)
    //     .await
    // }

    // pub async fn delete_user(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
    //     let user: user::ActiveModel = User::find_by_id(id)
    //         .one(db)
    //         .await?
    //         .ok_or(DbErr::Custom("Cannot find user.".to_owned()))
    //         .map(Into::into)?;

    //     user.delete(db).await
    // }
}
