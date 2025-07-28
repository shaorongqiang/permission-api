use anyhow::Result;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Statement,
};
use uuid::Uuid;

use crate::entity::{OnlineActiveModel, OnlineColumn, OnlineEntity, OnlineModel};

pub fn generate_token() -> String {
    let uuid = Uuid::new_v4();
    let token = format!("{uuid}");
    token
}

pub async fn create<C: ConnectionTrait>(db: &C, user_id: i64) -> Result<OnlineModel> {
    loop {
        let token = generate_token();
        if OnlineEntity::find()
            .filter(OnlineColumn::Token.eq(&token))
            .one(db)
            .await?
            .is_none()
        {
            let online = OnlineEntity::insert(OnlineActiveModel {
                token: Set(token.clone()),
                user_id: Set(user_id),
            })
            .exec_with_returning(db)
            .await?;
            return Ok(online);
        }
    }
}

pub async fn delete<C: ConnectionTrait>(db: &C, token: &str) -> Result<()> {
    OnlineEntity::delete_by_id(token)
        .exec(db)
        .await
        .map(|_| ())
        .map_err(|e| anyhow::anyhow!("delete online error: {}", e))
}

pub async fn get<C: ConnectionTrait>(db: &C, token: &str) -> Result<Option<OnlineModel>> {
    OnlineEntity::find()
        .filter(OnlineColumn::Token.eq(token))
        .one(db)
        .await
        .map_err(|e| anyhow::anyhow!("get online error: {}", e))
}

pub async fn list<C: ConnectionTrait>(
    db: &C,
    page: u64,
    page_size: u64,
) -> Result<Vec<OnlineModel>> {
    let offset = (page - 1) * page_size;
    OnlineEntity::find()
        .order_by_asc(OnlineColumn::UserId)
        .offset(offset)
        .limit(page_size)
        .all(db)
        .await
        .map_err(|e| anyhow::anyhow!("list online error: {}", e))
}

pub async fn get_menu_path_by_token<C: ConnectionTrait>(
    db: &C,
    token: &str,
) -> Result<Vec<String>> {
    let sql = r#"
    SELECT m.path
    FROM online o
    left join user_role ur on ur.user_id = o.user_id
    left join role_menu rm on rm.role_id = ur.role_id
    left join menu m on m.id = rm.menu_id
    where o.token = $1
"#;

    let result = db
        .query_all(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            sql,
            vec![token.into()],
        ))
        .await?;

    let menus = result
        .into_iter()
        .filter_map(|row| row.try_get("", "path").ok())
        .collect();

    Ok(menus)
}

pub async fn is_admin_by_token<C: ConnectionTrait>(db: &C, token: &str) -> Result<bool> {
    let sql = r#"
    SELECT ur.role_id
    FROM online o
    left join user_role ur on ur.user_id = o.user_id
    where o.token = $1 and ur.role_id = 1
"#;

    let result = db
        .query_all(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            sql,
            vec![token.into()],
        ))
        .await?;

    Ok(!result.is_empty())
}
