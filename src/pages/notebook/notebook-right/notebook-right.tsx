import { memo, useState } from "react";
import {
  Modal,
  ModalContent,
  ModalHeader,
  ModalBody,
  Tabs,
  Tab,
  Snippet,
} from "@heroui/react";
import { useTranslation } from "@/i18n";

// 定义方法参数类型
interface MethodParam {
  name: string;
  type: string;
  default: string | boolean | undefined;
  desc: string;
  example: string;
}

// 定义函数类型
type FunctionType = "table-valued" | "scalar-valued";

// 定义方法类型
interface Method {
  name: string;
  description: string;
  params: MethodParam[];
  example: string;
  type: FunctionType;
  isBeta?: boolean;
}

// 创建支持国际化的方法数据
const createMethods = (t: (key: string) => string) => [
  {
    name: "read_csv",
    description: t("functions.readCsv.description"),
    type: "table-valued" as FunctionType,
    params: [
      {
        name: "infer_schema",
        type: "boolean",
        default: true,
        desc: t("functions.readCsv.inferSchema"),
        example: "false",
      },
    ],
    example: `select * from read_csv('data.csv', infer_schema => false)`,
  },
  {
    name: "read_tsv",
    description: t("functions.readTsv.description"),
    type: "table-valued" as FunctionType,
    params: [
      {
        name: "infer_schema",
        type: "boolean",
        default: true,
        desc: t("functions.readTsv.inferSchema"),
        example: "false",
      },
    ],
    example: `select * from read_tsv('data.tsv', infer_schema => false)`,
  },
  // {
  //   name: "read_json",
  //   description: t("functions.readJson.description"),
  //   type: "table-valued" as FunctionType,
  //   params: [
  //     {
  //       name: "infer_schema",
  //       type: "boolean",
  //       default: true,
  //       desc: "",
  //       example: "false",
  //     },
  //   ],
  //   example: `select * from read_json("data.json")`,
  // },
  {
    name: "read_ndjson",
    description: t("functions.readNdjson.description"),
    type: "table-valued" as FunctionType,
    params: [
      {
        name: "infer_schema",
        type: "boolean",
        default: true,
        desc: "",
        example: "false",
      },
    ],
    example: `select * from read_ndjson('data.ndjson')`,
  },
  {
    name: "read_parquet",
    description: t("functions.readParquet.description"),
    type: "table-valued" as FunctionType,
    params: [],
    example: `select * from read_parquet('data.parquet')`,
  },
  {
    name: "read_excel",
    description: t("functions.readExcel.description"),
    type: "table-valued" as FunctionType,
    params: [
      {
        name: "sheet_name",
        type: "string",
        default: "Sheet1",
        desc: "",
        example: "Sheet1",
      },
      {
        name: "infer_schema",
        type: "boolean",
        default: true,
        desc: t("functions.readExcel.inferSchema"),
        example: "false",
      },
    ],
    example: `select * from read_excel('data.xlsx', sheet_name => 'Sheet2', infer_schema => false)`,
    isBeta: true,
  },
  {
    name: "regexp_like",
    description: t("functions.regexpLike.description"),
    type: "scalar-valued" as FunctionType,
    params: [
      {
        name: "column",
        type: "string",
        default: undefined,
        desc: "",
        example: "my_column",
      },
      {
        name: "pattern",
        type: "string",
        default: undefined,
        desc: "",
        example: "^[0-9]+.[0-9]+?$",
      },
    ],
    example: `REGEXP_LIKE("Distance",'^[0-9]+.[0-9]+?$')`,
  },
];

