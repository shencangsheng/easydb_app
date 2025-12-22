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
    dragDropHint: string;
    run: string;
    stop: string;
    format: string;
    clear: string;
    history: {
      title: string;
      emptyState: {
        icon: string;
        title: string;
        description: string;
      };
      clickToUse: string;
      searchPlaceholder: string;
      noResults: string;
      noResultsDescription: string;
    };
    export: {
      export: string;
      completed: string;
      queryTime: string;
      fileName: string;
      sqlExportSettings: string;
      sqlStatementType: string;
      sqlStatementTypePlaceholder: string;
      insertDescription: string;
      updateDescription: string;
      whereColumn: string;
      whereColumnPlaceholder: string;
      whereColumnDisabledHint: string;
      tableName: string;
      tableNamePlaceholder: string;
      maxValuesPerInsert: string;
      maxValuesPerInsertPlaceholder: string;
      batchSizeDisabledHint: string;
      databaseDialect: string;
      databaseDialectPlaceholder: string;
      mysql: string;
      postgresql: string;
      cancel: string;
      confirmExport: string;
    };
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
      hasHeader: string;
      delimiter: string;
      fileExtension: string;
    };
    readText: {
      name: string;
      description: string;
      inferSchema: string;
      hasHeader: string;
      delimiter: string;
      fileExtension: string;
    };
    readTsv: {
      name: string;
      description: string;
      inferSchema: string;
      hasHeader: string;
    };
    readJson: {
      name: string;
      description: string;
    };
    readNdjson: {
      name: string;
      description: string;
      inferSchema: string;
    };
    readExcel: {
      name: string;
      description: string;
      inferSchema: string;
      sheetName: string;
    };
    readParquet: {
      name: string;
      description: string;
    };
    readMysql: {
      name: string;
      description: string;
      table: string;
      conn: string;
    };
    regexpLike: {
      name: string;
      description: string;
      column: string;
      pattern: string;
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
    checkingUpdate: string;
    updateCheckTitle: string;
    parseRepoError: string;
    noReleaseAvailable: string;
    newVersionAvailable: string;
    newVersionTitle: string;
    goToDownload: string;
    cancel: string;
    alreadyLatestVersion: string;
    updateCheckError: string;
    updateCheckFailed: string;
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
      dragDropHint: "支持拖拽文件 (CSV, Excel, JSON, Parquet) 自动生成查询",
      run: "运行",
      stop: "停止",
      format: "格式化",
      clear: "清空",
      history: {
        title: "查询历史",
        emptyState: {
          icon: "📝",
          title: "暂无查询历史",
          description: "执行查询后将显示历史记录",
        },
        clickToUse: "点击使用此查询",
        searchPlaceholder: "搜索查询历史...",
        noResults: "未找到匹配的查询",
        noResultsDescription: "请尝试使用其他关键词搜索",
      },
      export: {
        export: "导出",
        completed: "下载完成",
        queryTime: "查询时间",
        fileName: "文件名",
        sqlExportSettings: "SQL 导出设置",
        sqlStatementType: "SQL 语句类型",
        sqlStatementTypePlaceholder: "请选择 SQL 语句类型",
        insertDescription: "配置 INSERT 语句的导出参数",
        updateDescription: "配置 UPDATE 语句的导出参数",
        whereColumn: "WHERE 字段",
        whereColumnPlaceholder: "请选择 WHERE 字段",
        whereColumnDisabledHint: "INSERT 语句不需要 WHERE 条件",
        tableName: "表名",
        tableNamePlaceholder: "请输入表名",
        maxValuesPerInsert: "单次插入行数",
        maxValuesPerInsertPlaceholder: "设置单次插入的最大行数",
        batchSizeDisabledHint: "UPDATE 语句不需要批次大小设置",
        databaseDialect: "数据库方言",
        databaseDialectPlaceholder: "请选择数据库方言",
        mysql: "MySQL",
        postgresql: "PostgreSQL",
        cancel: "取消",
        confirmExport: "确认导出",
      },
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
        hasHeader: "文件是否包含表头行。为 true 时，第一行将被视为列名。",
        delimiter: "字段分隔符，默认为逗号 ','。",
        fileExtension: "文件扩展名，默认为 '.csv'。",
      },
      readText: {
        name: "read_text",
        description: "读取文本文件为表。",
        inferSchema:
          "是否自动推断数据类型。为 true 时，将根据前 100 行进行推断。",
        hasHeader: "文件是否包含表头行。为 true 时，第一行将被视为列名。",
        delimiter: "字段分隔符，默认为逗号 ','。",
        fileExtension: "文件扩展名，默认为 '.txt'。",
      },
      readTsv: {
        name: "read_tsv",
        description: "读取 TSV 文件为表。",
        inferSchema:
          "是否自动推断数据类型。为 true 时，将根据前 100 行进行推断。",
        hasHeader: "文件是否包含表头行。为 true 时，第一行将被视为列名。",
      },
      readJson: {
        name: "read_json",
        description: "读取 JSON 文件为表。",
      },
      readNdjson: {
        name: "read_ndjson",
        description: "读取 NDJSON 文件为表。",
        inferSchema:
          "是否自动推断数据类型。为 true 时，将根据前 100 行进行推断。",
      },
      readExcel: {
        name: "read_excel",
        description: "读取 Excel 文件为表。",
        inferSchema:
          "是否自动推断数据类型。为 true 时，将根据前 100 行进行推断。",
        sheetName: "要读取的工作表名称，默认读取第一个 sheet。",
      },
      readParquet: {
        name: "read_parquet",
        description: "读取 Parquet 文件为表。",
      },
      readMysql: {
        name: "read_mysql",
        description: "从 MySQL 数据库读取表数据。",
        table: "要读取的表名称。",
        conn: "MySQL 连接字符串",
      },
      regexpLike: {
        name: "REGEXP_LIKE",
        description: "正则表达式匹配函数。",
        column: "要匹配的列名。",
        pattern: "正则表达式模式，用于匹配列值。",
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
      title: "EasyDB",
      version: "版本",
      author: "作者",
      description: "一个简约强大的 SQL 桌面客户端，支持多种文件格式查询",
      descriptionDetail:
        "EasyDB 是一个简约强大的 SQL 桌面客户端，基于 Rust 构建，具备高性能的文件查询能力，轻松处理数百兆乃至数GB的大型文本文件，仅需极少的硬件资源。支持 CSV、NdJson、JSON、Excel 和 Parquet 文件格式，无需进行文件转换，开箱即用。",
      checkUpdate: "检查更新",
      checkingUpdate: "检查中...",
      updateCheckTitle: "检查更新",
      parseRepoError: "无法解析仓库地址，请手动访问 GitHub 检查更新。",
      noReleaseAvailable: "当前没有可用的发布版本。",
      newVersionAvailable:
        "发现新版本：{latestVersion}\n\n当前版本：{currentVersion}\n\n是否前往下载页面？",
      newVersionTitle: "发现新版本",
      goToDownload: "前往下载",
      cancel: "取消",
      alreadyLatestVersion: "当前已是最新版本（{currentVersion}）",
      updateCheckError:
        "检查更新时发生错误，请稍后重试或手动访问 GitHub 检查。",
      updateCheckFailed: "检查更新失败",
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
      dragDropHint:
        "Drag and drop files (CSV, Excel, JSON, Parquet) to auto-generate queries",
      run: "Run",
      stop: "Stop",
      format: "Format",
      clear: "Clear",
      history: {
        title: "Query History",
        emptyState: {
          icon: "📝",
          title: "No Query History",
          description: "Query history will appear after executing queries",
        },
        clickToUse: "Click to use this query",
        searchPlaceholder: "Search query history...",
        noResults: "No matching queries found",
        noResultsDescription: "Try using different search keywords",
      },
      export: {
        export: "Export",
        completed: "Download Completed",
        queryTime: "Query Time",
        fileName: "File Name",
        sqlExportSettings: "SQL Export Settings",
        sqlStatementType: "SQL Statement Type",
        sqlStatementTypePlaceholder: "Please select SQL statement type",
        insertDescription: "Configure export parameters for INSERT statements",
        updateDescription: "Configure export parameters for UPDATE statements",
        whereColumn: "WHERE Column",
        whereColumnPlaceholder: "Please select WHERE column",
        whereColumnDisabledHint:
          "INSERT statements do not require WHERE conditions",
        tableName: "Table Name",
        tableNamePlaceholder: "Enter table name",
        maxValuesPerInsert: "Batch Size",
        maxValuesPerInsertPlaceholder: "Set the number of rows per batch",
        batchSizeDisabledHint:
          "UPDATE statements do not require batch size settings",
        databaseDialect: "Database Dialect",
        databaseDialectPlaceholder: "Please select database dialect",
        mysql: "MySQL",
        postgresql: "PostgreSQL",
        cancel: "Cancel",
        confirmExport: "Confirm Export",
      },
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
        hasHeader:
          "Whether the file contains a header row. If true, the first row will be treated as column names.",
        delimiter: "Field delimiter, defaults to comma ','.",
        fileExtension: "File extension, defaults to '.csv'.",
      },
      readText: {
        name: "read_text",
        description: "Read text file as table.",
        inferSchema:
          "Whether to automatically infer data types. If true, the first 100 rows are used for inference.",
        hasHeader:
          "Whether the file contains a header row. If true, the first row will be treated as column names.",
        delimiter: "Field delimiter, defaults to comma ','.",
        fileExtension: "File extension, defaults to '.txt'.",
      },
      readTsv: {
        name: "read_tsv",
        description: "Read TSV file as table.",
        inferSchema:
          "Whether to automatically infer data types. If true, the first 100 rows are used for inference.",
        hasHeader:
          "Whether the file contains a header row. If true, the first row will be treated as column names.",
      },
      readJson: {
        name: "read_json",
        description: "Read JSON file as table.",
      },
      readNdjson: {
        name: "read_ndjson",
        description: "Read NDJSON file as table.",
        inferSchema:
          "Whether to automatically infer data types. If true, the first 100 rows are used for inference.",
      },
      readExcel: {
        name: "read_excel",
        description: "Read Excel file as table.",
        inferSchema:
          "Whether to automatically infer data types. If true, the first 100 rows are used for inference.",
        sheetName: "Name of the sheet to read, defaults first sheet.",
      },
      readParquet: {
        name: "read_parquet",
        description: "Read Parquet file as table.",
      },
      readMysql: {
        name: "read_mysql",
        description: "Read table data from MySQL database.",
        table: "Name of the table to read.",
        conn: "MySQL connection string.",
      },
      regexpLike: {
        name: "REGEXP_LIKE",
        description: "Regular expression matching function.",
        column: "The column name to match against.",
        pattern: "Regular expression pattern used to match column values.",
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
      title: "EasyDB",
      version: "Version",
      author: "Author",
      description:
        "A simple yet powerful SQL desktop client supporting multiple file formats",
      descriptionDetail:
        "EasyDB is a simple yet powerful SQL desktop client built with Rust, featuring high-performance file querying capabilities that effortlessly handle hundreds of megabytes to multiple gigabytes of large text files with minimal hardware resources. It supports CSV, NdJson, JSON, Excel, and Parquet file formats without requiring file conversion, ready to use out of the box.",
      checkUpdate: "Check for Updates",
      checkingUpdate: "Checking...",
      updateCheckTitle: "Check for Updates",
      parseRepoError:
        "Unable to parse repository URL. Please check GitHub manually for updates.",
      noReleaseAvailable: "No releases are currently available.",
      newVersionAvailable:
        "New version available: {latestVersion}\n\nCurrent version: {currentVersion}\n\nWould you like to go to the download page?",
      newVersionTitle: "New Version Available",
      goToDownload: "Go to Download",
      cancel: "Cancel",
      alreadyLatestVersion:
        "You are using the latest version ({currentVersion})",
      updateCheckError:
        "An error occurred while checking for updates. Please try again later or check GitHub manually.",
      updateCheckFailed: "Update Check Failed",
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
        } else {
          // 如果没有保存的语言设置，根据浏览器默认语言自动选择
          const browserLanguage = this.detectBrowserLanguage();
          this.currentLanguage = browserLanguage;
          // 保存自动检测的语言设置
          localStorage.setItem("app-language", browserLanguage);
        }
      }
      this.initialized = true;
    } catch (error) {
      console.warn("Failed to initialize i18n:", error);
      this.currentLanguage = "zh-CN";
      this.initialized = true;
    }
  }

  private detectBrowserLanguage(): Language {
    try {
      if (typeof window !== "undefined" && window.navigator) {
        const browserLang =
          window.navigator.language || window.navigator.languages?.[0];

        // 检查是否是中文（包括简体中文、繁体中文等）
        if (browserLang && browserLang.toLowerCase().includes("zh")) {
          return "zh-CN";
        }

        // 其他情况默认使用英文
        return "en-US";
      }
    } catch (error) {
      console.warn("Failed to detect browser language:", error);
    }

    // 如果检测失败，默认使用中文
    return "zh-CN";
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
