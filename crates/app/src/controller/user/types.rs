use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::entity::UserModel;

#[derive(Deserialize, ToSchema, Validate)]
pub struct ListRequest {
    #[validate(range(min = 1, message = "page must be greater than 0"))]
    pub page: u64,
    #[validate(range(
        min = 1,
        max = 100,
        message = "page_size must be greater than 0 and less than 100"
    ))]
    pub page_size: u64,
}

#[derive(Serialize, ToSchema)]
pub struct User {
    pub id: i64,
    pub username: String,
}
impl From<UserModel> for User {
    fn from(user: UserModel) -> Self {
        User {
            id: user.id,
            username: user.name,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct ListResponse {
    pub users: Vec<User>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateRequest {
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct GetResponse {
    pub user: User,
}
