use kernel::model::id::UserId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(debug_assertions, derive(ToSchema))]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(debug_assertions, derive(ToSchema))]
pub struct AccessTokenResponse {
    pub user_id: UserId,
    pub access_token: String,
}
