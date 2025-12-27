use crate::{constants::MAX_UPLOAD_SESSION_DURATION_SECONDS, schema::user::User};

use super::{
    AssetMetadata, CreateAssetInput, CreateUploadSessionInput, UploadSession, UploadStatus,
};
use async_graphql::*;
use chrono::{Duration, Utc};
use nanoid::nanoid;

pub struct AssetMutation;

#[Object]
impl AssetMutation {
    async fn create_upload_session(
        &self,
        ctx: &Context<'_>,
        input: CreateUploadSessionInput,
    ) -> Result<UploadSession> {
        let CreateUploadSessionInput { method, album_id } = input;

        let upload_session = UploadSession {
            id: ID::from(nanoid!()),
            user: todo!(), // TODO: Get user from context
            method,
            album_id: album_id.map(|id| id.to_string()),
            status: UploadStatus::Pending,
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::seconds(MAX_UPLOAD_SESSION_DURATION_SECONDS),
        };

        // TODO: Save to database (need to make upload_session model)

        // match method {
        //     UploadMethod::Direct => {
        //         // TODO: Save to database (need to make upload_session model)
        //     }
        //     UploadMethod::Multipart => {
        //         // TODO: Save to database (need to make upload_session model)
        //     }
        //     UploadMethod::Tus => {
        //         // TODO: Implement TUS upload
        //     }
        // }
        Ok(upload_session)
    }

    async fn upload_asset(
        &self,
        ctx: &Context<'_>,
        input: CreateAssetInput,
    ) -> Result<AssetMetadata> {
        // TODO: Validate session ID is associated with

        todo!()
    }

    /// Close an upload session
    async fn close_upload_session(&self, ctx: &Context<'_>, id: ID) -> Result<UploadSession> {
        // E.g. check if no more uploads are in progress
        todo!()
    }

    // TODO: Add mutation to make sure upload session is dealt with

    // async fn update_asset(
    //     &self,
    //     ctx: &Context<'_>,
    //     id: ID,
    //     input: UpdateAssetInput,
    // ) -> Result<AssetMetadata> {
    //     todo!()
    // }
}
