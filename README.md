# LibreService
ModernLibre 的基础业务服务

## 环境变量相关

### 本地单一服务的开发环境
- 检查环境变量 `KUBERNETES_SERVICE` ，不存在或者不为 True，则使用 `.env` 文件注入环境变量，使用 `casdoorConfig` 注入 casdoor 配置

### 容器环境
- 使用 Docker Compose 管理多个容器和服务


### 集群环境
- 通过指定 namespace 来区分不同环境，部署时自动注入对应的 ConfigMap 和 Secret（在其中指定归属的 namespace）
- ConfigMap 注入环境变量配置
- Secret 注入敏感信息配置置
- secret-xxx 注入 xxx 版本的敏感信息配置
- `KUBERNETES_SERVICE` 为 True 时，忽略 `.env` 的注入避免覆盖