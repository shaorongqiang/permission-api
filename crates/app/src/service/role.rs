use anyhow::Result;
use sea_orm::{
    ActiveValue::{NotSet, Set},
    ConnectionTrait, EntityTrait, QueryOrder, QuerySelect,
};

use crate::entity::{RoleActiveModel, RoleColumn, RoleEntity, RoleModel};

pub async fn create<C: ConnectionTrait>(
    db: &C,
    name: &str,
    data_scope: i16,
    status: i16,
) -> Result<RoleModel> {
    RoleEntity::insert(RoleActiveModel {
        id: NotSet,
        name: Set(name.to_string()),
        data_scope: Set(data_scope),
        status: Set(status),
    })
    .exec_with_returning(db)
    .await
    .map_err(|e| anyhow::anyhow!("create role error: {}", e))
}

pub async fn delete<C: ConnectionTrait>(db: &C, id: i32) -> Result<()> {
    RoleEntity::delete_by_id(id)
        .exec(db)
        .await
        .map(|_| ())
        .map_err(|e| anyhow::anyhow!("delete role error: {}", e))
}

pub async fn update<C: ConnectionTrait>(
    db: &C,
    id: i32,
    name: Option<String>,
    data_scope: Option<i16>,
    status: Option<i16>,
) -> Result<()> {
    RoleEntity::update(RoleActiveModel {
        id: Set(id),
        name: name.map(Set).unwrap_or(NotSet),
        data_scope: data_scope.map(Set).unwrap_or(NotSet),
        status: status.map(Set).unwrap_or(NotSet),
    })
    .exec(db)
    .await
    .map(|_| ())
    .map_err(|e| anyhow::anyhow!("update role error: {}", e))
}

pub async fn get<C: ConnectionTrait>(db: &C, id: i32) -> Result<Option<RoleModel>> {
    RoleEntity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| anyhow::anyhow!("get role error: {}", e))
}

pub async fn list<C: ConnectionTrait>(db: &C, page: u64, page_size: u64) -> Result<Vec<RoleModel>> {
    let offset = (page - 1) * page_size;
    RoleEntity::find()
        .order_by_asc(RoleColumn::Id)
        .offset(offset)
        .limit(page_size)
        .all(db)
        .await
        .map_err(|e| anyhow::anyhow!("list role error: {}", e))
}
