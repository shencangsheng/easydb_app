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
  functions: {
    title: string;
    description: string;
    parameters: string;
    example: string;
    paramName: string;
    type: string;
    defaultValue: string;
    exampleValue: string;
    required: string;
    clickToView: string;
    apiMethodDetails: string;
    usageExample: string;
    tableValued: string;
    scalarValued: string;
    readCsv: {
      name: string;
      description: string;
      inferSchema: string;
    };
    readTsv: {
      name: string;
      description: string;
      inferSchema: string;
    };
    readJson: {
      name: string;
      description: string;
    };
    readNdjson: {
      name: string;
      description: string;
    };
    readExcel: {
      name: string;
      description: string;
      inferSchema: string;
    };
    regexpLike: {
      name: string;
      description: string;
    };
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
    description: string;
    descriptionDetail: string;
    checkUpdate: string;
    privacyPolicy: string;
    userTerms: string;
    genuineAlert: string;
    genuineAlertTitle: string;
    genuineAlertContent: string;
    socialMedia: string;
    github: string;
    xiaohongshu: string;
    wechat: string;
    qrCode: string;
    homepage: string;
    survey: string;
    feedback: string;
    changelog: string;
    email: string;
    faq: string;
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
    functions: {
      title: "函数",
      description: "描述",
      parameters: "参数说明",
      example: "使用示例",
      paramName: "参数名",
      type: "类型",
      defaultValue: "默认值",
      exampleValue: "示例值",
      required: "必需",
      clickToView: "点击查看详情 →",
      apiMethodDetails: "API 方法详情",
      usageExample: "使用示例",
      tableValued: "表值函数",
      scalarValued: "标量函数",
      readCsv: {
        name: "read_csv",
        description: "读取 CSV 文件为表。",
        inferSchema:
          "是否自动推断数据类型。为 true 时，将根据前 100 行进行推断。",
      },
      readTsv: {
        name: "read_tsv",
        description: "读取 TSV 文件为表。",
        inferSchema:
          "是否自动推断数据类型。为 true 时，将根据前 100 行进行推断。",
      },
      readJson: {
        name: "read_json",
        description: "读取 JSON 文件为表。",
      },
      readNdjson: {
        name: "read_ndjson",
        description: "读取 NDJSON 文件为表。",
      },
      readExcel: {
        name: "read_excel",
        description: "读取 Excel 文件为表。",
        inferSchema:
          "是否自动推断数据类型。为 true 时，将根据前 100 行进行推断。",
      },
      regexpLike: {
        name: "REGEXP_LIKE",
        description: "正则表达式匹配函数。",
      },
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
      title: "EasyDB (v1.0.0)",
      version: "版本",
      author: "作者",
      description: "一个简约强大的 SQL 桌面客户端，支持多种文件格式查询",
      descriptionDetail:
        "EasyDB 是一个简约强大的 SQL 桌面客户端，基于 Rust 构建，具备高性能的文件查询能力，轻松处理数百兆乃至数GB的大型文本文件，仅需极少的硬件资源。支持 CSV、NdJson、JSON、Excel 和 Parquet 文件格式，无需进行文件转换，开箱即用。",
      checkUpdate: "检查更新",
      privacyPolicy: "隐私政策",
      userTerms: "用户条款",
      genuineAlert: "正版提示",
      genuineAlertTitle: "正版提示",
      genuineAlertContent:
        "EasyDB客户端是开源免费软件。如果您发现任何付费捆绑软件声称包含EasyDB，请及时申请退款。",
      socialMedia: "社交媒体",
      github: "Github",
      xiaohongshu: "小红书",
      wechat: "微信",
      qrCode: "二维码",
      homepage: "首页",
      survey: "调查问卷",
      feedback: "建议反馈",
      changelog: "更新日志",
      email: "电子邮件",
      faq: "常见疑问",
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
    functions: {
      title: "Functions",
      description: "Description",
      parameters: "Parameters",
      example: "Example",
      paramName: "Parameter",
      type: "Type",
      defaultValue: "Default",
      exampleValue: "Example",
      required: "Required",
      clickToView: "Click to view details →",
      apiMethodDetails: "API Method Details",
      usageExample: "Usage Example",
      tableValued: "Table-Valued Functions",
      scalarValued: "Scalar-Valued Functions",
      readCsv: {
        name: "read_csv",
        description: "Read CSV file as table.",
        inferSchema:
          "Whether to automatically infer data types. If true, the first 100 rows are used for inference.",
      },
      readTsv: {
        name: "read_tsv",
        description: "Read TSV file as table.",
        inferSchema:
          "Whether to automatically infer data types. If true, the first 100 rows are used for inference.",
      },
      readJson: {
        name: "read_json",
        description: "Read JSON file as table.",
      },
      readNdjson: {
        name: "read_ndjson",
        description: "Read NDJSON file as table.",
      },
      readExcel: {
        name: "read_excel",
        description: "Read Excel file as table.",
        inferSchema:
          "Whether to automatically infer data types. If true, the first 100 rows are used for inference.",
      },
      regexpLike: {
        name: "REGEXP_LIKE",
        description: "Regular expression matching function.",
      },
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
      title: "EasyDB (v1.0.0)",
      version: "Version",
      author: "Author",
      description:
        "A simple yet powerful SQL desktop client supporting multiple file formats",
      descriptionDetail:
        "EasyDB is a simple yet powerful SQL desktop client built with Rust, featuring high-performance file querying capabilities that effortlessly handle hundreds of megabytes to multiple gigabytes of large text files with minimal hardware resources. It supports CSV, NdJson, JSON, Excel, and Parquet file formats without requiring file conversion, ready to use out of the box.",
      checkUpdate: "Check for Updates",
      privacyPolicy: "Privacy Policy",
      userTerms: "User Terms",
      genuineAlert: "Genuine Product Alert",
      genuineAlertTitle: "Genuine Product Alert",
      genuineAlertContent:
        "EasyDB client is open-source free software. If you find any paid bundled software claiming to include EasyDB, please apply for a refund promptly.",
      socialMedia: "Social Media",
      github: "Github",
      xiaohongshu: "Xiaohongshu",
      wechat: "WeChat",
      qrCode: "QR Code",
      homepage: "Homepage",
      survey: "Survey",
      feedback: "Feedback",
      changelog: "Changelog",
      email: "Email",
      faq: "FAQ",
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
      let value: unknown = translations[this.currentLanguage];

      if (!value) {
        console.warn(
          `No translations found for language: ${this.currentLanguage}`
        );
        return key;
      }

      for (const k of keys) {
        if (typeof value === "object" && value !== null && k in value) {
          value = (value as Record<string, unknown>)[k];
        } else {
          value = undefined;
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
        getLanguage: () => "zh-CN" as Language,
        setLanguage: () => {},
        t: (key: string) => key,
      } as unknown as I18n;
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
