use super::types::{Admin, CreateAdminInput, UpdateAdminInput};
use async_graphql::*;

pub struct AdminMutation;

#[Object]
impl AdminMutation {
    async fn create_admin(&self, ctx: &Context<'_>, input: CreateAdminInput) -> Result<Admin> {
        todo!()
    }

    async fn update_admin(
        &self,
        ctx: &Context<'_>,
        id: ID,
        input: UpdateAdminInput,
    ) -> Result<Admin> {
        todo!()
    }
}
