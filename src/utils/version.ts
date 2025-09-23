// 版本信息工具函数
export interface VersionInfo {
  version: string;
  buildNumber: string;
  releaseDate: string;
  description: string;
  author: string;
  repository: string;
  license: string;
}

// 获取版本信息 - 使用环境变量或默认值
export const getVersionInfo = (): VersionInfo => {
  // 在构建时，Vite会将这些值注入到环境中
  const version = import.meta.env.VITE_APP_VERSION || "0.1.0";
  const buildNumber = import.meta.env.VITE_APP_BUILD_NUMBER || "1";
  const releaseDate = import.meta.env.VITE_APP_RELEASE_DATE || "2024-12-19";

  return {
    version,
    buildNumber,
    releaseDate,
    description: "EasyDB - 数据库管理工具",
    author: "shencangsheng",
    repository: "https://github.com/shencangsheng/easydb_app",
    license: "MIT",
  };
};

// 获取版本号
export const getVersion = (): string => {
  return getVersionInfo().version;
};

// 获取构建号
export const getBuildNumber = (): string => {
  return getVersionInfo().buildNumber;
};

// 获取完整版本信息字符串
export const getFullVersion = (): string => {
  const info = getVersionInfo();
  return `${info.version} (Build ${info.buildNumber})`;
};
