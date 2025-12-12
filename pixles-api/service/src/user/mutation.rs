use ::entity::{user, user::Entity as User};
use model::user::{CreateUser, UpdateUser};
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    /// Creates a new user
    pub async fn create_user(db: &DbConn, user: CreateUser) -> Result<user::Model, DbErr> {
        let CreateUser {
            username,
            name,
            email,
            password_hash,
        } = user;

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

    /// Updates an existing user
    pub async fn update_user(
        db: &DbConn,
        id: String,
        user: UpdateUser,
    ) -> Result<user::Model, DbErr> {
        let UpdateUser {
            username,
            name,
            email,
            password_hash,
        } = user;

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

    /// Tracks a successful login for a user
    pub async fn track_login_success(db: &DbConn, id: String) -> Result<user::Model, DbErr> {
        let user_model = User::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("User not found".to_string()))?;

        let mut user: user::ActiveModel = user_model.into();
        user.last_login_at = Set(Some(chrono::Utc::now()));
        user.failed_login_attempts = Set(0);

        user.update(db).await
    }

    /// Tracks a failed login attempt for a user
    pub async fn track_login_failure(db: &DbConn, id: String) -> Result<user::Model, DbErr> {
        let user_model = User::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("User not found".to_string()))?;

        // Increment failed attempts
        let current_attempts = user_model.failed_login_attempts;
        let mut user: user::ActiveModel = user_model.into();
        user.failed_login_attempts = Set(current_attempts + 1);

        user.update(db).await
    }

    /// Updates a user's password reset token
    pub async fn update_password_reset_token(
        db: &DbConn,
        id: String,
        token: String,
        expires_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<user::Model, DbErr> {
        let user_model = User::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("User not found".to_string()))?;

        let mut user: user::ActiveModel = user_model.into();
        user.password_reset_token = Set(Some(token));
        user.password_reset_expires_at = Set(Some(expires_at));

        user.update(db).await
    }

    /// Confirms a password reset for a user
    pub async fn confirm_password_reset(
        db: &DbConn,
        id: String,
        new_password_hash: String,
    ) -> Result<user::Model, DbErr> {
        let user_model = User::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("User not found".to_string()))?;

        let mut user: user::ActiveModel = user_model.into();
        user.password_hash = Set(new_password_hash);
        user.password_reset_token = Set(None);
        user.password_reset_expires_at = Set(None);

        user.update(db).await
    }
}
