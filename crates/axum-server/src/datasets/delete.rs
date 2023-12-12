use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};
use db::queries;
use db::Pool;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct Delete {
    pub id: i32,
    pub team_id: i32,
}

pub async fn delete(
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Form(delete_form): Form<Delete>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    queries::datasets::delete()
        .bind(&transaction, &delete_form.id)
        .await?;

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &ui_pages::routes::api_keys::index_route(delete_form.team_id),
        "Document Deleted",
    )
}
