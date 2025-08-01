//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.14

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "role")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub data_scope: i16,
    pub status: i16,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::role_menu::Entity")]
    RoleMenu,
    #[sea_orm(has_many = "super::user_role::Entity")]
    UserRole,
}

impl Related<super::role_menu::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RoleMenu.def()
    }
}

impl Related<super::user_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserRole.def()
    }
}

impl Related<super::menu::Entity> for Entity {
    fn to() -> RelationDef {
        super::role_menu::Relation::Menu.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::role_menu::Relation::Role.def().rev())
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_role::Relation::User.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::user_role::Relation::Role.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
