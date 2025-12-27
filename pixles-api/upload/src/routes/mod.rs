use salvo::prelude::*;

use crate::state::AppState;

mod tus;

pub fn get_router(state: AppState) -> Router {
    Router::new()
        .hoop(affix_state::inject(state))
        .push(Router::with_path("status").get(status))
        .push(Router::new().post(tus::create_upload))
        .push(
            Router::with_path("<id>")
                .head(tus::head_upload)
                .patch(tus::patch_upload)
                .delete(tus::delete_upload),
        )
}

#[handler]
async fn status() -> &'static str {
    "Upload service is running"
}
