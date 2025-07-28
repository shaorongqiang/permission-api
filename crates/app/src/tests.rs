use anyhow::Result;
use sea_orm::{ConnectionTrait, DatabaseConnection, Schema};

use crate::{
    entity::{DatabaseConfig, MenuEntity, RoleEntity, UserEntity},
    service::{menu, role, user},
};

/// 创建内存数据库连接并初始化所有表结构
async fn create_test_db() -> Result<DatabaseConnection> {
    let config = DatabaseConfig::default_with_url("sqlite::memory:");
    let db = crate::entity::db_connect(&config).await?;

    // 创建所有表结构
    let schema = Schema::new(db.get_database_backend());

    // 创建用户表
    let create_user_table = schema.create_table_from_entity(UserEntity);
    db.execute(db.get_database_backend().build(&create_user_table))
        .await?;

    // 创建角色表
    let create_role_table = schema.create_table_from_entity(RoleEntity);
    db.execute(db.get_database_backend().build(&create_role_table))
        .await?;

    // 创建菜单表
    let create_menu_table = schema.create_table_from_entity(MenuEntity);
    db.execute(db.get_database_backend().build(&create_menu_table))
        .await?;

    Ok(db)
}

// ==================== 用户服务测试 ====================

#[tokio::test]
async fn test_create_user() -> Result<()> {
    let db = create_test_db().await?;

    // 测试创建用户
    let user = user::create(&db, "test_user", "test_password").await?;
    assert_eq!(user.name, "test_user");
    assert_eq!(user.password, "test_password");

    // 验证用户已保存到数据库
    let saved_user = user::get(&db, user.id).await?;
    assert!(saved_user.is_some());
    let saved_user = saved_user.unwrap();
    assert_eq!(saved_user.name, "test_user");
    assert_eq!(saved_user.password, "test_password");

    Ok(())
}

#[tokio::test]
async fn test_get_user() -> Result<()> {
    let db = create_test_db().await?;

    // 创建用户
    let created_user = user::create(&db, "test_user", "test_password").await?;

    // 测试获取用户
    let user = user::get(&db, created_user.id).await?;
    assert!(user.is_some());
    let user = user.unwrap();
    assert_eq!(user.name, "test_user");
    assert_eq!(user.password, "test_password");

    // 测试获取不存在的用户
    let non_existent_user = user::get(&db, 999).await?;
    assert!(non_existent_user.is_none());

    Ok(())
}

#[tokio::test]
async fn test_update_user() -> Result<()> {
    let db = create_test_db().await?;

    // 创建用户
    let created_user = user::create(&db, "test_user", "test_password").await?;

    // 测试更新用户名
    user::update(&db, created_user.id, Some("updated_user".to_string()), None).await?;

    // 验证更新结果
    let updated_user = user::get(&db, created_user.id).await?;
    assert!(updated_user.is_some());
    let updated_user = updated_user.unwrap();
    assert_eq!(updated_user.name, "updated_user");
    assert_eq!(updated_user.password, "test_password"); // 密码应该保持不变

    // 测试更新密码
    user::update(&db, created_user.id, None, Some("new_password".to_string())).await?;

    // 验证密码更新
    let updated_user = user::get(&db, created_user.id).await?;
    assert!(updated_user.is_some());
    let updated_user = updated_user.unwrap();
    assert_eq!(updated_user.name, "updated_user");
    assert_eq!(updated_user.password, "new_password");

    Ok(())
}

#[tokio::test]
async fn test_delete_user() -> Result<()> {
    let db = create_test_db().await?;

    // 创建用户
    let created_user = user::create(&db, "test_user", "test_password").await?;

    // 验证用户存在
    let user = user::get(&db, created_user.id).await?;
    assert!(user.is_some());

    // 删除用户
    user::delete(&db, created_user.id).await?;

    // 验证用户已被删除
    let deleted_user = user::get(&db, created_user.id).await?;
    assert!(deleted_user.is_none());

    Ok(())
}

#[tokio::test]
async fn test_list_users() -> Result<()> {
    let db = create_test_db().await?;

    // 创建多个用户
    user::create(&db, "user1", "password1").await?;
    user::create(&db, "user2", "password2").await?;
    user::create(&db, "user3", "password3").await?;

    // 测试分页查询
    let users = user::list(&db, 1, 2).await?;
    assert_eq!(users.len(), 2);

    let users = user::list(&db, 2, 2).await?;
    assert_eq!(users.len(), 1);

    Ok(())
}

