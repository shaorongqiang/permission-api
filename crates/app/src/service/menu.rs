use anyhow::Result;
use sea_orm::{
    ActiveValue::{NotSet, Set},
    ConnectionTrait, EntityTrait, QueryOrder, QuerySelect,
};

use crate::entity::{MenuActiveModel, MenuColumn, MenuEntity, MenuModel};

pub async fn create<C: ConnectionTrait>(
    db: &C,
    name: &str,
    path: &str,
    is_frame: bool,
) -> Result<MenuModel> {
    MenuEntity::insert(MenuActiveModel {
        id: NotSet,
        name: Set(name.to_string()),
        path: Set(path.to_string()),
        is_frame: Set(is_frame),
    })
    .exec_with_returning(db)
    .await
    .map_err(|e| anyhow::anyhow!("create role error: {}", e))
}

pub async fn delete<C: ConnectionTrait>(db: &C, id: i32) -> Result<()> {
    MenuEntity::delete_by_id(id)
        .exec(db)
        .await
        .map(|_| ())
        .map_err(|e| anyhow::anyhow!("delete role error: {}", e))
}

pub async fn update<C: ConnectionTrait>(
    db: &C,
    id: i32,
    name: Option<String>,
    path: Option<String>,
    is_frame: Option<bool>,
) -> Result<()> {
    MenuEntity::update(MenuActiveModel {
        id: Set(id),
        name: name.map(Set).unwrap_or(NotSet),
        path: path.map(Set).unwrap_or(NotSet),
        is_frame: is_frame.map(Set).unwrap_or(NotSet),
    })
    .exec(db)
    .await
    .map(|_| ())
    .map_err(|e| anyhow::anyhow!("update role error: {}", e))
}

pub async fn get<C: ConnectionTrait>(db: &C, id: i32) -> Result<Option<MenuModel>> {
    MenuEntity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| anyhow::anyhow!("get role error: {}", e))
}

pub async fn list<C: ConnectionTrait>(db: &C, page: u64, page_size: u64) -> Result<Vec<MenuModel>> {
    let offset = (page - 1) * page_size;
    MenuEntity::find()
        .order_by_asc(MenuColumn::Id)
        .offset(offset)
        .limit(page_size)
        .all(db)
        .await
        .map_err(|e| anyhow::anyhow!("list role error: {}", e))
}
