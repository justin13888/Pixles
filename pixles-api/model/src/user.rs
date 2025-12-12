pub struct CreateUser {
    pub username: String,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

pub struct UpdateUser {
    pub username: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<String>,
}
