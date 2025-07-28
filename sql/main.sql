DROP TABLE IF EXISTS "role_menu";
DROP TABLE IF EXISTS "user_role";
DROP TABLE IF EXISTS "online";
DROP TABLE IF EXISTS "menu";
DROP TABLE IF EXISTS "role";
DROP TABLE IF EXISTS "user";

CREATE TABLE IF NOT EXISTS "user"
(
    id bigserial NOT NULL,
    name character varying(20) NOT NULL,
    password character varying(20) NOT NULL,
    PRIMARY KEY (id), UNIQUE (name)
);
COMMENT ON TABLE "user" IS '用户表';
COMMENT ON COLUMN "user".id IS '主键，自增';
COMMENT ON COLUMN "user".name IS '用户名';
COMMENT ON COLUMN "user".password IS '密码';

INSERT INTO "user" (id, name, password)VALUES (1, 'admin', 'admin123');
INSERT INTO "user" (id, name, password)VALUES (2, 'user', 'user123');

CREATE TABLE IF NOT EXISTS role
(
    id serial NOT NULL,
    name character varying(20) NOT NULL,
    data_scope smallint NOT NULL DEFAULT 1,
    status smallint NOT NULL DEFAULT 0,
    PRIMARY KEY (id)
);

COMMENT ON TABLE role IS '角色表';
COMMENT ON COLUMN role.id IS '主键，自增';
COMMENT ON COLUMN role.name IS '角色名称';
COMMENT ON COLUMN role.data_scope IS '数据范围（0：管理员权限 1：自定数据权限）';
COMMENT ON COLUMN role.status IS '角色状态（0正常 1停用）';

INSERT INTO "role" (id, name, data_scope, status) VALUES (1, 'admin', 0, 0);
INSERT INTO "role" (id, name, data_scope, status) VALUES (2, 'user', 1, 0);

CREATE TABLE IF NOT EXISTS menu
(
    id serial NOT NULL,
    name character varying(20) NOT NULL,
    path character varying(100) NOT NULL,
    is_frame boolean NOT NULL DEFAULT false,
    PRIMARY KEY (id)
);
COMMENT ON TABLE menu IS '菜单表';
COMMENT ON COLUMN menu.id IS '主键，自增';
COMMENT ON COLUMN menu.name IS '菜单名称';
COMMENT ON COLUMN menu.path IS '菜单路径';
COMMENT ON COLUMN menu.is_frame IS '是否为外链';

INSERT INTO menu(id, name, path, is_frame) VALUES (1, '管理用户', '/user/list', false);
INSERT INTO menu(id, name, path, is_frame) VALUES (2, '获取用户', '/user/get', false);
INSERT INTO menu(id, name, path, is_frame) VALUES (3, '新增用户', '/user/create', false);
INSERT INTO menu(id, name, path, is_frame) VALUES (4, '编辑用户', '/user/update', false);
INSERT INTO menu(id, name, path, is_frame) VALUES (5, '删除用户', '/user/delete', false);

INSERT INTO menu(id, name, path, is_frame) VALUES (6, '管理角色', '/role/list', false);
INSERT INTO menu(id, name, path, is_frame) VALUES (7, '获取角色', '/role/get', false);
INSERT INTO menu(id, name, path, is_frame) VALUES (8, '新增角色', '/role/create', false);
INSERT INTO menu(id, name, path, is_frame) VALUES (9, '编辑角色', '/role/update', false);
INSERT INTO menu(id, name, path, is_frame) VALUES (10, '删除角色', '/role/delete', false);

INSERT INTO menu(id, name, path, is_frame) VALUES (11, '管理菜单', '/menu/list', false);
INSERT INTO menu(id, name, path, is_frame) VALUES (12, '获取菜单', '/menu/get', false);
INSERT INTO menu(id, name, path, is_frame) VALUES (13, '新增菜单', '/menu/create', false);
INSERT INTO menu(id, name, path, is_frame) VALUES (14, '编辑菜单', '/menu/update', false);
INSERT INTO menu(id, name, path, is_frame) VALUES (15, '删除菜单', '/menu/delete', false);

INSERT INTO menu(id, name, path, is_frame) VALUES (16, '管理在线用户', '/online/list', false);
INSERT INTO menu(id, name, path, is_frame) VALUES (17, '获取在线用户', '/online/get', false);
INSERT INTO menu(id, name, path, is_frame) VALUES (18, '删除在线用户', '/online/delete', false);


CREATE TABLE IF NOT EXISTS user_role
(
    user_id bigint NOT NULL REFERENCES "user" (id),
    role_id bigint NOT NULL REFERENCES "role" (id),
    PRIMARY KEY (user_id, role_id)
);
COMMENT ON TABLE user_role IS '用户和角色关联表';
COMMENT ON COLUMN user_role.user_id IS '用户ID';
COMMENT ON COLUMN user_role.role_id IS '角色ID';

insert into "user_role" values (1, 1);
insert into "user_role" values (2, 2);

CREATE TABLE "role_menu"
(
    role_id bigint NOT NULL REFERENCES "role" (id),
    menu_id bigint NOT NULL REFERENCES "menu" (id),
    PRIMARY KEY (role_id, menu_id)
);
COMMENT ON TABLE "role_menu" IS '角色和菜单关联表';
COMMENT ON COLUMN "role_menu".role_id IS '角色ID';
COMMENT ON COLUMN "role_menu".menu_id IS '菜单ID';

insert into "role_menu" values (2, 1);
insert into "role_menu" values (2, 2);
insert into "role_menu" values (2, 3);
insert into "role_menu" values (2, 4);
insert into "role_menu" values (2, 5);
insert into "role_menu" values (2, 6);
insert into "role_menu" values (2, 7);
insert into "role_menu" values (2, 8);
insert into "role_menu" values (2, 9);
insert into "role_menu" values (2, 10);
insert into "role_menu" values (2, 11);
insert into "role_menu" values (2, 12);
insert into "role_menu" values (2, 13);
insert into "role_menu" values (2, 14);
insert into "role_menu" values (2, 15);
insert into "role_menu" values (2, 16);
insert into "role_menu" values (2, 17);
insert into "role_menu" values (2, 18);

CREATE TABLE IF NOT EXISTS online
(
    token character varying(50) NOT NULL,
    user_id bigint NOT NULL REFERENCES "user" (id),
    PRIMARY KEY (token)
);

COMMENT ON TABLE online IS '在线用户表';
COMMENT ON COLUMN online.token IS 'session id';
COMMENT ON COLUMN online.user_id IS '用户ID';
