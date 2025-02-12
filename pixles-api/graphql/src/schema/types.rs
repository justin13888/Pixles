use async_graphql::*;

#[derive(SimpleObject)]
pub struct Tag {
    pub id: String,
    pub name: String,
}
