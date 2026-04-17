# Auth-Sphere 认证系统

## 🎯 项目概述
支持多主体的认证与会话管理系统
```
初始运行需配置 auth-sphere.toml 文件
因数据库采用sqlite, 会自动创建db.
```

### 支持的主体类型
- **会员（Member）**: 普通用户
- **社区运营（Community Staff）**: 社区管理人员  
- **平台运营（Platform Staff）**: 平台管理员

### 核心功能
✅ **三类主体独立登录** - 每类用户有独立的认证端点  
✅ **多设备支持** - 支持同一用户在多个设备同时登录  
✅ **多维凭证** - 支持 Password、OTP 验证码 + PassKey无密码登陆  
✅ **MFA** - 预留多因素认证接口  
✅ **全栈可演示** - 完整的 API + 图形化演示页面  
✅ **Rust 技术栈** - Actix-web + SQLx + SQLite + Redis


### 配置文件 auth-sphere.toml
```
[app]
name = "Auth-Sphere"

[server]
server = "localhost"
port = 8000

[logging]
level = "INFO"

[logging.console]
level = "DEBUG"

[logging.file]
path = "/home/xxx/logs/auth-sphere"
prefix = "auth-sphere.log"

[database]
path = "/home/xxx/auth-sphere.db?mode=rwc"
max_connections = 10

[redis]
host = "127.0.0.1"
port = 6379
auth = "root"
db = 0
timeout = 3
```