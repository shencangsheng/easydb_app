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
import { memo, useState, useRef, useEffect, useCallback, useMemo } from "react";
import { format } from "sql-formatter";
import NotebookMiddleBottom from "./notebook-mddle-bottom";
import { invoke } from "@tauri-apps/api/core";
import { useTranslation } from "@/i18n";
import { listen } from "@tauri-apps/api/event";

const QUERY_PAGE_SIZE = 200;

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
  const [lastExecutedSql, setLastExecutedSql] = useState("");
  const [hasMore, setHasMore] = useState(true);
  const [isLoadingMore, setIsLoadingMore] = useState(false);
  const dropAreaRef = useRef<HTMLDivElement>(null);
  const abortControllerRef = useRef<AbortController | null>(null);
  const editorRef = useRef<{ getSelectedText: () => string } | null>(null);
  const popoverRef = useRef<HTMLDivElement>(null);
  const [isDropModalOpen, setIsDropModalOpen] = useState(false);
  const [droppedFilePath, setDroppedFilePath] = useState<string | null>(null);
  const [droppedFileExtension, setDroppedFileExtension] = useState<
    string | null
  >(null);

  // Cache format function with useCallback
  const formatSql = useCallback(() => {
    setSql(getFormatSql(sql));
  }, [sql]);

  // Cache clear function with useCallback
  const clearSql = useCallback(() => {
    setSql("");
  }, []);

  // Cache file extension to SQL query mapping with useMemo
  const fileExtensionToSql = useMemo(
    () => ({
      csv: (filePath: string) => `SELECT * FROM read_csv('${filePath}');`,
      xlsx: (filePath: string) => `SELECT * FROM read_excel('${filePath}');`,
      json: (filePath: string) => `SELECT * FROM read_ndjson('${filePath}');`,
      // ndjson: (filePath: string) =>
      //   `SELECT * FROM read_ndjson('${filePath}') LIMIT 100;`,
      parquet: (filePath: string) =>
        `SELECT * FROM read_parquet('${filePath}');`,
      tsv: (filePath: string) => `SELECT * FROM read_tsv('${filePath}');`,
      text: (filePath: string) => `SELECT * FROM read_text('${filePath}');`,
    }),
    [],
  );

  // Cache file extension to read_xxx function name mapping with useMemo
  const fileExtensionToReadFunction = useMemo(
    () => ({
      csv: "read_csv",
      xlsx: "read_excel",
      json: "read_ndjson",
      // ndjson: "read_ndjson",
      parquet: "read_parquet",
      tsv: "read_tsv",
      text: "read_text",
    }),
    [],
  );

  // Handle file drop - show options menu
  const handleFileDrop = useCallback(
    (filePath: string) => {
      const fileExtension = filePath.split(".").pop()?.toLowerCase();

      if (fileExtension) {
        // If file extension is not in mapping, use default "text" (corresponds to read_text)
        const extensionKey =
          fileExtension in fileExtensionToSql ? fileExtension : "text";
        setDroppedFilePath(filePath);
        setDroppedFileExtension(extensionKey);
        setIsDropModalOpen(true);
      }
    },
    [fileExtensionToSql],
  );

  // Handle inserting full SQL
  const handleInsertFullSql = useCallback(() => {
    if (droppedFilePath && droppedFileExtension) {
      const sqlQuery =
        fileExtensionToSql[
          droppedFileExtension as keyof typeof fileExtensionToSql
        ](droppedFilePath);
      setSql(getFormatSql(sqlQuery));
      setIsDropModalOpen(false);
      setDroppedFilePath(null);
      setDroppedFileExtension(null);
    }
  }, [droppedFilePath, droppedFileExtension, fileExtensionToSql]);

  // Handle inserting only read_xxx
  const handleInsertReadFunction = useCallback(() => {
    if (droppedFilePath && droppedFileExtension) {
      const readFunction =
        fileExtensionToReadFunction[
          droppedFileExtension as keyof typeof fileExtensionToReadFunction
        ];
      const readFunctionCall = `${readFunction}('${droppedFilePath}')`;
      setSql((prevSql) => {
        // If editor is not empty, add newline and content at the end
        return prevSql ? `${prevSql}\n${readFunctionCall}` : readFunctionCall;
      });
      setIsDropModalOpen(false);
      setDroppedFilePath(null);
      setDroppedFileExtension(null);
    }
  }, [droppedFilePath, droppedFileExtension, fileExtensionToReadFunction]);

  // Generate option preview text
  const insertOptions = useMemo(() => {
    if (!droppedFilePath || !droppedFileExtension) return [];

    const readFunction =
      fileExtensionToReadFunction[
        droppedFileExtension as keyof typeof fileExtensionToReadFunction
      ];
    const fullSql =
      fileExtensionToSql[
        droppedFileExtension as keyof typeof fileExtensionToSql
      ](droppedFilePath);

    return [
      {
        key: "full",
        title: translate("notebook.insertFullSql") || "插入完整 SQL",
        preview: fullSql,
        onClick: handleInsertFullSql,
      },
      {
        key: "read",
        title: translate("notebook.insertReadFunction") || "仅插入 read_xxx",
        preview: `${readFunction}('${droppedFilePath}')`,
        onClick: handleInsertReadFunction,
      },
    ];
  }, [
    droppedFilePath,
    droppedFileExtension,
    fileExtensionToSql,
    fileExtensionToReadFunction,
    handleInsertFullSql,
    handleInsertReadFunction,
    translate,
  ]);

  // Cache query execution function with useCallback
  const executeQuery = useCallback(async () => {
    // Get selected text, use selected text if available, otherwise use entire SQL
    let sqlToExecute = sql;
    if (editorRef.current) {
      const selectedText = editorRef.current.getSelectedText();
      if (selectedText && selectedText.trim()) {
        sqlToExecute = selectedText;
      }
    }

    if (isRunning || !sqlToExecute.trim()) return;

    setIsRunning(true);
    setIsLoading(true);
    setData({ header: [], rows: [], query_time: "" });

    // Create AbortController for canceling request
    const abortController = new AbortController();
    abortControllerRef.current = abortController;

    try {
      const results: {
        header: string[];
        rows: string[][];
        query_time: string;
      } = await invoke("fetch", {
        sql: sqlToExecute,
        offset: 0,
        limit: QUERY_PAGE_SIZE,
      });

      // Check if cancelled
      if (abortController.signal.aborted) {
        setData({
          header: ["Status"],
          rows: [["Query cancelled"]],
          query_time: "-",
        });
        return;
      }

      setData(results);
      setLastExecutedSql(sqlToExecute);
      setHasMore(results.rows.length >= QUERY_PAGE_SIZE);
    } catch (error) {
      // Check if it's a cancellation error
      if (abortController.signal.aborted) {
        setData({
          header: ["Status"],
          rows: [["Query cancelled"]],
          query_time: "-",
        });
      } else {
        setData({
          header: ["Error"],
          rows: [[`${error}`]],
          query_time: "<1ms",
        });
      }
    } finally {
      setIsRunning(false);
      setIsLoading(false);
      abortControllerRef.current = null;
    }
  }, [sql, isRunning]);

  // Cache cancel query function with useCallback
  const cancelQuery = useCallback(() => {
    if (abortControllerRef.current) {
      abortControllerRef.current.abort();
      setIsRunning(false);
      setIsLoading(false);
    }
  }, []);

  // Cache clear data function with useCallback
  const clearData = useCallback(() => {
    setData({
      header: [],
      rows: [],
      query_time: "",
    });
    setHasMore(true);
  }, []);

  // Load next page of data
  const loadMore = useCallback(async () => {
    if (!lastExecutedSql || isLoadingMore || !hasMore || data.rows.length === 0)
      return;
    const offset = data.rows.length;
    setIsLoadingMore(true);
    try {
      const results: {
        header: string[];
        rows: string[][];
        query_time: string;
      } = await invoke("fetch_page", {
        sql: lastExecutedSql,
        offset,
        limit: QUERY_PAGE_SIZE,
      });
      setData((prev) => ({
        ...prev,
        rows: [...prev.rows, ...results.rows],
      }));
      setHasMore(results.rows.length >= QUERY_PAGE_SIZE);
    } catch (error) {
      console.error("Failed to load more:", error);
      setHasMore(false);
    } finally {
      setIsLoadingMore(false);
    }
  }, [lastExecutedSql, isLoadingMore, hasMore, data.rows.length]);

  // Listen to Tauri drag and drop events
  useEffect(() => {
    const setupDragDropListeners = async () => {
      // Listen to file drag and drop completion event
      const unlistenDrop = await listen(
        "tauri://drag-drop",
        (event: { payload: { paths: string[] } }) => {
          const filePaths = event.payload.paths;
          if (filePaths && filePaths.length > 0) {
            handleFileDrop(filePaths[0]);
          }
        },
      );

      return () => {
        unlistenDrop();
      };
    };

    setupDragDropListeners();
  }, [handleFileDrop]);

  // Close menu when clicking outside
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (
        isDropModalOpen &&
        popoverRef.current &&
        !popoverRef.current.contains(event.target as Node)
      ) {
        setIsDropModalOpen(false);
        setDroppedFilePath(null);
        setDroppedFileExtension(null);
      }
    };

    if (isDropModalOpen) {
      document.addEventListener("mousedown", handleClickOutside);
      return () => {
        document.removeEventListener("mousedown", handleClickOutside);
      };
    }
  }, [isDropModalOpen]);

  // Cache style objects with useMemo
  const containerStyle = useMemo(
    () => ({
      flex: "1",
      textAlign: "center" as const,
      borderLeft: "1px solid rgba(17, 17, 17, 0.15)",
      borderRight: "1px solid rgba(17, 17, 17, 0.15)",
      overflow: "hidden",
      position: "relative" as const,
    }),
    [],
  );

  const headerStyle = useMemo(
    () => ({
      height: 60,
      borderBottom: "1px solid rgba(17, 17, 17, 0.15)",
      backgroundColor: "#F5F5F5",
    }),
    [],
  );

  const editorContainerStyle = useMemo(
    () => ({
      display: "flex",
      justifyContent: "space-between",
      height: "45%",
      width: "100%",
      overflowY: "auto" as const,
      flex: 1,
    }),
    [],
  );

  return (
    <div ref={dropAreaRef} style={containerStyle}>
      <div style={headerStyle}>
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
      <div style={editorContainerStyle}>
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
              onPress={isRunning ? cancelQuery : executeQuery}
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
                <DropdownItem key="clear" onPress={clearSql}>
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
            position: "relative",
          }}
        >
          <CustomAceEditor
            value={sql}
            onChange={setSql}
            onLoad={(editor) => {
              editorRef.current = editor;
            }}
            placeholder={`${translate(
              "notebook.editorPlaceholder",
            )}\n\n${translate("notebook.dragDropHint")}`}
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
            tableColumns={data.header}
          />
          {/* File drag and drop options popup menu */}
          {isDropModalOpen && (
            <div
              ref={popoverRef}
              style={{
                position: "absolute",
                left: "50%",
                top: "50%",
                transform: "translate(-50%, -50%)",
                zIndex: 9999,
                background: "#fff",
                borderRadius: "8px",
                border: "1px solid #e5e7eb",
                boxShadow: "0 10px 25px rgba(0, 0, 0, 0.15)",
                minWidth: "320px",
                maxWidth: "400px",
                padding: "12px",
              }}
            >
              <div
                style={{
                  background: "#fff",
                  borderRadius: "6px",
                  overflow: "hidden",
                }}
              >
                {insertOptions.map((option, index) => (
                  <div
                    key={option.key}
                    onClick={option.onClick}
                    style={{
                      padding: "10px 12px",
                      borderBottom:
                        index === insertOptions.length - 1
                          ? "none"
                          : "1px solid #f3f4f6",
                      cursor: "pointer",
                      transition: "background-color 0.2s ease",
                    }}
                    className="hover:bg-gray-50"
                  >
                    <div
                      style={{
                        fontSize: "11px",
                        color: "#6b7280",
                        fontFamily: "monospace",
                        wordBreak: "break-all",
                        lineHeight: "1.4",
                      }}
                    >
                      {option.preview}
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>
      </div>
      <div style={{ marginTop: "20px", marginLeft: "50px" }}>
        <NotebookMiddleBottom
          data={data}
          isLoading={isLoading}
          setSql={setSql}
          sql={sql}
          onClearData={clearData}
          onLoadMore={loadMore}
          hasMore={hasMore}
          isLoadingMore={isLoadingMore}
        />
      </div>
    </div>
  );
}

export default memo(NotebookMiddle);
