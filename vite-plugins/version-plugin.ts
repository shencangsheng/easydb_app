import { Plugin } from "vite";
import fs from "fs";
import path from "path";

export function versionPlugin(): Plugin {
  return {
    name: "version-plugin",
    config(config, { command }) {
      try {
        // 读取version.json
        const versionPath = path.resolve(__dirname, "../version.json");
        const versionData = JSON.parse(fs.readFileSync(versionPath, "utf8"));

        console.log(
          `📦 注入版本信息: ${versionData.version} (Build ${versionData.buildNumber})`
        );

        // 注入环境变量
        config.define = {
          ...config.define,
          "import.meta.env.VITE_APP_VERSION": JSON.stringify(
            versionData.version
          ),
          "import.meta.env.VITE_APP_BUILD_NUMBER": JSON.stringify(
            versionData.buildNumber
          ),
          "import.meta.env.VITE_APP_RELEASE_DATE": JSON.stringify(
            versionData.releaseDate
          ),
        };
      } catch (error) {
        console.warn("无法读取version.json，使用默认版本信息");
        config.define = {
          ...config.define,
          "import.meta.env.VITE_APP_VERSION": JSON.stringify("0.1.0"),
          "import.meta.env.VITE_APP_BUILD_NUMBER": JSON.stringify("1"),
          "import.meta.env.VITE_APP_RELEASE_DATE": JSON.stringify("2024-12-19"),
        };
      }
    },
  };
}
