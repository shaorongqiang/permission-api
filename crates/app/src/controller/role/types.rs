use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::entity::RoleModel;

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
pub struct Role {
    pub id: i32,
    pub name: String,
    pub data_scope: i16,
    pub status: i16,
}
impl From<RoleModel> for Role {
    fn from(role: RoleModel) -> Self {
        Role {
            id: role.id,
            name: role.name,
            data_scope: role.data_scope,
            status: role.status,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct ListResponse {
    pub roles: Vec<Role>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateRequest {
    pub name: String,
    pub data_scope: i16,
    pub status: i16,
}

#[derive(Serialize, ToSchema)]
pub struct CreateResponse {
    pub role: Role,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateRequest {
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_scope: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i16>,
}

#[derive(Serialize, ToSchema)]
pub struct GetResponse {
    pub role: Role,
}