#[tokio::test]
async fn test_get_by_username() -> Result<()> {
    let db = create_test_db().await?;

    // 创建用户
    user::create(&db, "test_user", "test_password").await?;

    // 测试通过用户名查找
    let user = user::get_by_username(&db, "test_user").await?;
    assert!(user.is_some());
    let user = user.unwrap();
    assert_eq!(user.name, "test_user");
    assert_eq!(user.password, "test_password");

    // 测试查找不存在的用户名
    let non_existent_user = user::get_by_username(&db, "non_existent").await?;
    assert!(non_existent_user.is_none());

    Ok(())
}

// ==================== 角色服务测试 ====================

#[tokio::test]
async fn test_create_role() -> Result<()> {
    let db = create_test_db().await?;

    // 测试创建角色
    let role = role::create(&db, "admin", 1, 1).await?;
    assert_eq!(role.name, "admin");
    assert_eq!(role.data_scope, 1);
    assert_eq!(role.status, 1);

    // 验证角色已保存到数据库
    let saved_role = role::get(&db, role.id).await?;
    assert!(saved_role.is_some());
    let saved_role = saved_role.unwrap();
    assert_eq!(saved_role.name, "admin");
    assert_eq!(saved_role.data_scope, 1);
    assert_eq!(saved_role.status, 1);

    Ok(())
}

#[tokio::test]
async fn test_get_role() -> Result<()> {
    let db = create_test_db().await?;

    // 创建角色
    let created_role = role::create(&db, "admin", 1, 1).await?;

    // 测试获取角色
    let role = role::get(&db, created_role.id).await?;
    assert!(role.is_some());
    let role = role.unwrap();
    assert_eq!(role.name, "admin");
    assert_eq!(role.data_scope, 1);
    assert_eq!(role.status, 1);

    // 测试获取不存在的角色
    let non_existent_role = role::get(&db, 999).await?;
    assert!(non_existent_role.is_none());

    Ok(())
}

#[tokio::test]
async fn test_update_role() -> Result<()> {
    let db = create_test_db().await?;

    // 创建角色
    let created_role = role::create(&db, "admin", 1, 1).await?;

    // 测试更新角色名
    role::update(
        &db,
        created_role.id,
        Some("super_admin".to_string()),
        None,
        None,
    )
    .await?;

    // 验证更新结果
    let updated_role = role::get(&db, created_role.id).await?;
    assert!(updated_role.is_some());
    let updated_role = updated_role.unwrap();
    assert_eq!(updated_role.name, "super_admin");
    assert_eq!(updated_role.data_scope, 1); // 数据范围应该保持不变
    assert_eq!(updated_role.status, 1); // 状态应该保持不变

    // 测试更新数据范围和状态
    role::update(&db, created_role.id, None, Some(2), Some(0)).await?;

    // 验证更新
    let updated_role = role::get(&db, created_role.id).await?;
    assert!(updated_role.is_some());
    let updated_role = updated_role.unwrap();
    assert_eq!(updated_role.name, "super_admin");
    assert_eq!(updated_role.data_scope, 2);
    assert_eq!(updated_role.status, 0);

    Ok(())
}

#[tokio::test]
async fn test_delete_role() -> Result<()> {
    let db = create_test_db().await?;

    // 创建角色
    let created_role = role::create(&db, "admin", 1, 1).await?;

    // 验证角色存在
    let role = role::get(&db, created_role.id).await?;
    assert!(role.is_some());

    // 删除角色
    role::delete(&db, created_role.id).await?;

    // 验证角色已被删除
    let deleted_role = role::get(&db, created_role.id).await?;
    assert!(deleted_role.is_none());

    Ok(())
}

#[tokio::test]
async fn test_list_roles() -> Result<()> {
    let db = create_test_db().await?;

    // 创建多个角色
    role::create(&db, "admin", 1, 1).await?;
    role::create(&db, "user", 2, 1).await?;
    role::create(&db, "guest", 3, 0).await?;

    // 测试分页查询
    let roles = role::list(&db, 1, 2).await?;
    assert_eq!(roles.len(), 2);

    let roles = role::list(&db, 2, 2).await?;
    assert_eq!(roles.len(), 1);

    Ok(())
}

// ==================== 菜单服务测试 ====================

