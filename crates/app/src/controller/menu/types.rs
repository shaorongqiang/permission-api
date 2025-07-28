use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::entity::MenuModel;

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
pub struct Menu {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub is_frame: bool,
}
impl From<MenuModel> for Menu {
    fn from(menu: MenuModel) -> Self {
        Menu {
            id: menu.id,
            name: menu.name,
            path: menu.path,
            is_frame: menu.is_frame,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct ListResponse {
    pub menus: Vec<Menu>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateRequest {
    pub name: String,
    pub path: String,
    pub is_frame: bool,
}

#[derive(Serialize, ToSchema)]
pub struct CreateResponse {
    pub menu: Menu,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateRequest {
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_frame: Option<bool>,
}

#[derive(Serialize, ToSchema)]
pub struct GetResponse {
    pub menu: Menu,
}
