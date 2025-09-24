import React, { useState, useEffect } from "react";
import { i18n, Language } from "../i18n";
import { invoke } from "@tauri-apps/api/core";
import { ask } from "@tauri-apps/plugin-dialog";
import { LanguageContextType, LanguageContext } from "./LanguageContextTypes";

interface LanguageProviderProps {
  children: React.ReactNode;
}

export const LanguageProvider: React.FC<LanguageProviderProps> = ({
  children,
}) => {
  const [language, setLanguageState] = useState<Language>(() => {
    // 在客户端初始化时获取语言
    try {
      if (typeof window !== "undefined") {
        return i18n.getLanguage();
      }
    } catch (error) {
      console.warn("Failed to get initial language:", error);
    }
    return "zh-CN"; // 默认语言
  });

  const handleSetLanguage = async (newLanguage: Language) => {
    // 只有当语言真正改变时才询问重启
    if (newLanguage !== language) {
      // 先显示确认对话框，使用当前语言
      try {
        const isChinese = language === "zh-CN"; // 使用切换前的语言
        const message = isChinese
          ? "语言已更改，需要重启应用以使新语言完全生效。是否立即重启？"
          : "Language has been changed. Restart the application to fully apply the new language. Restart now?";
        const title = isChinese ? "重启应用" : "Restart Application";
        const okLabel = isChinese ? "立即重启" : "Restart Now";
        const cancelLabel = isChinese ? "稍后重启" : "Restart Later";

        const shouldRestart = await ask(message, {
          title: title,
          kind: "info",
          okLabel: okLabel,
          cancelLabel: cancelLabel,
        });

        if (shouldRestart) {
          // 用户确认重启后，更新语言状态
          setLanguageState(newLanguage);
          i18n.setLanguage(newLanguage);

          // 触发重新渲染
          if (typeof window !== "undefined") {
            window.dispatchEvent(
              new CustomEvent("languageChanged", { detail: newLanguage })
            );
          }

          // 延迟一小段时间确保状态更新完成，然后重启应用
          setTimeout(async () => {
            try {
              await invoke("restart_app");
            } catch (error) {
              console.error("Failed to restart app:", error);
            }
          }, 100);
        } else {
          // 用户选择稍后重启，仍然更新语言状态但不重启
          setLanguageState(newLanguage);
          i18n.setLanguage(newLanguage);

          // 触发重新渲染
          if (typeof window !== "undefined") {
            window.dispatchEvent(
              new CustomEvent("languageChanged", { detail: newLanguage })
            );
          }
        }
      } catch (error) {
        console.error("Failed to show restart dialog:", error);
        // 如果对话框显示失败，仍然更新语言状态
        setLanguageState(newLanguage);
        i18n.setLanguage(newLanguage);

        if (typeof window !== "undefined") {
          window.dispatchEvent(
            new CustomEvent("languageChanged", { detail: newLanguage })
          );
        }
      }
    }
  };

  useEffect(() => {
    if (typeof window === "undefined") return;

    const handleLanguageChange = () => {
      setLanguageState(i18n.getLanguage());
    };

    window.addEventListener("languageChanged", handleLanguageChange);
    return () =>
      window.removeEventListener("languageChanged", handleLanguageChange);
  }, []);

  const contextValue: LanguageContextType = {
    language,
    setLanguage: handleSetLanguage,
  };

  return (
    <LanguageContext.Provider value={contextValue}>
      {children}
    </LanguageContext.Provider>
  );
};