#[tokio::test]
async fn test_create_menu() -> Result<()> {
    let db = create_test_db().await?;

    // 测试创建菜单
    let menu = menu::create(&db, "用户管理", "/users", false).await?;
    assert_eq!(menu.name, "用户管理");
    assert_eq!(menu.path, "/users");
    assert_eq!(menu.is_frame, false);

    // 验证菜单已保存到数据库
    let saved_menu = menu::get(&db, menu.id).await?;
    assert!(saved_menu.is_some());
    let saved_menu = saved_menu.unwrap();
    assert_eq!(saved_menu.name, "用户管理");
    assert_eq!(saved_menu.path, "/users");
    assert_eq!(saved_menu.is_frame, false);

    Ok(())
}

#[tokio::test]
async fn test_get_menu() -> Result<()> {
    let db = create_test_db().await?;

    // 创建菜单
    let created_menu = menu::create(&db, "用户管理", "/users", false).await?;

    // 测试获取菜单
    let menu = menu::get(&db, created_menu.id).await?;
    assert!(menu.is_some());
    let menu = menu.unwrap();
    assert_eq!(menu.name, "用户管理");
    assert_eq!(menu.path, "/users");
    assert_eq!(menu.is_frame, false);

    // 测试获取不存在的菜单
    let non_existent_menu = menu::get(&db, 999).await?;
    assert!(non_existent_menu.is_none());

    Ok(())
}

#[tokio::test]
async fn test_update_menu() -> Result<()> {
    let db = create_test_db().await?;

    // 创建菜单
    let created_menu = menu::create(&db, "用户管理", "/users", false).await?;

    // 测试更新菜单名
    menu::update(
        &db,
        created_menu.id,
        Some("用户列表".to_string()),
        None,
        None,
    )
    .await?;

    // 验证更新结果
    let updated_menu = menu::get(&db, created_menu.id).await?;
    assert!(updated_menu.is_some());
    let updated_menu = updated_menu.unwrap();
    assert_eq!(updated_menu.name, "用户列表");
    assert_eq!(updated_menu.path, "/users"); // 路径应该保持不变
    assert_eq!(updated_menu.is_frame, false); // is_frame应该保持不变

    // 测试更新路径和is_frame
    menu::update(
        &db,
        created_menu.id,
        None,
        Some("/user/list".to_string()),
        Some(true),
    )
    .await?;

    // 验证更新
    let updated_menu = menu::get(&db, created_menu.id).await?;
    assert!(updated_menu.is_some());
    let updated_menu = updated_menu.unwrap();
    assert_eq!(updated_menu.name, "用户列表");
    assert_eq!(updated_menu.path, "/user/list");
    assert_eq!(updated_menu.is_frame, true);

    Ok(())
}

#[tokio::test]
async fn test_delete_menu() -> Result<()> {
    let db = create_test_db().await?;

    // 创建菜单
    let created_menu = menu::create(&db, "用户管理", "/users", false).await?;

    // 验证菜单存在
    let menu = menu::get(&db, created_menu.id).await?;
    assert!(menu.is_some());

    // 删除菜单
    menu::delete(&db, created_menu.id).await?;

    // 验证菜单已被删除
    let deleted_menu = menu::get(&db, created_menu.id).await?;
    assert!(deleted_menu.is_none());

    Ok(())
}

#[tokio::test]
async fn test_list_menus() -> Result<()> {
    let db = create_test_db().await?;

    // 创建多个菜单
    menu::create(&db, "用户管理", "/users", false).await?;
    menu::create(&db, "角色管理", "/roles", false).await?;
    menu::create(&db, "菜单管理", "/menus", false).await?;

    // 测试分页查询
    let menus = menu::list(&db, 1, 2).await?;
    assert_eq!(menus.len(), 2);

    let menus = menu::list(&db, 2, 2).await?;
    assert_eq!(menus.len(), 1);

    Ok(())
}

// ==================== 数据库配置测试 ====================

#[tokio::test]
async fn test_database_config() -> Result<()> {
    // 测试默认配置
    let config = DatabaseConfig::default_with_url("sqlite::memory:");
    assert_eq!(config.url, "sqlite::memory:");
    assert_eq!(config.enable_logging, false);

    // 测试数据库连接
    let db = crate::entity::db_connect(&config).await?;
    assert!(db.ping().await.is_ok());

    Ok(())
}
