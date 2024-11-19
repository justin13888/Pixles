use ::entity::{user, user::Entity as User};
use sea_orm::*;

// TODO: Implement Query
pub struct Query;

impl Query {
    pub async fn find_user_by_id(db: &DbConn, id: String) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(id).one(db).await
    }

    pub async fn find_user_by_email(
        db: &DbConn,
        email: &str,
    ) -> Result<Option<user::Model>, DbErr> {
        User::find()
            .filter(user::Column::Email.eq(email))
            .one(db)
            .await
    }

    // /// If ok, returns (user models, num pages).
    // pub async fn find_users_in_page(
    //     db: &DbConn,
    //     page: u64,
    //     users_per_page: u64,
    // ) -> Result<(Vec<user::Model>, u64), DbErr> {
    //     // Setup paginator
    //     let paginator = User::find()
    //         .order_by_asc(user::Column::Id)
    //         .paginate(db, users_per_page);
    //     let num_pages = paginator.num_pages().await?;

    //     // Fetch paginated users
    //     paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    // }
}
