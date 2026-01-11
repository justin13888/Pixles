use async_graphql::*;
use chrono::{DateTime, Utc};
use entity::person::Model as PersonModel;

use super::asset::{AssetMetadata, BoundingBox};

/// A recognized person in the photo library
pub struct Person {
    pub model: PersonModel,
}

#[Object]
impl Person {
    async fn id(&self) -> ID {
        ID::from(&self.model.id)
    }

    async fn name(&self) -> Option<&String> {
        self.model.name.as_ref()
    }

    // TODO: Resolve cover photo via dataloader
    async fn cover_photo(&self) -> Option<AssetMetadata> {
        None
    }

    async fn is_hidden(&self) -> bool {
        self.model.is_hidden
    }

    async fn face_count(&self) -> i32 {
        self.model.face_count
    }

    async fn created_at(&self) -> DateTime<Utc> {
        self.model.created_at
    }
}

/// A detected face within an asset
#[derive(SimpleObject)]
pub struct Face {
    pub id: ID,
    // TODO: Resolve asset via dataloader
    // pub asset: AssetMetadata,
    // TODO: Resolve person via dataloader
    // pub person: Option<Person>,
    pub bounding_box: BoundingBox,
    pub confidence: f64,
    pub is_confirmed: bool,
}

// ===== Inputs =====

#[derive(InputObject)]
pub struct UpdatePersonInput {
    pub name: Option<String>,
    pub is_hidden: Option<bool>,
    pub cover_photo_id: Option<ID>,
}

// ===== Query =====

#[derive(Default)]
pub struct PersonQuery;

#[Object]
impl PersonQuery {
    /// Get person by ID
    async fn by_id(&self, ctx: &Context<'_>, id: ID) -> Result<Person> {
        todo!("Implement get person by ID")
    }

    /// Get all people (optionally including hidden)
    async fn all(&self, ctx: &Context<'_>, include_hidden: Option<bool>) -> Result<Vec<Person>> {
        todo!("Implement list all people")
    }

    /// Search people by name
    async fn search(&self, ctx: &Context<'_>, query: String) -> Result<Vec<Person>> {
        todo!("Implement person search")
    }

    /// Get unassigned faces (faces not yet linked to a person)
    async fn unassigned_faces(&self, ctx: &Context<'_>, limit: Option<i32>) -> Result<Vec<Face>> {
        todo!("Implement get unassigned faces")
    }
}

// ===== Mutations =====

#[derive(Default)]
pub struct PersonMutation;

#[Object]
impl PersonMutation {
    /// Merge multiple people into one (e.g., "Mom" and "Mother")
    async fn merge_people(
        &self,
        ctx: &Context<'_>,
        source_ids: Vec<ID>,
        target_id: ID,
    ) -> Result<Person> {
        todo!("Implement merge people")
    }

    /// Update person name or visibility
    async fn update_person(
        &self,
        ctx: &Context<'_>,
        id: ID,
        input: UpdatePersonInput,
    ) -> Result<Person> {
        todo!("Implement update person")
    }

    /// Assign a face to a person
    async fn assign_face(&self, ctx: &Context<'_>, face_id: ID, person_id: ID) -> Result<Face> {
        todo!("Implement assign face")
    }

    /// Confirm or reject a suggested face assignment
    async fn confirm_face(&self, ctx: &Context<'_>, face_id: ID, confirmed: bool) -> Result<Face> {
        todo!("Implement confirm face")
    }

    /// Remove a face from a person (marks as unassigned)
    async fn unassign_face(&self, ctx: &Context<'_>, face_id: ID) -> Result<Face> {
        todo!("Implement unassign face")
    }
}
