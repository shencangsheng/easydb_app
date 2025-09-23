import { useNavigate } from "react-router-dom";
import { Button } from "@heroui/react";
import { useTranslation } from "../../i18n";
import { invoke } from "@tauri-apps/api/core";
import { getVersionInfo } from "../../utils/version";

function AboutPage() {
  const navigate = useNavigate();
  const { translate } = useTranslation();
  const versionInfo = getVersionInfo();

  const handleCheckUpdate = () => {
    // 这里可以添加检查更新的逻辑
    alert(translate("about.checkUpdate") + " - 当前已是最新版本");
  };

  const handleGithubClick = async () => {
    await invoke("open_url", {
      url: "https://github.com/shencangsheng/easydb_app",
    });
  };

  return (
    <div
      style={{
        display: "flex",
        height: "100vh",
        backgroundColor: "#f8f9fa",
        padding: "24px",
      }}
    >
      <div
        style={{
          maxWidth: "800px",
          width: "100%",
          margin: "0 auto",
          backgroundColor: "#fff",
          borderRadius: "12px",
          padding: "32px",
          boxShadow: "0 4px 6px -1px rgba(0, 0, 0, 0.1)",
        }}
      >
        {/* 顶部导航 */}
        <div
          style={{
            display: "flex",
            alignItems: "center",
            justifyContent: "space-between",
            marginBottom: "32px",
          }}
        >
          <Button
            variant="light"
            onPress={() => navigate("/")}
            style={{ minWidth: "80px" }}
          >
            {translate("common.back")}
          </Button>
        </div>

        {/* 应用信息区域 */}
        <div style={{ marginBottom: "32px" }}>
          <div
            style={{
              display: "flex",
              alignItems: "center",
              marginBottom: "16px",
            }}
          >
            <div
              style={{
                width: "64px",
                height: "64px",
                backgroundColor: "#fff",
                borderRadius: "12px",
                display: "flex",
                alignItems: "center",
                justifyContent: "center",
                marginRight: "16px",
                fontSize: "32px",
                color: "#000", // 黑色
                border: "1px solid #e5e7eb",
              }}
            >
              <svg fill="none" height="64" viewBox="0 0 32 32" width="64">
                <path
                  clipRule="evenodd"
                  d="M17.6482 10.1305L15.8785 7.02583L7.02979 22.5499H10.5278L17.6482 10.1305ZM19.8798 14.0457L18.11 17.1983L19.394 19.4511H16.8453L15.1056 22.5499H24.7272L19.8798 14.0457Z"
                  fill="currentColor"
                  fillRule="evenodd"
                />
              </svg>
            </div>
            <div>
              <h1
                style={{
                  fontSize: "24px",
                  fontWeight: 600,
                  margin: 0,
                  color: "#1f2937",
                }}
              >
                {translate("about.title")}
              </h1>
              <p
                style={{
                  fontSize: "16px",
                  color: "#6b7280",
                  margin: "4px 0 0 0",
                }}
              >
                {translate("about.description")}
              </p>
            </div>
            <div style={{ marginLeft: "auto" }}>
              <Button
                color="primary"
                variant="solid"
                onPress={handleCheckUpdate}
                style={{ minWidth: "120px" }}
              >
                {translate("about.checkUpdate")}
              </Button>
            </div>
          </div>

          <p
            style={{
              fontSize: "14px",
              color: "#6b7280",
              lineHeight: "1.6",
              marginBottom: "16px",
            }}
          >
            {translate("about.descriptionDetail")}
          </p>

          {/* 版本详细信息 */}
          <div
            style={{
              backgroundColor: "#f8fafc",
              border: "1px solid #e2e8f0",
              borderRadius: "8px",
              padding: "16px",
              marginBottom: "16px",
            }}
          >
            <h4
              style={{
                fontSize: "14px",
                fontWeight: 600,
                margin: "0 0 12px 0",
                color: "#374151",
              }}
            >
              版本信息
            </h4>
            <div
              style={{
                display: "grid",
                gridTemplateColumns: "1fr 1fr",
                gap: "8px",
              }}
            >
              <div>
                <span style={{ fontSize: "12px", color: "#6b7280" }}>
                  版本号：
                </span>
                <span style={{ fontSize: "12px", fontWeight: 500 }}>
                  {versionInfo.version}
                </span>
              </div>
              <div>
                <span style={{ fontSize: "12px", color: "#6b7280" }}>
                  构建号：
                </span>
                <span style={{ fontSize: "12px", fontWeight: 500 }}>
                  {versionInfo.buildNumber}
                </span>
              </div>
              <div>
                <span style={{ fontSize: "12px", color: "#6b7280" }}>
                  发布日期：
                </span>
                <span style={{ fontSize: "12px", fontWeight: 500 }}>
                  {versionInfo.releaseDate}
                </span>
              </div>
              <div>
                <span style={{ fontSize: "12px", color: "#6b7280" }}>
                  许可证：
                </span>
                <span style={{ fontSize: "12px", fontWeight: 500 }}>
                  {versionInfo.license}
                </span>
              </div>
            </div>
          </div>
        </div>

        {/* 正版提示 */}
        <div
          style={{
            backgroundColor: "#fef3c7",
            border: "1px solid #f59e0b",
            borderRadius: "8px",
            padding: "16px",
            marginBottom: "32px",
          }}
        >
          <div style={{ display: "flex", alignItems: "flex-start" }}>
            <span
              style={{
                fontSize: "20px",
                marginRight: "12px",
                marginTop: "2px",
              }}
            >
              ⚠️
            </span>
            <div>
              <h3
                style={{
                  fontSize: "16px",
                  fontWeight: 600,
                  margin: "0 0 8px 0",
                  color: "#92400e",
                }}
              >
                {translate("about.genuineAlertTitle")}
              </h3>
              <p
                style={{
                  fontSize: "14px",
                  color: "#92400e",
                  margin: 0,
                  lineHeight: "1.5",
                }}
              >
                {translate("about.genuineAlertContent")}
              </p>
            </div>
          </div>
        </div>

        {/* 社交媒体和联系方式 */}
        <div style={{ marginBottom: "32px" }}>
          <h3
            style={{
              fontSize: "18px",
              fontWeight: 600,
              marginBottom: "16px",
              color: "#1f2937",
            }}
          >
            {translate("about.socialMedia")}
          </h3>

          <div
            style={{ display: "flex", flexDirection: "column", gap: "12px" }}
          >
            {/* Github */}
            <div
              onClick={handleGithubClick}
              style={{
                display: "flex",
                alignItems: "center",
                padding: "12px",
                backgroundColor: "#f9fafb",
                borderRadius: "8px",
                cursor: "pointer",
                transition: "background-color 0.2s",
              }}
              onMouseEnter={(e) => {
                e.currentTarget.style.backgroundColor = "#f3f4f6";
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.backgroundColor = "#f9fafb";
              }}
            >
              <span style={{ fontSize: "20px", marginRight: "12px" }}>
                <svg
                  width="20"
                  height="20"
                  viewBox="0 0 24 24"
                  fill="currentColor"
                  aria-label="GitHub"
                  style={{ display: "inline-block", verticalAlign: "middle" }}
                >
                  <path d="M12 2C6.477 2 2 6.484 2 12.021c0 4.428 2.865 8.184 6.839 9.504.5.092.682-.217.682-.483 0-.237-.009-.868-.014-1.703-2.782.605-3.369-1.342-3.369-1.342-.454-1.154-1.11-1.462-1.11-1.462-.908-.62.069-.608.069-.608 1.004.07 1.532 1.032 1.532 1.032.892 1.53 2.341 1.088 2.91.832.091-.647.35-1.088.636-1.339-2.221-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.025A9.564 9.564 0 0 1 12 6.844c.85.004 1.705.115 2.504.337 1.909-1.295 2.748-1.025 2.748-1.025.546 1.378.202 2.397.1 2.65.64.7 1.028 1.595 1.028 2.688 0 3.847-2.337 4.695-4.566 4.944.359.309.678.919.678 1.853 0 1.337-.012 2.419-.012 2.749 0 .268.18.579.688.481C19.138 20.2 22 16.447 22 12.021 22 6.484 17.523 2 12 2z" />
                </svg>
              </span>
              <span style={{ fontSize: "14px", fontWeight: 500 }}>
                {translate("about.github")}
              </span>
              <span
                style={{
                  marginLeft: "auto",
                  fontSize: "14px",
                  color: "#6b7280",
                }}
              >
                EasyDB
              </span>
              <span style={{ marginLeft: "8px", fontSize: "14px" }}>→</span>
            </div>

            {/* 邮箱 */}
            <div
              style={{
                display: "flex",
                alignItems: "center",
                padding: "12px",
                backgroundColor: "#f9fafb",
                borderRadius: "8px",
              }}
            >
              <span style={{ fontSize: "20px", marginRight: "12px" }}>✉️</span>
              <span style={{ fontSize: "14px", fontWeight: 500 }}>
                {translate("about.email")}
              </span>
              <span
                style={{
                  marginLeft: "auto",
                  fontSize: "14px",
                  color: "#6b7280",
                }}
              >
                shencangsheng@126.com
              </span>
              <span style={{ marginLeft: "8px", fontSize: "14px" }}>→</span>
            </div>
          </div>
        </div>

        {/* 其他链接 */}
        <div>
          <h3
            style={{
              fontSize: "18px",
              fontWeight: 600,
              marginBottom: "16px",
              color: "#1f2937",
            }}
          >
            {translate("common.general")}
          </h3>

          <div
            style={{ display: "flex", flexDirection: "column", gap: "12px" }}
          >
            {/* 首页 */}
            <div
              onClick={() => navigate("/")}
              style={{
                display: "flex",
                alignItems: "center",
                padding: "12px",
                backgroundColor: "#f9fafb",
                borderRadius: "8px",
                cursor: "pointer",
                transition: "background-color 0.2s",
              }}
              onMouseEnter={(e) => {
                e.currentTarget.style.backgroundColor = "#f3f4f6";
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.backgroundColor = "#f9fafb";
              }}
            >
              <span style={{ fontSize: "20px", marginRight: "12px" }}>🏠</span>
              <span style={{ fontSize: "14px", fontWeight: 500 }}>
                {translate("about.homepage")}
              </span>
              <span style={{ marginLeft: "auto", fontSize: "14px" }}>→</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default AboutPage;
