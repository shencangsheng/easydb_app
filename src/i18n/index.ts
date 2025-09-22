// 简化的国际化实现
export type Language = "zh-CN" | "en-US";

export interface Translations {
  common: {
    settings: string;
    about: string;
    back: string;
    language: string;
    fontSize: string;
    display: string;
    general: string;
  };
  navbar: {
    title: string;
    searchPlaceholder: string;
  };
  notebook: {
    methodList: string;
    server: string;
    editorPlaceholder: string;
    run: string;
    stop: string;
    format: string;
    clear: string;
  };
  settings: {
    title: string;
    language: string;
    fontSize: string;
    displaySettings: string;
    generalSettings: string;
  };
  languages: {
    "zh-CN": string;
    "en-US": string;
  };
  about: {
    title: string;
    version: string;
    author: string;
  };
}

const translations: Record<Language, Translations> = {
  "zh-CN": {
    common: {
      settings: "设置",
      about: "关于",
      back: "返回",
      language: "语言",
      fontSize: "字体大小",
      display: "显示设置",
      general: "常规设置",
    },
    navbar: {
      title: "EasyDB",
      searchPlaceholder: "搜索数据和保存的文档...",
    },
    notebook: {
      methodList: "方法列表",
      server: "服务器",
      editorPlaceholder: "示例: SELECT * FROM tablename, 或按 CTRL + space",
      run: "运行",
      stop: "停止",
      format: "格式化",
      clear: "清空",
    },
    settings: {
      title: "设置",
      language: "语言",
      fontSize: "字体大小",
      displaySettings: "显示设置",
      generalSettings: "常规设置",
    },
    languages: {
      "zh-CN": "简体中文",
      "en-US": "English",
    },
    about: {
      title: "关于 EasyDB",
      version: "版本",
      author: "作者",
    },
  },
  "en-US": {
    common: {
      settings: "Settings",
      about: "About",
      back: "Back",
      language: "Language",
      fontSize: "Font Size",
      display: "Display Settings",
      general: "General Settings",
    },
    navbar: {
      title: "EasyDB",
      searchPlaceholder: "Search data and saved documents...",
    },
    notebook: {
      methodList: "Method List",
      server: "Server",
      editorPlaceholder:
        "Example: SELECT * FROM tablename, or press CTRL + space",
      run: "Run",
      stop: "Stop",
      format: "Format",
      clear: "Clear",
    },
    settings: {
      title: "Settings",
      language: "Language",
      fontSize: "Font Size",
      displaySettings: "Display Settings",
      generalSettings: "General Settings",
    },
    languages: {
      "zh-CN": "简体中文",
      "en-US": "English",
    },
    about: {
      title: "About EasyDB",
      version: "Version",
      author: "Author",
    },
  },
};

class I18n {
  private currentLanguage: Language = "zh-CN";
  private initialized: boolean = false;

  constructor() {
    this.initialize();
  }

  private initialize() {
    try {
      // 从localStorage加载保存的语言
      if (typeof window !== "undefined" && window.localStorage) {
        const savedLanguage = localStorage.getItem("app-language") as Language;
        if (savedLanguage && ["zh-CN", "en-US"].includes(savedLanguage)) {
          this.currentLanguage = savedLanguage;
        }
      }
      this.initialized = true;
    } catch (error) {
      console.warn("Failed to initialize i18n:", error);
      this.currentLanguage = "zh-CN";
      this.initialized = true;
    }
  }

  getLanguage(): Language {
    return this.currentLanguage;
  }

  setLanguage(language: Language): void {
    this.currentLanguage = language;
    try {
      if (typeof window !== "undefined" && window.localStorage) {
        localStorage.setItem("app-language", language);
      }
    } catch (error) {
      console.warn("Failed to save language to localStorage:", error);
    }
  }

  t(key: string): string {
    try {
      if (!this.initialized) {
        return key;
      }

      const keys = key.split(".");
      let value: any = translations[this.currentLanguage];

      if (!value) {
        console.warn(
          `No translations found for language: ${this.currentLanguage}`
        );
        return key;
      }

      for (const k of keys) {
        value = value?.[k];
        if (value === undefined) {
          break;
        }
      }

      return typeof value === "string" ? value : key;
    } catch (error) {
      console.warn(`Translation error for key "${key}":`, error);
      return key;
    }
  }
}

// 延迟初始化，避免在模块加载时出错
let i18nInstance: I18n | null = null;

function getI18n(): I18n {
  if (!i18nInstance) {
    try {
      i18nInstance = new I18n();
    } catch (error) {
      console.warn("Failed to create i18n instance:", error);
      // 创建一个最小的实例
      i18nInstance = {
        getLanguage: () => "zh-CN",
        setLanguage: () => {},
        t: (key: string) => key,
      } as any;
    }
  }
  return i18nInstance!;
}

export const i18n = getI18n()!;

// React Hook
export function useTranslation() {
  const translate = (key: string): string => {
    try {
      const instance = getI18n();
      if (!instance) {
        return key;
      }
      return instance.t(key);
    } catch (error) {
      console.warn(`Translation error for key "${key}":`, error);
      return key;
    }
  };

  return {
    translate,
    t: translate, // 保持向后兼容
    i18n: getI18n(),
  };
}
