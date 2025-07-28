# Permission API Makefile
# 权限管理系统构建和开发工具

.PHONY: help build test clean check fmt lint doc install dev run docker-build docker-run

# 默认目标
.DEFAULT_GOAL := help

# 颜色定义
RED := \033[0;31m
GREEN := \033[0;32m
YELLOW := \033[0;33m
BLUE := \033[0;34m
PURPLE := \033[0;35m
CYAN := \033[0;36m
WHITE := \033[0;37m
RESET := \033[0m

# 项目信息
PROJECT_NAME := permission-api
APP_CRATE := crates/app
VERSION := $(shell grep '^version =' Cargo.toml | cut -d'"' -f2)

# 帮助信息
help: ## 显示帮助信息
	@echo "$(CYAN)$(PROJECT_NAME) - 权限管理系统$(RESET)"
	@echo "$(YELLOW)版本: $(VERSION)$(RESET)"
	@echo ""
	@echo "$(GREEN)可用命令:$(RESET)"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  $(YELLOW)%-15s$(RESET) %s\n", $$1, $$2}' $(MAKEFILE_LIST)
	@echo ""
	@echo "$(BLUE)开发命令:$(RESET)"
	@echo "  make build      - 构建项目"
	@echo "  make test       - 运行测试"
	@echo "  make check      - 代码检查"
	@echo "  make fmt        - 格式化代码"
	@echo "  make lint       - 代码检查"
	@echo "  make clean      - 清理构建文件"
	@echo ""
	@echo "$(BLUE)运行命令:$(RESET)"
	@echo "  make run        - 运行应用"
	@echo "  make dev        - 开发模式运行"
	@echo "  make install    - 安装依赖"
	@echo ""
	@echo "$(BLUE)Docker命令:$(RESET)"
	@echo "  make docker-build - 构建Docker镜像"
	@echo "  make docker-run   - 运行Docker容器"

# ==================== 构建命令 ====================

build: ## 构建项目
	@echo "$(GREEN)构建项目...$(RESET)"
	cargo build
	@echo "$(GREEN)构建完成!$(RESET)"

build-release: ## 构建发布版本
	@echo "$(GREEN)构建发布版本...$(RESET)"
	cargo build --release
	@echo "$(GREEN)发布版本构建完成!$(RESET)"

# ==================== 测试命令 ====================

test: ## 运行所有测试
	@echo "$(GREEN)运行测试...$(RESET)"
	cargo test
	@echo "$(GREEN)测试完成!$(RESET)"

test-app: ## 运行app crate测试
	@echo "$(GREEN)运行app crate测试...$(RESET)"
	cd $(APP_CRATE) && cargo test
	@echo "$(GREEN)app crate测试完成!$(RESET)"

test-verbose: ## 运行测试并显示详细输出
	@echo "$(GREEN)运行详细测试...$(RESET)"
	cargo test -- --nocapture
	@echo "$(GREEN)详细测试完成!$(RESET)"

test-coverage: ## 生成测试覆盖率报告
	@echo "$(GREEN)生成测试覆盖率报告...$(RESET)"
	@if ! command -v cargo-tarpaulin &> /dev/null; then \
		echo "$(YELLOW)安装cargo-tarpaulin...$(RESET)"; \
		cargo install cargo-tarpaulin; \
	fi
	cargo tarpaulin --out Html
	@echo "$(GREEN)覆盖率报告已生成到target/tarpaulin/html/index.html$(RESET)"

# ==================== 代码质量命令 ====================

check: ## 代码检查
	@echo "$(GREEN)运行代码检查...$(RESET)"
	cargo check
	@echo "$(GREEN)代码检查完成!$(RESET)"

fmt: ## 格式化代码
	@echo "$(GREEN)格式化代码...$(RESET)"
	cargo fmt
	@echo "$(GREEN)代码格式化完成!$(RESET)"

fmt-check: ## 检查代码格式
	@echo "$(GREEN)检查代码格式...$(RESET)"
	cargo fmt -- --check
	@echo "$(GREEN)代码格式检查完成!$(RESET)"

lint: ## 运行clippy检查
	@echo "$(GREEN)运行clippy检查...$(RESET)"
	cargo clippy
	@echo "$(GREEN)clippy检查完成!$(RESET)"

lint-fix: ## 运行clippy并自动修复
	@echo "$(GREEN)运行clippy自动修复...$(RESET)"
	cargo clippy --fix
	@echo "$(GREEN)clippy自动修复完成!$(RESET)"

# ==================== 清理命令 ====================

clean: ## 清理构建文件
	@echo "$(GREEN)清理构建文件...$(RESET)"
	cargo clean
	@echo "$(GREEN)清理完成!$(RESET)"

clean-all: ## 清理所有文件（包括target和Cargo.lock）
	@echo "$(RED)清理所有文件...$(RESET)"
	cargo clean
	rm -f Cargo.lock
	@echo "$(GREEN)完全清理完成!$(RESET)"

# ==================== 文档命令 ====================

doc: ## 生成文档
	@echo "$(GREEN)生成文档...$(RESET)"
	cargo doc --no-deps
	@echo "$(GREEN)文档生成完成!$(RESET)"

doc-open: ## 生成并打开文档
	@echo "$(GREEN)生成并打开文档...$(RESET)"
	cargo doc --no-deps --open
	@echo "$(GREEN)文档已打开!$(RESET)"

# ==================== 安装和依赖命令 ====================

install: ## 安装项目依赖
	@echo "$(GREEN)安装项目依赖...$(RESET)"
	cargo build
	@echo "$(GREEN)依赖安装完成!$(RESET)"

install-tools: ## 安装开发工具
	@echo "$(GREEN)安装开发工具...$(RESET)"
	@if ! command -v cargo-tarpaulin &> /dev/null; then \
		echo "$(YELLOW)安装cargo-tarpaulin...$(RESET)"; \
		cargo install cargo-tarpaulin; \
	fi
	@if ! command -v cargo-audit &> /dev/null; then \
		echo "$(YELLOW)安装cargo-audit...$(RESET)"; \
		cargo install cargo-audit; \
	fi
	@if ! command -v cargo-outdated &> /dev/null; then \
		echo "$(YELLOW)安装cargo-outdated...$(RESET)"; \
		cargo install cargo-outdated; \
	fi
	@echo "$(GREEN)开发工具安装完成!$(RESET)"

# ==================== 运行命令 ====================

run: ## 运行应用
	@echo "$(GREEN)运行应用...$(RESET)"
	cargo run
	@echo "$(GREEN)应用运行完成!$(RESET)"

dev: ## 开发模式运行（带日志）
	@echo "$(GREEN)开发模式运行...$(RESET)"
	RUST_LOG=debug cargo run
	@echo "$(GREEN)开发模式运行完成!$(RESET)"

# ==================== 安全命令 ====================

audit: ## 安全审计
	@echo "$(GREEN)运行安全审计...$(RESET)"
	@if command -v cargo-audit &> /dev/null; then \
		cargo audit; \
	else \
		echo "$(YELLOW)cargo-audit未安装，跳过安全审计$(RESET)"; \
	fi

outdated: ## 检查过时的依赖
	@echo "$(GREEN)检查过时的依赖...$(RESET)"
	@if command -v cargo-outdated &> /dev/null; then \
		cargo outdated; \
	else \
		echo "$(YELLOW)cargo-outdated未安装，跳过依赖检查$(RESET)"; \
	fi

# ==================== Docker命令 ====================

docker-build: ## 构建Docker镜像
	@echo "$(GREEN)构建Docker镜像...$(RESET)"
	docker build -t $(PROJECT_NAME):$(VERSION) .
	@echo "$(GREEN)Docker镜像构建完成!$(RESET)"

docker-run: ## 运行Docker容器
	@echo "$(GREEN)运行Docker容器...$(RESET)"
	docker run -p 8080:8080 $(PROJECT_NAME):$(VERSION)
	@echo "$(GREEN)Docker容器运行完成!$(RESET)"

docker-clean: ## 清理Docker镜像
	@echo "$(GREEN)清理Docker镜像...$(RESET)"
	docker rmi $(PROJECT_NAME):$(VERSION) 2>/dev/null || true
	@echo "$(GREEN)Docker镜像清理完成!$(RESET)"

# ==================== 数据库命令 ====================

db-setup: ## 设置数据库
	@echo "$(GREEN)设置数据库...$(RESET)"
	@if [ -f "sql/main.sql" ]; then \
		echo "$(YELLOW)数据库SQL文件存在$(RESET)"; \
	else \
		echo "$(RED)数据库SQL文件不存在$(RESET)"; \
	fi

# ==================== 发布命令 ====================

release: ## 发布版本
	@echo "$(GREEN)发布版本...$(RESET)"
	@echo "$(YELLOW)当前版本: $(VERSION)$(RESET)"
	@read -p "输入新版本号: " new_version; \
	sed -i '' 's/version = ".*"/version = "'$$new_version'"/' Cargo.toml; \
	echo "$(GREEN)版本已更新为: $$new_version$(RESET)"

# ==================== 组合命令 ====================

all: clean build test ## 完整构建流程：清理、构建、测试
	@echo "$(GREEN)完整构建流程完成!$(RESET)"

pre-commit: fmt lint test ## 提交前检查：格式化、检查、测试
	@echo "$(GREEN)提交前检查完成!$(RESET)"

ci: fmt-check lint test ## CI检查：格式检查、代码检查、测试
	@echo "$(GREEN)CI检查完成!$(RESET)"

# ==================== 信息命令 ====================

info: ## 显示项目信息
	@echo "$(CYAN)项目信息:$(RESET)"
	@echo "  名称: $(PROJECT_NAME)"
	@echo "  版本: $(VERSION)"
	@echo "  路径: $(shell pwd)"
	@echo "  Rust版本: $(shell rustc --version)"
	@echo "  Cargo版本: $(shell cargo --version)"

status: ## 显示项目状态
	@echo "$(CYAN)项目状态:$(RESET)"
	@echo "  构建状态: $(shell if [ -d "target" ]; then echo "$(GREEN)已构建$(RESET)"; else echo "$(RED)未构建$(RESET)"; fi)"
	@echo "  测试状态: $(shell if [ -d "target/debug" ]; then echo "$(GREEN)可运行$(RESET)"; else echo "$(RED)不可运行$(RESET)"; fi)"
	@echo "  文档状态: $(shell if [ -d "target/doc" ]; then echo "$(GREEN)已生成$(RESET)"; else echo "$(RED)未生成$(RESET)"; fi)" 