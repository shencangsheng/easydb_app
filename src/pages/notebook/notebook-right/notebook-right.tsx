import { memo, useState } from "react";
import { useTranslation } from "../../../i18n";
import {
  Accordion,
  AccordionItem,
  Card,
  CardBody,
  CardHeader,
  Divider,
} from "@heroui/react";

const METHODS = [
  {
    name: "read_csv",
    description: "读取 CSV 文件为 DataFrame。",
    params: [
      { name: "filepath", type: "string", desc: "CSV 文件路径" },
      { name: "sep", type: "string", desc: "分隔符，默认 ','" },
      { name: "header", type: "boolean", desc: "是否包含表头，默认 true" },
      { name: "encoding", type: "string", desc: "文件编码，默认 'utf-8'" },
    ],
    example: `read_csv("data.csv", sep=",", header=true)`,
  },
  {
    name: "read_json",
    description: "读取 JSON 文件为 DataFrame。",
    params: [
      { name: "filepath", type: "string", desc: "JSON 文件路径" },
      { name: "orient", type: "string", desc: "json 格式，默认 'records'" },
      { name: "encoding", type: "string", desc: "文件编码，默认 'utf-8'" },
    ],
    example: `read_json("data.json", orient="records")`,
  },
  {
    name: "to_csv",
    description: "将 DataFrame 保存为 CSV 文件。",
    params: [
      { name: "df", type: "DataFrame", desc: "要保存的数据" },
      { name: "filepath", type: "string", desc: "保存路径" },
      { name: "index", type: "boolean", desc: "是否保存索引，默认 false" },
      { name: "encoding", type: "string", desc: "文件编码，默认 'utf-8'" },
    ],
    example: `to_csv(df, "output.csv", index=false)`,
  },
  {
    name: "to_json",
    description: "将 DataFrame 保存为 JSON 文件。",
    params: [
      { name: "df", type: "DataFrame", desc: "要保存的数据" },
      { name: "filepath", type: "string", desc: "保存路径" },
      { name: "orient", type: "string", desc: "json 格式，默认 'records'" },
      { name: "encoding", type: "string", desc: "文件编码，默认 'utf-8'" },
    ],
    example: `to_json(df, "output.json", orient="records")`,
  },
];

function NotebookRight() {
  const [selectedKeys, setSelectedKeys] = useState<Set<string>>(new Set());
  const { translate } = useTranslation();

  return (
    <div
      style={{
        width: "250px",
        textAlign: "left",
        padding: "0 10px",
        height: "100%",
        boxSizing: "border-box",
        background: "#fafbfc",
      }}
    >
      <div
        style={{
          height: 60,
          borderBottom: "1px solid rgba(17, 17, 17, 0.15)",
          display: "flex",
          alignItems: "center",
          fontWeight: "bold",
          fontSize: "18px",
          color: "#333",
          paddingLeft: "5px",
        }}
      >
        {translate("notebook.methodList")}
      </div>
      <div style={{ marginTop: "10px" }}>
        <Accordion
          selectedKeys={selectedKeys}
          onSelectionChange={(keys) => setSelectedKeys(keys as Set<string>)}
          variant="shadow"
        >
          {METHODS.map((method) => (
            <AccordionItem
              key={method.name}
              aria-label={method.name}
              title={method.name}
              subtitle={method.description}
            >
              <Card
                style={{
                  background: "#fff",
                  boxShadow: "none",
                  border: "none",
                }}
              >
                <CardHeader style={{ fontWeight: "bold", fontSize: "16px" }}>
                  {method.name}
                </CardHeader>
                <CardBody>
                  <div style={{ color: "#666", marginBottom: "8px" }}>
                    {method.description}
                  </div>
                  <div style={{ fontWeight: "bold", marginBottom: "4px" }}>
                    参数说明:
                  </div>
                  {/* 参数说明排版优化：用flex布局一行展示每个参数 */}
                  <div
                    style={{
                      display: "flex",
                      flexDirection: "column",
                      gap: "4px",
                      marginBottom: "8px",
                    }}
                  >
                    <div
                      style={{
                        display: "flex",
                        fontWeight: 500,
                        color: "#888",
                        fontSize: "13px",
                        borderBottom: "1px solid #eee",
                        paddingBottom: "2px",
                        gap: "6px",
                      }}
                    >
                      <span style={{ minWidth: 60, flex: "0 0 60px" }}>
                        参数
                      </span>
                      <span style={{ minWidth: 60, flex: "0 0 60px" }}>
                        类型
                      </span>
                      <span style={{ flex: 1 }}>说明</span>
                    </div>
                    {method.params.map((param) => (
                      <div
                        key={param.name}
                        style={{
                          display: "flex",
                          alignItems: "center",
                          fontSize: "13px",
                          gap: "6px",
                          padding: "2px 0",
                          borderBottom: "1px solid #f5f5f5",
                        }}
                      >
                        <span
                          style={{
                            minWidth: 60,
                            flex: "0 0 60px",
                            color: "#222",
                            whiteSpace: "nowrap",
                            overflow: "hidden",
                            textOverflow: "ellipsis",
                          }}
                          title={param.name}
                        >
                          {param.name}
                        </span>
                        <span
                          style={{
                            minWidth: 60,
                            flex: "0 0 60px",
                            color: "#555",
                            whiteSpace: "nowrap",
                            overflow: "hidden",
                            textOverflow: "ellipsis",
                          }}
                          title={param.type}
                        >
                          {param.type}
                        </span>
                        <span
                          style={{
                            flex: 1,
                            color: "#666",
                            whiteSpace: "nowrap",
                            overflow: "hidden",
                            textOverflow: "ellipsis",
                          }}
                          title={param.desc}
                        >
                          {param.desc}
                        </span>
                      </div>
                    ))}
                  </div>
                  <Divider />
                  <div style={{ marginTop: "8px", color: "#888" }}>
                    <span style={{ fontWeight: "bold" }}>示例：</span>
                    <code
                      style={{
                        background: "#f5f5f5",
                        padding: "2px 4px",
                        borderRadius: "3px",
                        wordBreak: "break-all",
                      }}
                    >
                      {method.example}
                    </code>
                  </div>
                </CardBody>
              </Card>
            </AccordionItem>
          ))}
        </Accordion>
      </div>
    </div>
  );
}

export default memo(NotebookRight);
