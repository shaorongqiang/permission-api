# Permission API

权限管理系统 - 基于Rust和SeaORM构建的现代化权限管理API

## 项目特性

- 🔐 基于角色的访问控制 (RBAC)
- 🚀 高性能异步API
- 🛡️ 安全的身份验证和授权
- 📊 完整的用户、角色、菜单管理
- 🧪 完整的单元测试覆盖
- 🐳 Docker支持
- 📝 自动生成的API文档

## 快速开始

### 使用Makefile（推荐）

```bash
# 显示所有可用命令
make help

# 安装依赖并构建项目
make install

# 运行测试
make test

# 运行应用
make run

# 开发模式运行（带调试日志）
make dev

# 代码格式化和检查
make fmt
make lint

# 完整构建流程
make all
```

### 手动构建

```bash
# 安装依赖
cargo build

# 运行测试
cargo test

# 运行应用
cargo run
```

## 项目结构

```
permission-api/
├── Cargo.toml              # 工作空间配置
├── Makefile                # 构建和开发工具
├── Dockerfile              # Docker配置
├── crates/
│   ├── app/               # 核心应用模块
│   │   ├── src/
│   │   │   ├── controller/ # API控制器
│   │   │   ├── entity/     # 数据库实体
│   │   │   ├── service/    # 业务逻辑
│   │   │   └── tests.rs    # 单元测试
│   │   └── Cargo.toml
│   └── utils/              # 工具模块
├── sql/                    # 数据库脚本
└── src/
    └── main.rs            # 应用入口
```

## 开发工具

### Makefile命令

#### 构建命令
- `make build` - 构建项目
- `make build-release` - 构建发布版本
- `make clean` - 清理构建文件

#### 测试命令
- `make test` - 运行所有测试
- `make test-app` - 运行app crate测试
- `make test-verbose` - 运行详细测试
- `make test-coverage` - 生成测试覆盖率报告

#### 代码质量命令
- `make check` - 代码检查
- `make fmt` - 格式化代码
- `make fmt-check` - 检查代码格式
- `make lint` - 运行clippy检查
- `make lint-fix` - 自动修复clippy问题

#### 运行命令
- `make run` - 运行应用
- `make dev` - 开发模式运行
- `make install` - 安装依赖

#### Docker命令
- `make docker-build` - 构建Docker镜像
- `make docker-run` - 运行Docker容器
- `make docker-clean` - 清理Docker镜像

#### 组合命令
- `make all` - 完整构建流程
- `make pre-commit` - 提交前检查
- `make ci` - CI检查

### 开发工具安装

```bash
# 安装所有开发工具
make install-tools
```

这将安装：
- `cargo-tarpaulin` - 测试覆盖率工具
- `cargo-audit` - 安全审计工具
- `cargo-outdated` - 依赖更新检查工具

## 测试

项目使用SQLite内存数据库进行单元测试，确保测试的快速性和隔离性。

```bash
# 运行所有测试
make test

# 运行app crate测试
make test-app

# 生成测试覆盖率报告
make test-coverage
```

### 测试覆盖范围

- ✅ 用户服务 (6个测试)
- ✅ 角色服务 (5个测试)
- ✅ 菜单服务 (5个测试)
- ✅ 数据库配置 (1个测试)

## Docker支持

```bash
# 构建Docker镜像
make docker-build

# 运行Docker容器
make docker-run

# 清理Docker镜像
make docker-clean
```

## 配置

### 数据库配置

项目支持多种数据库配置，默认使用SQLite进行开发。

### 环境变量

- `RUST_LOG` - 日志级别 (debug, info, warn, error)
- `DATABASE_URL` - 数据库连接URL

## API文档

启动应用后，访问以下地址查看API文档：

- Swagger UI: http://localhost:8080/swagger
- OpenAPI JSON: http://localhost:8080/apidoc/openapi.json

## 开发指南

### 代码规范

项目使用以下工具确保代码质量：

- `rustfmt` - 代码格式化
- `clippy` - 代码检查
- `cargo-audit` - 安全审计

### 提交前检查

```bash
# 运行完整的提交前检查
make pre-commit
```

这将执行：
1. 代码格式化
2. 代码检查
3. 运行测试

### CI/CD

项目包含CI配置，确保代码质量：

```bash
# 运行CI检查
make ci
```

## 许可证

本项目采用MIT许可证 - 详见 [LICENSE](LICENSE) 文件。

## 贡献

欢迎贡献代码！请确保：

1. 运行 `make pre-commit` 通过所有检查
2. 添加适当的测试
3. 更新相关文档

## 支持

如有问题或建议，请提交Issue或Pull Request。
