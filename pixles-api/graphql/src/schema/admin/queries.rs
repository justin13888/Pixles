use super::types::Admin;
use async_graphql::*;

pub struct AdminQuery;

#[Object]
impl AdminQuery {
    async fn get_admin(&self, ctx: &Context<'_>, id: ID) -> Result<Admin> {
        todo!()
    }

    async fn list_admins(&self, ctx: &Context<'_>) -> Result<Vec<Admin>> {
        todo!()
    }
}
