import React, { createContext, useContext, useState, useEffect } from "react";
import { i18n, Language } from "../i18n";

interface LanguageContextType {
  language: Language;
  setLanguage: (language: Language) => void;
}

const LanguageContext = createContext<LanguageContextType | undefined>(
  undefined
);

export const useLanguage = () => {
  const context = useContext(LanguageContext);
  if (!context) {
    throw new Error("useLanguage must be used within a LanguageProvider");
  }
  return context;
};

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

  const handleSetLanguage = (newLanguage: Language) => {
    setLanguageState(newLanguage);
    i18n.setLanguage(newLanguage);
    // 触发重新渲染
    if (typeof window !== "undefined") {
      window.dispatchEvent(
        new CustomEvent("languageChanged", { detail: newLanguage })
      );
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

  return (
    <LanguageContext.Provider
      value={{ language, setLanguage: handleSetLanguage }}
    >
      {children}
    </LanguageContext.Provider>
  );
};
