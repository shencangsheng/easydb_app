import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { Select, SelectItem, Slider, Button } from "@heroui/react";
import { useTranslation } from "../../i18n";
import { useFontSize } from "../../contexts/FontSizeContext";
import { useLanguage } from "../../contexts/LanguageContext";

// 设置分类定义
const SETTING_CATEGORIES = [
  {
    key: "general",
    label: "常规设置",
    icon: "⚙️",
  },
];

// 语言选项
const LANGUAGES = [
  { label: "简体中文", value: "zh-CN" },
  { label: "English", value: "en-US" },
];

function SettingsPage() {
  const navigate = useNavigate();
  const { translate } = useTranslation();
  const [selectedCategory, setSelectedCategory] = useState("general");

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

  // 渲染其他设置分类的内容
  const renderCategoryContent = () => {
    switch (selectedCategory) {
      case "general":
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
                {translate("common.general")}
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
