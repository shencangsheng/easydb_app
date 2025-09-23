#!/usr/bin/env node

import { readFileSync } from "fs";
import { join, dirname } from "path";
import { fileURLToPath } from "url";

// 获取当前文件的目录路径（ES模块中的__dirname替代方案）
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

console.log("🔍 检查版本一致性...\n");

// 检查各个文件中的版本号
const files = [
  { path: "version.json", key: "version" },
  { path: "package.json", key: "version" },
  {
    path: "src-tauri/Cargo.toml",
    key: "version",
    regex: /^version = "(.*)"$/m,
  },
  { path: "src-tauri/tauri.conf.json", key: "version" },
];

const versions = {};

files.forEach((file) => {
  try {
    const filePath = join(__dirname, "..", file.path);
    const content = readFileSync(filePath, "utf8");

    let version;
    if (file.regex) {
      const match = content.match(file.regex);
      version = match ? match[1] : null;
    } else {
      const data = JSON.parse(content);
      version = data[file.key];
    }

    versions[file.path] = version;
    console.log(`✓ ${file.path}: ${version}`);
  } catch (error) {
    console.log(`✗ ${file.path}: 读取失败 - ${error.message}`);
    versions[file.path] = null;
  }
});

// 检查版本一致性
const uniqueVersions = [
  ...new Set(Object.values(versions).filter((v) => v !== null)),
];

if (uniqueVersions.length === 1) {
  console.log(`\n🎉 所有文件版本一致: ${uniqueVersions[0]}`);
} else {
  console.log("\n⚠️  版本不一致:");
  Object.entries(versions).forEach(([file, version]) => {
    console.log(`  ${file}: ${version || "N/A"}`);
  });
}

console.log("\n📋 版本管理文件:");
console.log("  - version.json: 主版本配置");
console.log("  - package.json: 前端版本");
console.log("  - src-tauri/Cargo.toml: Rust后端版本");
console.log("  - src-tauri/tauri.conf.json: Tauri应用版本");
console.log("  - src/utils/version.ts: 版本工具函数");

console.log("\n🚀 使用方法:");
console.log("  npm run version:update 0.2.0  # 更新到新版本");
console.log("  npm run version:check         # 检查版本一致性");
console.log("  node scripts/test-version.js   # 检查版本一致性");

console.log("\n💡 提示:");
console.log("  - 版本信息通过Vite插件注入到前端应用");
console.log("  - 开发模式下会从version.json读取版本信息");
console.log("  - 构建时会自动注入正确的版本环境变量");
