mod api_type;

mod middleware;

mod auth;
mod menu;
mod role;
mod user;

mod router;
pub use router::*;

pub const AUTH_TAG: &str = "Auth";
pub const USER_TAG: &str = "User";
pub const ROLE_TAG: &str = "Role";
pub const MENU_TAG: &str = "Menu";
