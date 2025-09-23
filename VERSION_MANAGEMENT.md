# 版本管理说明

本项目采用统一的版本管理策略，确保所有配置文件中的版本号保持一致。

## 版本文件

### 1. version.json

主版本配置文件，包含完整的版本信息：

```json
{
  "version": "0.1.0",
  "buildNumber": "1",
  "releaseDate": "2024-12-19",
  "description": "EasyDB - 数据库管理工具",
  "author": "shencangsheng",
  "repository": "https://github.com/shencangsheng/easydb_app",
  "license": "MIT"
}
```

### 2. package.json

前端项目的版本配置，与主版本保持一致。

### 3. src-tauri/Cargo.toml

Rust 后端的版本配置，与主版本保持一致。

### 4. src-tauri/tauri.conf.json

Tauri 应用的版本配置，与主版本保持一致。

### 5. src/utils/version.ts

版本工具函数，通过 Vite 环境变量获取版本信息供前端使用。

### 6. vite-plugins/version-plugin.ts

Vite 插件，在构建时从 version.json 读取版本信息并注入到环境变量中。

## 版本更新

### 自动更新

使用提供的脚本自动更新所有文件中的版本号：

```bash
# 更新到新版本
npm run version:update 0.2.0

# 或者直接使用node
node scripts/update-version.js 0.2.0
```

### 手动更新

如果需要手动更新版本，请确保以下文件中的版本号保持一致：

1. `version.json` - 主版本配置
2. `package.json` - 前端版本
3. `src-tauri/Cargo.toml` - Rust 后端版本
4. `src-tauri/tauri.conf.json` - Tauri 应用版本
5. `src/utils/version.ts` - 版本工具函数（可选，会自动从 version.json 读取）

## 版本号规范

采用语义化版本控制（Semantic Versioning）：

- 格式：`MAJOR.MINOR.PATCH`
- 示例：`1.0.0`, `1.1.0`, `1.1.1`

- **MAJOR**: 不兼容的 API 修改
- **MINOR**: 向下兼容的功能性新增
- **PATCH**: 向下兼容的问题修正

## 构建号

每次版本更新时，构建号会自动递增：

- 初始构建号：1
- 每次版本更新：构建号 +1

## 版本显示

版本信息在以下位置显示：

1. 关于页面 - 显示完整版本信息
2. 应用标题栏 - 显示版本号
3. 构建产物 - 包含版本信息

## 技术实现

### 前端版本获取

- 使用 Vite 插件在构建时注入版本环境变量
- 前端通过`import.meta.env.VITE_APP_VERSION`等环境变量获取版本信息
- 开发模式下自动从 version.json 读取最新版本信息

### 环境变量

- `VITE_APP_VERSION`: 版本号
- `VITE_APP_BUILD_NUMBER`: 构建号
- `VITE_APP_RELEASE_DATE`: 发布日期

## 发布流程

1. 更新版本号：

   ```bash
   npm run version:update 0.2.0
   ```

2. 提交更改：

   ```bash
   git add .
   git commit -m "Bump version to 0.2.0"
   ```

3. 创建标签：

   ```bash
   git tag v0.2.0
   git push origin v0.2.0
   ```

4. 构建发布：
   ```bash
   npm run tauri build
   ```

## 注意事项

- 确保所有配置文件中的版本号保持一致
- 版本更新后记得更新 CHANGELOG.md
- 发布前进行充分测试
- 遵循语义化版本控制规范

## 故障排除

### ES 模块问题

如果遇到`__dirname is not defined in ES module scope`错误，这是因为项目使用了 ES 模块。脚本已经更新为使用 ES 模块语法：

```javascript
import { fileURLToPath } from "url";
import { dirname } from "path";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
```

### 版本不一致问题

如果版本显示不一致，请：

1. 运行 `npm run version:check` 检查版本一致性
2. 确保 Vite 开发服务器已重启
3. 检查浏览器缓存是否已清除
