use anyhow::Result;
use sea_orm::{
    ActiveValue::{NotSet, Set},
    ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect,
};

use crate::entity::{UserActiveModel, UserColumn, UserEntity, UserModel};

pub async fn create<C: ConnectionTrait>(db: &C, name: &str, password: &str) -> Result<UserModel> {
    UserEntity::insert(UserActiveModel {
        id: NotSet,
        name: Set(name.to_string()),
        password: Set(password.to_string()),
    })
    .exec_with_returning(db)
    .await
    .map_err(|e| anyhow::anyhow!("create user error: {}", e))
}

pub async fn delete<C: ConnectionTrait>(db: &C, id: i64) -> Result<()> {
    UserEntity::delete_by_id(id)
        .exec(db)
        .await
        .map(|_| ())
        .map_err(|e| anyhow::anyhow!("delete user error: {}", e))
}

pub async fn update<C: ConnectionTrait>(
    db: &C,
    id: i64,
    name: Option<String>,
    password: Option<String>,
) -> Result<()> {
    UserEntity::update(UserActiveModel {
        id: Set(id),
        name: name.map(Set).unwrap_or(NotSet),
        password: password.map(Set).unwrap_or(NotSet),
    })
    .exec(db)
    .await
    .map(|_| ())
    .map_err(|e| anyhow::anyhow!("update user error: {}", e))
}

pub async fn get<C: ConnectionTrait>(db: &C, id: i64) -> Result<Option<UserModel>> {
    UserEntity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| anyhow::anyhow!("get user error: {}", e))
}

pub async fn list<C: ConnectionTrait>(db: &C, page: u64, page_size: u64) -> Result<Vec<UserModel>> {
    let offset = (page - 1) * page_size;
    UserEntity::find()
        .order_by_asc(UserColumn::Id)
        .offset(offset)
        .limit(page_size)
        .all(db)
        .await
        .map_err(|e| anyhow::anyhow!("list user error: {}", e))
}

pub async fn get_by_username<C: ConnectionTrait>(
    db: &C,
    username: &str,
) -> Result<Option<UserModel>> {
    let user = UserEntity::find()
        .filter(UserColumn::Name.eq(username))
        .one(db)
        .await?;
    Ok(user)
}
