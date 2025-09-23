#!/usr/bin/env node

import { readFileSync, writeFileSync, existsSync } from "fs";
import { join, dirname } from "path";
import { fileURLToPath } from "url";

// 获取当前文件的目录路径（ES模块中的__dirname替代方案）
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// 获取命令行参数
const args = process.argv.slice(2);
const newVersion = args[0];

if (!newVersion) {
  console.error("请提供新版本号，例如: node scripts/update-version.js 0.2.0");
  process.exit(1);
}

// 验证版本号格式 (简单验证)
const versionRegex = /^\d+\.\d+\.\d+$/;
if (!versionRegex.test(newVersion)) {
  console.error("版本号格式不正确，请使用格式: x.y.z (例如: 0.2.0)");
  process.exit(1);
}

console.log(`正在更新版本号到 ${newVersion}...`);

// 更新 version.json
const versionJsonPath = join(__dirname, "..", "version.json");
try {
  const versionData = JSON.parse(readFileSync(versionJsonPath, "utf8"));
  versionData.version = newVersion;
  versionData.buildNumber = String(parseInt(versionData.buildNumber) + 1);
  versionData.releaseDate = new Date().toISOString().split("T")[0];

  writeFileSync(versionJsonPath, JSON.stringify(versionData, null, 2));
  console.log("✓ 已更新 version.json");
} catch (error) {
  console.error("✗ 更新 version.json 失败:", error.message);
}

// 更新 package.json
const packageJsonPath = join(__dirname, "..", "package.json");
try {
  const packageData = JSON.parse(readFileSync(packageJsonPath, "utf8"));
  packageData.version = newVersion;

  writeFileSync(packageJsonPath, JSON.stringify(packageData, null, 2));
  console.log("✓ 已更新 package.json");
} catch (error) {
  console.error("✗ 更新 package.json 失败:", error.message);
}

// 更新 Cargo.toml
const cargoTomlPath = join(__dirname, "..", "src-tauri", "Cargo.toml");
try {
  let cargoContent = readFileSync(cargoTomlPath, "utf8");
  cargoContent = cargoContent.replace(
    /^version = ".*"$/m,
    `version = "${newVersion}"`
  );

  writeFileSync(cargoTomlPath, cargoContent);
  console.log("✓ 已更新 Cargo.toml");
} catch (error) {
  console.error("✗ 更新 Cargo.toml 失败:", error.message);
}

// 更新 tauri.conf.json
const tauriConfPath = join(__dirname, "..", "src-tauri", "tauri.conf.json");
try {
  const tauriData = JSON.parse(readFileSync(tauriConfPath, "utf8"));
  tauriData.version = newVersion;

  writeFileSync(tauriConfPath, JSON.stringify(tauriData, null, 2));
  console.log("✓ 已更新 tauri.conf.json");
} catch (error) {
  console.error("✗ 更新 tauri.conf.json 失败:", error.message);
}

// 更新 .env 文件（如果存在）
const envPath = join(__dirname, "..", ".env");
try {
  let envContent = "";
  if (existsSync(envPath)) {
    envContent = readFileSync(envPath, "utf8");
  }

  const versionData = JSON.parse(readFileSync(versionJsonPath, "utf8"));

  // 添加或更新版本环境变量
  const versionLines = [
    `VITE_APP_VERSION=${versionData.version}`,
    `VITE_APP_BUILD_NUMBER=${versionData.buildNumber}`,
    `VITE_APP_RELEASE_DATE=${versionData.releaseDate}`,
  ];

  // 移除旧的版本行
  envContent = envContent.replace(/VITE_APP_VERSION=.*\n?/g, "");
  envContent = envContent.replace(/VITE_APP_BUILD_NUMBER=.*\n?/g, "");
  envContent = envContent.replace(/VITE_APP_RELEASE_DATE=.*\n?/g, "");

  // 添加新的版本行
  if (envContent && !envContent.endsWith("\n")) {
    envContent += "\n";
  }
  envContent += versionLines.join("\n") + "\n";

  writeFileSync(envPath, envContent);
  console.log("✓ 已更新 .env 文件");
} catch (error) {
  console.error("✗ 更新 .env 文件失败:", error.message);
}

console.log(`\n🎉 版本更新完成！所有文件已更新到版本 ${newVersion}`);
console.log("\n建议执行以下命令来验证更新:");
console.log("  git add .");
console.log(`  git commit -m "Bump version to ${newVersion}"`);
console.log(`  git tag v${newVersion}`);
