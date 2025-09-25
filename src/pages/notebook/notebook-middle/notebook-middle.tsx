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
import { memo, useState } from "react";
import { format } from "sql-formatter";
import NotebookMiddleBottom from "./notebook-mddle-bottom";
import { invoke } from "@tauri-apps/api/core";
import { useTranslation } from "@/i18n";

interface NotebookMiddleProps {
  source: string;
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

  const formatSql = () => {
    setSql(
      format(sql, {
        language: "sql",
        keywordCase: "upper",
      }).replace(/=\s>/g, "=>")
    );
  };

  return (
    <div
      style={{
        flex: "1",
        textAlign: "center",
        borderLeft: "1px solid rgba(17, 17, 17, 0.15)",
        borderRight: "1px solid rgba(17, 17, 17, 0.15)",
        overflow: "hidden",
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
