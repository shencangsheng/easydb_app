// Version information utility functions
export interface VersionInfo {
  version: string;
  buildNumber: string;
  releaseDate: string;
  description: string;
  author: string;
  repository: string;
  license: string;
}

// Get version information - using environment variables or defaults
export const getVersionInfo = (): VersionInfo => {
  // At build time, Vite injects these values into the environment
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

// Get version number
export const getVersion = (): string => {
  return getVersionInfo().version;
};

// Get build number
export const getBuildNumber = (): string => {
  return getVersionInfo().buildNumber;
};

// Get full version information string
export const getFullVersion = (): string => {
  const info = getVersionInfo();
  return `${info.version} (Build ${info.buildNumber})`;
};

// Compare version numbers (semantic versioning)
// Returns: 1 if version1 > version2, -1 if version1 < version2, 0 if equal
export const compareVersions = (version1: string, version2: string): number => {
  // Remove 'v' prefix if present
  const v1 = version1.replace(/^v/i, "");
  const v2 = version2.replace(/^v/i, "");

  const parts1 = v1.split(".").map(Number);
  const parts2 = v2.split(".").map(Number);

  // Ensure both version numbers have 3 parts (major.minor.patch)
  while (parts1.length < 3) parts1.push(0);
  while (parts2.length < 3) parts2.push(0);

  for (let i = 0; i < 3; i++) {
    if (parts1[i] > parts2[i]) return 1;
    if (parts1[i] < parts2[i]) return -1;
  }

  return 0;
};

// GitHub Release information interface
export interface GitHubRelease {
  tag_name: string;
  name: string;
  html_url: string;
  body: string;
  published_at: string;
}

// Fetch latest release from GitHub API
export const fetchLatestRelease = async (
  repoOwner: string,
  repoName: string
): Promise<GitHubRelease | null> => {
  try {
    const response = await fetch(
      `https://api.github.com/repos/${repoOwner}/${repoName}/releases/latest`,
      {
        headers: {
          Accept: "application/vnd.github.v3+json",
        },
      }
    );

    if (!response.ok) {
      if (response.status === 404) {
        // No releases available
        return null;
      }
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    const data = await response.json();
    return {
      tag_name: data.tag_name,
      name: data.name,
      html_url: data.html_url,
      body: data.body,
      published_at: data.published_at,
    };
  } catch (error) {
    console.error("Failed to fetch latest release:", error);
    throw error;
  }
};
