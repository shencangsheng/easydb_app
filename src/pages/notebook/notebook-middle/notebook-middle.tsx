import CustomAceEditor from "@/components/common/ace-editor";
import {
  faServer,
  faStop,
  faPlay,
  faScrewdriverWrench,
  faAlignLeft,
  faEraser,
} from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  Button,
  Dropdown,
  DropdownItem,
  DropdownMenu,
  DropdownTrigger,
} from "@heroui/react";
import { memo, useState, useRef, useEffect, useCallback } from "react";
import { format } from "sql-formatter";
import NotebookMiddleBottom from "./notebook-mddle-bottom";
import { invoke } from "@tauri-apps/api/core";
import { useTranslation } from "@/i18n";
import { listen } from "@tauri-apps/api/event";

interface NotebookMiddleProps {
  source: string;
}

function getFormatSql(sql: string) {
  return format(sql, {
    language: "sql",
    keywordCase: "upper",
  }).replace(/=\s>/g, "=>");
}

function NotebookMiddle({ source }: NotebookMiddleProps) {
  const { translate } = useTranslation();
  const [sql, setSql] = useState("");
  const [isRunning, setIsRunning] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [data, setData] = useState<{
    header: string[];
    rows: string[][];
    query_time: string;
  }>({
    header: [],
    rows: [],
    query_time: "",
  });
  const dropAreaRef = useRef<HTMLDivElement>(null);

  const formatSql = () => {
    setSql(
      format(sql, {
        language: "sql",
        keywordCase: "upper",
      }).replace(/=\s>/g, "=>")
    );
  };

  // 处理文件拖拽
  const handleFileDrop = useCallback((filePath: string) => {
    // 如果SQL编辑器为空，根据文件扩展名生成相应的SQL查询
    const fileExtension = filePath.split(".").pop()?.toLowerCase();
    let sqlQuery = "";

    switch (fileExtension) {
      case "csv":
        sqlQuery = `SELECT * FROM read_csv('${filePath}', infer_schema => false) LIMIT 10;`;
        break;
      case "xlsx":
      case "xls":
        sqlQuery = `SELECT * FROM read_excel('${filePath}') LIMIT 10;`;
        break;
      case "json":
        sqlQuery = `SELECT * FROM read_json('${filePath}') LIMIT 10;`;
        break;
      case "ndjson":
        sqlQuery = `SELECT * FROM read_ndjson('${filePath}') LIMIT 10;`;
        break;
      case "parquet":
        sqlQuery = `SELECT * FROM read_parquet('${filePath}') LIMIT 10;`;
        break;
      default:
        return;
    }

    setSql(getFormatSql(sqlQuery));
  }, []);

  // 监听Tauri的拖拽事件
  useEffect(() => {
    const setupDragDropListeners = async () => {
      // 监听文件拖拽完成事件
      const unlistenDrop = await listen(
        "tauri://drag-drop",
        (event: { payload: { paths: string[] } }) => {
          const filePaths = event.payload.paths;
          if (filePaths && filePaths.length > 0) {
            handleFileDrop(filePaths[0]); // 处理第一个文件
          }
        }
      );

      return () => {
        unlistenDrop();
      };
    };

    setupDragDropListeners();
  }, [handleFileDrop]);

  return (
    <div
      ref={dropAreaRef}
      style={{
        flex: "1",
        textAlign: "center",
        borderLeft: "1px solid rgba(17, 17, 17, 0.15)",
        borderRight: "1px solid rgba(17, 17, 17, 0.15)",
        overflow: "hidden",
        position: "relative",
      }}
    >
      <div
        style={{
          height: 60,
          borderBottom: "1px solid rgba(17, 17, 17, 0.15)",
          backgroundColor: "#F5F5F5",
        }}
      >
        <p
          style={{
            fontSize: "20px",
            textAlign: "left",
            paddingLeft: "15px",
            display: "flex",
            alignItems: "center",
            height: "100%",
          }}
        >
          <FontAwesomeIcon icon={faServer} style={{ marginRight: "10px" }} />
          {source}
        </p>
      </div>
      <div
        style={{
          display: "flex",
          justifyContent: "space-between",
          height: "45%",
          width: "100%",
          overflowY: "auto", // Enable vertical scrolling
          flex: 1,
        }}
      >
        <div
          style={{
            display: "flex",
            alignItems: "end",
            paddingBottom: "10px",
            height: "100%",
            width: "60px",
          }}
        >
          <div>
            <Button
              isIconOnly
              isDisabled={sql === ""}
              style={{ backgroundColor: "transparent" }}
              aria-label={isRunning ? "Stop query" : "Run query"}
              onPress={async () => {
                setIsRunning(!isRunning);
                if (!isRunning) {
                  try {
                    setData({ header: [], rows: [], query_time: "" });
                    setIsLoading(true);
                    const results: {
                      header: string[];
                      rows: string[][];
                      query_time: string;
                    } = await invoke("fetch", {
                      sql: sql,
                    });
                    setData(results);
                  } catch (error) {
                    setData({
                      header: ["Error"],
                      rows: [[`${error}`]],
                      query_time: "<1ms",
                    });
                  } finally {
                    setIsRunning(false);
                    setIsLoading(false);
                  }
                }
              }}
            >
              <FontAwesomeIcon
                icon={isRunning ? faStop : faPlay}
                style={{
                  color: isRunning ? "red" : "#87CEEB",
                  fontSize: "1.2em",
                }}
              />
            </Button>
            <Dropdown placement="bottom-start" isDisabled={sql === ""}>
              <DropdownTrigger>
                <Button
                  variant="light"
                  isIconOnly
                  aria-label="Tools and settings"
                >
                  <FontAwesomeIcon icon={faScrewdriverWrench} />
                </Button>
              </DropdownTrigger>
              <DropdownMenu aria-label="Static Actions">
                <DropdownItem key="format" onPress={formatSql}>
                  <FontAwesomeIcon
                    icon={faAlignLeft}
                    style={{ marginRight: "5px" }}
                  />
                  Format
                </DropdownItem>
                <DropdownItem key="clear" onPress={() => setSql("")}>
                  <FontAwesomeIcon
                    icon={faEraser}
                    style={{ marginRight: "5px" }}
                  />
                  Clear
                </DropdownItem>
              </DropdownMenu>
            </Dropdown>
          </div>
        </div>
        <div
          className="textarea-container"
          style={{
            width: "100%",
            height: "100%",
            display: "flex",
            paddingTop: "10px",
          }}
        >
          <CustomAceEditor
            value={sql}
            onChange={(value) => setSql(value)}
            placeholder={translate("notebook.editorPlaceholder")}
            fontSize={16}
            height="100%"
            width="100%"
            showPrintMargin={true}
            showGutter={true}
            highlightActiveLine={true}
            enableBasicAutocompletion={true}
            enableLiveAutocompletion={true}
            enableSnippets={false}
            showLineNumbers={true}
            tabSize={2}
          />
        </div>
      </div>
      <div style={{ marginTop: "20px", marginLeft: "50px" }}>
        <NotebookMiddleBottom
          data={data}
          isLoading={isLoading}
          setSql={setSql}
          sql={sql}
        />
      </div>
    </div>
  );
}

export default memo(NotebookMiddle);
