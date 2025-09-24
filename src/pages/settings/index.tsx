import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { Select, SelectItem, Slider, Button } from "@heroui/react";
import { useTranslation } from "@/i18n";
import { useFontSize } from "../../contexts/FontSizeContext";
import { useLanguage } from "../../contexts/LanguageContext";
import { open, ask } from "@tauri-apps/plugin-dialog";

// 设置分类定义
const SETTING_CATEGORIES = [
  {
    key: "general",
    label: "常规设置",
    icon: "⚙️",
  },
  {
    key: "authorization",
    label: "权限管理",
    icon: "🔐",
  },
];

// 语言选项
const LANGUAGES = [
  { label: "简体中文", value: "zh-CN" },
  { label: "English", value: "en-US" },
];

// 简化的授权功能 - Tauri 会自动管理文件系统权限

function SettingsPage() {
  const navigate = useNavigate();
  const { translate } = useTranslation();
  const [selectedCategory, setSelectedCategory] = useState("general");
  const [feedbackMessage, setFeedbackMessage] = useState<{
    type: "success" | "error" | "info";
    message: string;
  } | null>(null);

  // 使用全局上下文
  const { fontSize, setFontSize } = useFontSize();
  const { language, setLanguage } = useLanguage();

  const handleSettingChange = (key: string, value: string | number) => {
    if (key === "fontSize") {
      setFontSize(value as number);
    } else if (key === "language") {
      setLanguage(value as "zh-CN" | "en-US");
    }
  };

  // 显示反馈消息
  const showFeedback = (
    type: "success" | "error" | "info",
    message: string
  ) => {
    setFeedbackMessage({ type, message });
  };

  const handleOpenDialog = async () => {
    try {
      // 首先询问用户要选择文件还是目录
      const selectionType = await ask("请选择权限类型", {
        title: "选择权限类型",
        kind: "info",
        okLabel: "选择文件",
        cancelLabel: "选择目录",
      });

      let result;
      if (selectionType) {
        // 选择文件
        result = await open({
          title: "选择文件进行权限管理",
          filters: [
            {
              name: "所有文件",
              extensions: ["*"],
            },
            {
              name: "数据文件",
              extensions: ["csv", "xlsx", "json", "sql"],
            },
          ],
          multiple: false,
        });

        if (result) {
          showFeedback("success", `已授予文件权限: ${result}`);
        }
      } else {
        // 选择目录
        result = await open({
          title: "选择目录进行权限管理",
          directory: true,
        });

        if (result) {
          showFeedback("success", `已授予目录权限: ${result}`);
        }
      }
    } catch (error) {
      console.error("打开对话框失败:", error);
      showFeedback("error", "打开对话框失败，请检查权限设置");
    }
  };

  // 渲染常规设置内容
  const renderGeneralSettings = () => (
    <div style={{ padding: "24px" }}>
      {/* 显示设置 */}
      <div style={{ marginBottom: "32px" }}>
        <h3
          style={{
            fontSize: "16px",
            fontWeight: 600,
            marginBottom: "16px",
            color: "#333",
          }}
        >
          {translate("settings.displaySettings")}
        </h3>

        <div style={{ marginBottom: "20px" }}>
          <label
            style={{
              display: "block",
              marginBottom: "8px",
              fontWeight: 500,
              color: "#333",
            }}
          >
            {translate("common.language")}
          </label>
          <Select
            selectedKeys={[language]}
            onSelectionChange={(keys) => {
              const val = Array.from(keys)[0];
              if (typeof val === "string") handleSettingChange("language", val);
            }}
            style={{ width: "200px" }}
            placeholder={translate("common.language")}
          >
            {LANGUAGES.map((lang) => (
              <SelectItem key={lang.value}>
                {translate(`languages.${lang.value}`)}
              </SelectItem>
            ))}
          </Select>
        </div>

        <div style={{ marginBottom: "20px" }}>
          <label
            style={{
              display: "block",
              marginBottom: "8px",
              fontWeight: 500,
              color: "#333",
            }}
          >
            {translate("common.fontSize")}
          </label>
          <div style={{ width: "200px" }}>
            <div style={{ display: "flex", alignItems: "center", gap: "12px" }}>
              <Slider
                defaultValue={fontSize}
                onChange={(value) =>
                  handleSettingChange(
                    "fontSize",
                    Array.isArray(value) ? value[0] : value
                  )
                }
                minValue={10}
                maxValue={24}
                step={1}
                className="max-w-md"
                style={{ flex: 1 }}
              />
              <span style={{ minWidth: 32, textAlign: "right", color: "#555" }}>
                {fontSize}
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );

  // 渲染授权管理内容
  const renderAuthorizationSettings = () => (
    <div style={{ padding: "24px" }}>
      <div style={{ marginBottom: "32px" }}>
        <h3
          style={{
            fontSize: "16px",
            fontWeight: 600,
            marginBottom: "16px",
            color: "#333",
          }}
        >
          文件访问权限
        </h3>

        <div style={{ marginBottom: "20px" }}>
          <p
            style={{
              fontSize: "14px",
              color: "#666",
              marginBottom: "16px",
              lineHeight: "1.5",
            }}
          >
            通过此功能，您可以主动授予应用访问特定文件或目录的权限。点击下方按钮打开选择对话框，您可以选择单个文件或整个目录进行权限管理。
          </p>

          <Button
            color="primary"
            variant="solid"
            onPress={handleOpenDialog}
            style={{
              backgroundColor: "#3b82f6",
              color: "white",
              fontWeight: 500,
            }}
          >
            管理文件/目录权限
          </Button>

          {/* 反馈消息 */}
          {feedbackMessage && (
            <div
              style={{
                marginTop: "16px",
                padding: "12px 16px",
                borderRadius: "8px",
                fontSize: "14px",
                fontWeight: 500,
                backgroundColor:
                  feedbackMessage.type === "success"
                    ? "#f0f9ff"
                    : feedbackMessage.type === "error"
                    ? "#fef2f2"
                    : "#f8fafc",
                color:
                  feedbackMessage.type === "success"
                    ? "#1e40af"
                    : feedbackMessage.type === "error"
                    ? "#dc2626"
                    : "#475569",
                border: `1px solid ${
                  feedbackMessage.type === "success"
                    ? "#bfdbfe"
                    : feedbackMessage.type === "error"
                    ? "#fecaca"
                    : "#e2e8f0"
                }`,
                display: "flex",
                alignItems: "center",
                gap: "8px",
              }}
            >
              <span
                style={{
                  fontSize: "16px",
                }}
              >
                {feedbackMessage.type === "success"
                  ? "✅"
                  : feedbackMessage.type === "error"
                  ? "❌"
                  : "ℹ️"}
              </span>
              {feedbackMessage.message}
            </div>
          )}
        </div>

        <div style={{ marginBottom: "20px" }}>
          <h4
            style={{
              fontSize: "14px",
              fontWeight: 600,
              marginBottom: "8px",
              color: "#333",
            }}
          >
            说明
          </h4>
          <ul
            style={{
              fontSize: "13px",
              color: "#666",
              paddingLeft: "20px",
              lineHeight: "1.5",
            }}
          >
            <li>此功能使用 Tauri 的 dialog.open() 和 dialog.ask() API</li>
            <li>支持选择单个文件或整个目录进行权限管理</li>
            <li>文件选择支持各种类型，包括数据文件（CSV、Excel、JSON、SQL）</li>
            <li>目录选择允许授予访问整个文件夹及其所有内容的权限</li>
            <li>选择后，Tauri 会自动将路径添加到应用的文件系统权限中</li>
            <li>权限会在应用的生命周期内保持有效</li>
            <li>这是用户主动授予权限的安全机制</li>
          </ul>
        </div>
      </div>
    </div>
  );

  // 渲染其他设置分类的内容
  const renderCategoryContent = () => {
    switch (selectedCategory) {
      case "general":
        return renderGeneralSettings();
      case "authorization":
        return renderAuthorizationSettings();
      default:
        return renderGeneralSettings();
    }
  };

  return (
    <div
      style={{
        display: "flex",
        height: "100vh",
        backgroundColor: "#f8f9fa",
      }}
    >
      {/* 左侧导航 */}
      <div
        style={{
          width: "240px",
          backgroundColor: "#fff",
          borderRight: "1px solid #e5e7eb",
          padding: "16px 0",
        }}
      >
        <div style={{ padding: "0 16px" }}>
          <h2
            style={{
              fontSize: "18px",
              fontWeight: 600,
              marginBottom: "24px",
              color: "#333",
            }}
          >
            {translate("common.settings")}
          </h2>
        </div>

        <div style={{ display: "flex", flexDirection: "column" }}>
          {SETTING_CATEGORIES.map((category) => (
            <div
              key={category.key}
              onClick={() => setSelectedCategory(category.key)}
              style={{
                display: "flex",
                alignItems: "center",
                padding: "12px 16px",
                cursor: "pointer",
                backgroundColor:
                  selectedCategory === category.key ? "#f0f9ff" : "transparent",
                color:
                  selectedCategory === category.key ? "#1e40af" : "#374151",
                fontWeight: selectedCategory === category.key ? 500 : 400,
                transition: "all 0.2s",
                borderLeft:
                  selectedCategory === category.key
                    ? "3px solid #3b82f6"
                    : "3px solid transparent",
              }}
            >
              <span
                style={{
                  fontSize: "16px",
                  marginRight: "12px",
                  width: "20px",
                  textAlign: "center",
                }}
              >
                {category.icon}
              </span>
              <span style={{ fontSize: "14px" }}>
                {category.key === "general"
                  ? translate("common.general")
                  : category.label}
              </span>
            </div>
          ))}
        </div>
      </div>

      {/* 右侧内容区域 */}
      <div
        style={{
          flex: 1,
          backgroundColor: "#fff",
          overflow: "auto",
        }}
      >
        {/* 顶部导航栏 */}
        <div
          style={{
            padding: "16px 24px",
            borderBottom: "1px solid #e5e7eb",
            display: "flex",
            alignItems: "center",
            justifyContent: "space-between",
          }}
        >
          <h1
            style={{
              fontSize: "20px",
              fontWeight: 600,
              margin: 0,
              color: "#333",
            }}
          >
            {translate("common.settings")}
          </h1>
          <Button
            variant="light"
            onPress={() => navigate("/")}
            style={{ minWidth: "80px" }}
          >
            {translate("common.back")}
          </Button>
        </div>

        {renderCategoryContent()}
      </div>
    </div>
  );
}

export default SettingsPage;