function NotebookRight() {
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [selectedMethod, setSelectedMethod] = useState<Method | null>(null);
  const [selectedTab, setSelectedTab] = useState<FunctionType>("table-valued");
  const { translate } = useTranslation();

  // 创建支持国际化的方法数据
  const allMethods = createMethods(translate);

  // 根据选中的标签过滤方法
  const methods = allMethods.filter((method) => method.type === selectedTab);

  const handleMethodClick = (method: Method) => {
    setSelectedMethod(method);
    setIsModalOpen(true);
  };

  const handleCloseModal = () => {
    setIsModalOpen(false);
    setSelectedMethod(null);
  };

  return (
    <div
      style={{
        width: "280px",
        textAlign: "left",
        padding: "0 16px",
        height: "100%",
        boxSizing: "border-box",
        background: "linear-gradient(135deg, #f8fafc 0%, #f1f5f9 100%)",
        borderLeft: "1px solid #e2e8f0",
      }}
    >
      <div
        style={{
          height: 64,
          borderBottom: "1px solid #e2e8f0",
          display: "flex",
          alignItems: "center",
          fontWeight: "600",
          fontSize: "16px",
          color: "#1e293b",
          paddingLeft: "8px",
          background: "rgba(255, 255, 255, 0.8)",
          backdropFilter: "blur(10px)",
        }}
      >
        {translate("functions.title")}
      </div>

      {/* 标签切换 */}
      <div style={{ padding: "12px 0 8px 0" }}>
        <div style={{ display: "flex", justifyContent: "center" }}>
          <Tabs
            selectedKey={selectedTab}
            onSelectionChange={(key) => setSelectedTab(key as FunctionType)}
            size="sm"
            variant="underlined"
            classNames={{
              tabList:
                "gap-6 w-full relative rounded-none p-0 border-b border-divider",
              tab: "max-w-fit px-0 h-12",
            }}
          >
            <Tab
              key="table-valued"
              title={
                <span
                  style={{ whiteSpace: "pre-line", wordBreak: "break-all" }}
                >
                  {translate("functions.tableValued")}
                </span>
              }
            />
            <Tab
              key="scalar-valued"
              title={
                <span
                  style={{ whiteSpace: "pre-line", wordBreak: "break-all" }}
                >
                  {translate("functions.scalarValued")}
                </span>
              }
            />
          </Tabs>
        </div>
      </div>

      <div style={{ marginTop: "8px", paddingBottom: "16px" }}>
        <div
          style={{
            background: "#fff",
            borderRadius: "8px",
            border: "1px solid #e5e7eb",
            overflow: "hidden",
          }}
        >
          {methods.map((method, index) => (
            <div
              key={method.name}
              style={{
                padding: "12px 16px",
                borderBottom:
                  index === methods.length - 1 ? "none" : "1px solid #f3f4f6",
                cursor: "pointer",
                transition: "background-color 0.2s ease",
                display: "flex",
                alignItems: "center",
                gap: "12px",
              }}
              onClick={() => handleMethodClick(method)}
              className="hover:bg-gray-50"
            >
              <div
                style={{
                  width: "8px",
                  height: "8px",
                  borderRadius: "50%",
                  background: `hsl(${(index * 60) % 360}, 70%, 60%)`,
                  flexShrink: 0,
                }}
              />
              <div style={{ flex: 1 }}>
                <div
                  style={{
                    fontWeight: "600",
                    fontSize: "14px",
                    color: "#1e293b",
                    marginBottom: "2px",
                    display: "flex",
                    alignItems: "center",
                    justifyContent: "space-between",
                  }}
                >
                  <span>{method.name}</span>
                  {method.isBeta && (
                    <span
                      style={{
                        fontSize: "10px",
                        fontWeight: "700",
                        color: "#ffffff",
                        background:
                          "linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%)",
                        padding: "2px 6px",
                        borderRadius: "4px",
                        letterSpacing: "0.5px",
                        textTransform: "uppercase",
                        boxShadow: "0 1px 2px rgba(0, 0, 0, 0.1)",
                      }}
                    >
                      Beta
                    </span>
                  )}
                </div>
                <div
                  style={{
                    fontSize: "12px",
                    color: "#6b7280",
                    lineHeight: "1.4",
                  }}
                >
                  {method.description}
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* 弹窗组件 */}
      <Modal
        isOpen={isModalOpen}
        onClose={handleCloseModal}
        size="3xl"
        scrollBehavior="inside"
        backdrop="blur"
        classNames={{
          backdrop: "bg-black/50 backdrop-blur-sm",
          base: "border border-gray-200",
          header: "border-b border-gray-100",
          body: "py-6",
          closeButton: "hover:bg-gray-100",
        }}
      >
        <ModalContent>
          <ModalHeader className="flex flex-col gap-1">
            <div
              style={{
                fontSize: "20px",
                fontWeight: "700",
                color: "#1e293b",
                letterSpacing: "-0.02em",
                display: "flex",
                alignItems: "center",
                gap: "8px",
              }}
            >
              <span>{selectedMethod?.name}</span>
              {selectedMethod?.isBeta && (
                <span
                  style={{
                    fontSize: "10px",
                    fontWeight: "700",
                    color: "#3b82f6",
                    padding: "2px 6px",
                    letterSpacing: "0.5px",
                    textTransform: "uppercase",
                  }}
                >
                  Beta
                </span>
              )}
            </div>
          </ModalHeader>
          <ModalBody>
            {selectedMethod && (
              <div className="space-y-6">
                {/* 方法描述 */}
                <div
                  style={{
                    background:
                      "linear-gradient(135deg, #f8fafc 0%, #f1f5f9 100%)",
                    padding: "16px",
                    borderRadius: "12px",
                    border: "1px solid #e2e8f0",
                    marginBottom: "24px",
                  }}
                >
                  <div
                    style={{
                      fontSize: "15px",
                      color: "#475569",
                      lineHeight: "1.6",
                      fontWeight: "500",
                    }}
                  >
                    {selectedMethod.description}
                  </div>
                </div>

                {/* 参数说明 */}
                <div>
                  <div
                    style={{
                      fontWeight: "700",
                      marginBottom: "16px",
                      fontSize: "16px",
                      color: "#1e293b",
                      display: "flex",
                      alignItems: "center",
                      gap: "8px",
                    }}
                  >
                    <span>📋</span>
                    {translate("functions.parameters")}
                  </div>
                  <div
                    style={{
                      background: "#fff",
                      borderRadius: "8px",
                      border: "1px solid #e5e7eb",
                      overflow: "hidden",
                    }}
                  >
                    <div
                      style={{
                        display: "flex",
                        fontWeight: "600",
                        color: "#374151",
                        fontSize: "14px",
                        background: "#f9fafb",
                        padding: "12px 16px",
                        borderBottom: "1px solid #e5e7eb",
                        gap: "12px",
                      }}
                    >
                      <span style={{ width: "90px", flexShrink: 0 }}>
                        {translate("functions.paramName")}
                      </span>
                      <span
                        style={{
                          width: "70px",
                          flexShrink: 0,
                          textAlign: "center",
                        }}
                      >
                        {translate("functions.type")}
                      </span>
                      <span
                        style={{
                          width: "100px",
                          flexShrink: 0,
                          textAlign: "center",
                        }}
                      >
                        {translate("functions.defaultValue")}
                      </span>
                      <span
                        style={{
                          width: "120px",
                          flexShrink: 0,
                          textAlign: "center",
                        }}
                      >
                        {translate("functions.exampleValue")}
                      </span>
                      <span style={{ flex: 1 }}>
                        {translate("functions.description")}
                      </span>
                    </div>
                    {selectedMethod.params.map(
                      (param: MethodParam, index: number) => (
                        <div
                          key={param.name}
                          style={{
                            display: "flex",
                            alignItems: "center",
                            fontSize: "14px",
                            padding: "12px 16px",
                            gap: "12px",
                            background: index % 2 === 0 ? "#fff" : "#fafbfc",
                            borderBottom:
                              index === selectedMethod.params.length - 1
                                ? "none"
                                : "1px solid #f3f4f6",
                          }}
                        >
                          <span
                            style={{
                              width: "90px",
                              flexShrink: 0,
                              color: "#1f2937",
                              fontWeight: "600",
                              fontSize: "13px",
                            }}
                          >
                            {param.name}
                          </span>
                          <span
                            style={{
                              width: "70px",
                              flexShrink: 0,
                              color: "#6b7280",
                              fontWeight: "500",
                              fontSize: "13px",
                              textAlign: "center",
                            }}
                          >
                            {param.type}
                          </span>
                          <span
                            style={{
                              width: "100px",
                              flexShrink: 0,
                              color: "#6b7280",
                              fontSize: "13px",
                              textAlign: "center",
                            }}
                          >
                            {param.default !== undefined
                              ? String(param.default)
                              : translate("functions.required")}
                          </span>
                          <span
                            style={{
                              width: "120px",
                              flexShrink: 0,
                              color: "#374151",
                              fontSize: "13px",
                              fontFamily: "monospace",
                              textAlign: "center",
                            }}
                          >
                            {param.example}
                          </span>
                          <span
                            style={{
                              flex: 1,
                              color: "#6b7280",
                              fontSize: "13px",
                              lineHeight: "1.4",
                            }}
                          >
                            {param.desc}
                          </span>
                        </div>
                      )
                    )}
                  </div>
                </div>

                {/* 使用示例 */}
                <div>
                  <div
                    style={{
                      fontWeight: "700",
                      marginBottom: "16px",
                      fontSize: "16px",
                      color: "#1e293b",
                      display: "flex",
                      alignItems: "center",
                      gap: "8px",
                    }}
                  >
                    <span>💻</span>
                    {translate("functions.usageExample")}
                  </div>
                  <div
                    style={{
                      background:
                        "linear-gradient(135deg, #1e293b 0%, #334155 100%)",
                      padding: "20px",
                      borderRadius: "12px",
                      border: "1px solid #475569",
                      position: "relative",
                      overflow: "hidden",
                    }}
                  >
                    <div
                      style={{
                        position: "absolute",
                        top: "8px",
                        right: "12px",
                        fontSize: "10px",
                        color: "#94a3b8",
                        fontWeight: "500",
                        letterSpacing: "0.5px",
                      }}
                    >
                      CODE
                    </div>
                    <div
                      style={{
                        overflowX: "auto",
                        whiteSpace: "pre",
                        borderRadius: "8px",
                      }}
                    >
                      <div
                        style={{
                          overflowX: "auto",
                          whiteSpace: "pre",
                          borderRadius: "8px",
                          scrollbarWidth: "none", // Firefox
                          msOverflowStyle: "none", // IE and Edge
                        }}
                        className="no-scrollbar"
                      >
                        <Snippet
                          symbol=""
                          style={{
                            color: "#e2e8f0",
                            fontSize: "14px",
                            fontFamily: "JetBrains Mono, Consolas, monospace",
                            lineHeight: "1.6",
                            background: "transparent",
                            border: "none",
                            padding: 0,
                            minWidth: "fit-content",
                          }}
                        >
                          {selectedMethod.example}
                        </Snippet>
                        <style>
                          {`
                            .no-scrollbar {
                              scrollbar-width: none; /* Firefox */
                              -ms-overflow-style: none; /* IE and Edge */
                            }
                            .no-scrollbar::-webkit-scrollbar {
                              display: none; /* Chrome, Safari, Opera */
                            }
                          `}
                        </style>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            )}
          </ModalBody>
        </ModalContent>
      </Modal>
    </div>
  );
}

export default memo(NotebookRight);
