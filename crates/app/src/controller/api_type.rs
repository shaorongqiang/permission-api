use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use validator::Validate;

const SUCCESS_CODE: i32 = 0;
const USERNAME_NOT_FOUND_CODE: i32 = -1;
const WRONG_PASSWORD_CODE: i32 = -2;

#[derive(Deserialize, ToSchema, Validate)]
pub struct ApiRequest<T> {
    pub id: Value,
    pub params: T,
}

#[derive(Serialize, ToSchema, Validate)]
pub struct ApiResponse<T> {
    pub id: Value,
    pub code: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl ApiResponse<String> {
    pub fn new_success_without_data(id: Value) -> Self {
        Self {
            id,
            code: SUCCESS_CODE,
            data: Some("success".to_string()),
            error: None,
        }
    }
}

impl<T> ApiResponse<T> {
    pub fn new_success(id: Value, data: T) -> Self {
        Self {
            id,
            code: SUCCESS_CODE,
            data: Some(data),
            error: None,
        }
    }

    #[allow(dead_code)]
    pub fn new_error(id: Value, code: i32, message: String) -> Self {
        Self {
            id,
            code,
            data: None,
            error: Some(message),
        }
    }

    pub fn username_not_found(id: Value) -> Self {
        Self {
            id,
            code: USERNAME_NOT_FOUND_CODE,
            data: None,
            error: Some(String::from("Username not found")),
        }
    }

    pub fn wrong_password(id: Value) -> Self {
        Self {
            id,
            code: WRONG_PASSWORD_CODE,
            data: None,
            error: Some(String::from("Wrong password")),
        }
    }
}
