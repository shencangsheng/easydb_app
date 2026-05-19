import {
  faDownload,
  faFileCsv,
  faSpinner,
  faCheckCircle,
  faTimes,
  faFileCode,
  faTrash,
} from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  Button,
  Dropdown,
  DropdownItem,
  DropdownMenu,
  DropdownTrigger,
  Modal,
  ModalContent,
  ModalHeader,
  ModalBody,
  ModalFooter,
  Input,
  Tabs,
  Tab,
  Select,
  SelectItem,
} from "@heroui/react";
import { memo, useState, useEffect } from "react";
import DataResult from "./notebook-middle-data-result";
import { invoke } from "@tauri-apps/api/core";
import { useTranslation } from "../../../i18n";

interface TableProps {
  data: {
    header: string[];
    rows: string[][];
  };
  isLoading: boolean;
  sql: string;
  onClear?: () => void;
  onLoadMore?: () => Promise<void>;
  hasMore?: boolean;
  isLoadingMore?: boolean;
}

interface ExportResult {
  query_time: string;
  file_name: string;
}

interface ColumnTypeInfo {
  column_name: string;
  arrow_type: string;
  default_sql_type: string;
}

interface ExportColumnConfig {
  source_column_name: string;
  export_column_name: string;
  sql_type: string;
}

const SQL_TYPE_OPTIONS = [
  "INT",
  "DOUBLE",
  "BOOL",
  "TEXT",
];

// 预定义静态样式对象，避免每次渲染创建新对象
const CONTAINER_STYLE = {
  display: "flex",
  width: "100%",
  position: "relative",
} as const;

const TOOLBAR_STYLE = {
  display: "flex",
  flexDirection: "column",
  alignItems: "center",
  paddingRight: "8px",
} as const;

const TOAST_STYLE = {
  position: "absolute" as const,
  top: "10px",
  left: "50%",
  transform: "translateX(-50%)",
  zIndex: 1000,
  padding: "12px 16px",
  borderRadius: "8px",
  fontSize: "14px",
  fontWeight: 500,
  backgroundColor: "#f0f9ff",
  color: "#1e40af",
  border: "1px solid #bfdbfe",
  display: "flex",
  alignItems: "center",
  gap: "8px",
  maxWidth: "300px",
  boxShadow: "0 4px 6px -1px rgba(0, 0, 0, 0.1)",
} as const;

const CLOSE_BUTTON_STYLE = {
  background: "none",
  border: "none",
  color: "#1e40af",
  cursor: "pointer",
  padding: "4px",
  borderRadius: "4px",
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
} as const;

const ICON_MARGIN_STYLE = { marginRight: "5px" };

function DataTable({
  data,
  isLoading,
  sql,
  onClear,
  onLoadMore,
  hasMore = false,
  isLoadingMore = false,
}: TableProps) {
  const [isDownloading, setIsDownloading] = useState(false);
  const [exportResult, setExportResult] = useState<ExportResult | null>(null);
  const [isTableNameModalOpen, setIsTableNameModalOpen] = useState(false);
  const [tableName, setTableName] = useState("table_name");
  const [maxValuesPerInsert, setMaxValuesPerInsert] = useState(1000);
  const [sqlStatementType, setSqlStatementType] = useState("INSERT");
  const [whereColumn, setWhereColumn] = useState("");
  const [databaseDialect, setDatabaseDialect] = useState("MySQL");
  const [columnTypes, setColumnTypes] = useState<ColumnTypeInfo[]>([]);
  const [exportColumns, setExportColumns] = useState<ExportColumnConfig[]>([]);
  const [isLoadingColumnTypes, setIsLoadingColumnTypes] = useState(false);
  const { translate } = useTranslation();

  // 自动隐藏提示
  useEffect(() => {
    if (exportResult) {
      const timer = setTimeout(() => {
        setExportResult(null);
      }, 5000); // 5秒后自动隐藏
      return () => clearTimeout(timer);
    }
  }, [exportResult]);

  async function exportResults(
    fileType: string,
    tableName?: string,
    maxValuesPerInsert?: number,
    sqlStatementType?: string,
    whereColumn?: string,
    dialect?: string,
    exportColumns?: ExportColumnConfig[]
  ) {
    try {
      setIsDownloading(true);
      setExportResult(null);

      const result = await invoke<ExportResult>("writer", {
        sql,
        fileType,
        tableName: fileType === "SQL" ? tableName : undefined,
        maxValuesPerInsert: fileType === "SQL" ? maxValuesPerInsert : undefined,
        sqlStatementType: fileType === "SQL" ? sqlStatementType : undefined,
        whereColumn: fileType === "SQL" ? whereColumn : undefined,
        dialect: fileType === "SQL" ? dialect : undefined,
        exportColumns: fileType === "SQL" ? exportColumns : undefined,
      });
      setExportResult(result);
    } catch (error) {
      console.error("Export failed:", error);
    } finally {
      setIsDownloading(false);
    }
  }

  async function handleSqlExport() {
    setSqlStatementType("INSERT");
    setTableName("table_name");
    setMaxValuesPerInsert(1000);
    setWhereColumn("");
    setDatabaseDialect("MySQL");
    setColumnTypes([]);
    setExportColumns([]);
    setIsTableNameModalOpen(true);

    // Fetch column types from backend
    setIsLoadingColumnTypes(true);
    try {
      const result = await invoke<{ columns: ColumnTypeInfo[] }>("fetch_column_types", {
        sql,
      });
      setColumnTypes(result.columns);
      setExportColumns(
        result.columns.map((c) => ({
          source_column_name: c.column_name,
          export_column_name: c.column_name,
          sql_type: c.default_sql_type,
        }))
      );
    } catch (error) {
      console.error("Failed to fetch column types:", error);
      // Fallback: use header names with TEXT type
      const fallback = data.header.map((h) => ({
        column_name: h,
        arrow_type: "Unknown",
        default_sql_type: "TEXT",
      }));
      setColumnTypes(fallback);
      setExportColumns(
        fallback.map((c) => ({
          source_column_name: c.column_name,
          export_column_name: c.column_name,
          sql_type: c.default_sql_type,
        }))
      );
    } finally {
      setIsLoadingColumnTypes(false);
    }
  }

  async function confirmSqlExport() {
    setIsTableNameModalOpen(false);
    await exportResults(
      "SQL",
      tableName,
      maxValuesPerInsert,
      sqlStatementType,
      whereColumn,
      databaseDialect,
      exportColumns
    );
  }

  function updateExportColumn(
    sourceColumnName: string,
    patch: Partial<Pick<ExportColumnConfig, "export_column_name" | "sql_type">>
  ) {
    setExportColumns((prev) =>
      prev.map((col) =>
        col.source_column_name === sourceColumnName ? { ...col, ...patch } : col
      )
    );
  }

  function removeExportColumn(sourceColumnName: string) {
    setExportColumns((prev) =>
      prev.filter((col) => col.source_column_name !== sourceColumnName)
    );
  }

  // 按钮禁用状态
  const isExportDisabled = data.rows.length === 0 || isDownloading;
  const isClearDisabled = data.rows.length === 0 || isLoading;
  const hasValidExportColumns =
    exportColumns.length > 0 &&
    exportColumns.every((col) => col.export_column_name.trim().length > 0);
  const isConfirmDisabled =
    !tableName.trim() ||
    !hasValidExportColumns ||
    (sqlStatementType === "INSERT" && maxValuesPerInsert < 1) ||
    (sqlStatementType === "UPDATE" && !whereColumn.trim());

  return (
    <div style={CONTAINER_STYLE}>
      {/* 下载完成提示 */}
      {exportResult && (
        <div style={TOAST_STYLE}>
          <FontAwesomeIcon icon={faCheckCircle} />
          <div style={{ flex: 1 }}>
            <div style={{ fontWeight: 600, marginBottom: "4px" }}>
              {translate("notebook.export.completed")}
            </div>
            <div style={{ fontSize: "12px", opacity: 0.8 }}>
              {translate("notebook.export.queryTime")}:{" "}
              {exportResult.query_time}
            </div>
            <div style={{ fontSize: "12px", opacity: 0.8 }}>
              {translate("notebook.export.fileName")}:{" "}
              {exportResult.file_name.split("/").pop()}
            </div>
          </div>
          <button
            onClick={() => setExportResult(null)}
            style={CLOSE_BUTTON_STYLE}
            onMouseEnter={(e) => {
              e.currentTarget.style.backgroundColor = "rgba(30, 64, 175, 0.1)";
            }}
            onMouseLeave={(e) => {
              e.currentTarget.style.backgroundColor = "transparent";
            }}
          >
            <FontAwesomeIcon icon={faTimes} size="sm" />
          </button>
        </div>
      )}

      {/* 左侧工具栏 */}
      <div style={TOOLBAR_STYLE}>
        <Dropdown placement="bottom-start" isDisabled={isExportDisabled}>
          <DropdownTrigger>
            <Button
              variant="light"
              isIconOnly
              aria-label="Export data"
              isLoading={isDownloading}
            >
              <FontAwesomeIcon
                icon={isDownloading ? faSpinner : faDownload}
                spin={isDownloading}
              />
            </Button>
          </DropdownTrigger>
          <DropdownMenu aria-label="Export options">
            <DropdownItem key="csv" onPress={() => exportResults("CSV")}>
              <FontAwesomeIcon icon={faFileCsv} style={ICON_MARGIN_STYLE} />
              CSV
            </DropdownItem>
            <DropdownItem key="tsv" onPress={() => exportResults("TSV")}>
              <FontAwesomeIcon icon={faFileCsv} style={ICON_MARGIN_STYLE} />
              TSV
            </DropdownItem>
            <DropdownItem key="sql" onPress={handleSqlExport}>
              <FontAwesomeIcon icon={faFileCode} style={ICON_MARGIN_STYLE} />
              SQL
            </DropdownItem>
          </DropdownMenu>
        </Dropdown>
        <Button
          variant="light"
          isIconOnly
          aria-label={translate("notebook.clear")}
          isDisabled={isClearDisabled}
          onPress={() => {
            if (onClear) {
              onClear();
            }
          }}
          style={{ marginTop: "8px" }}
        >
          <FontAwesomeIcon icon={faTrash} />
        </Button>
      </div>

      {/* 表格区域 */}
      <div className="flex-1 flex flex-col overflow-hidden relative">
        <DataResult
          data={data}
          isLoading={isLoading}
          onLoadMore={onLoadMore}
          hasMore={hasMore}
          isLoadingMore={isLoadingMore}
        />
        {/* Total 统计悬浮标签 */}
        {data.rows.length > 0 && (
          <div className="absolute bottom-2 left-2 px-2 py-0.5 text-[10px] text-default-500 bg-default-100/80 rounded backdrop-blur-sm z-40">
            Total: {data.rows.length.toLocaleString()} rows
          </div>
        )}
      </div>

      {/* 表名输入模态框 */}
      <Modal
        isOpen={isTableNameModalOpen}
        onOpenChange={setIsTableNameModalOpen}
        placement="center"
        size="5xl"
        classNames={{
          base: "bg-background !max-w-[1024px] !max-h-[85vh] flex flex-col",
          backdrop: "bg-black/50",
        }}
      >
        <ModalContent>
          {(onClose) => (
            <>
              <ModalHeader className="flex flex-col gap-2 pb-4">
                <h2 className="text-xl font-semibold text-foreground">
                  {translate("notebook.export.sqlExportSettings")}
                </h2>
                <p className="text-sm text-default-500">
                  {sqlStatementType === "INSERT"
                    ? translate("notebook.export.insertDescription")
                    : translate("notebook.export.updateDescription")}
                </p>
              </ModalHeader>

              <ModalBody className="gap-0 py-0 overflow-hidden flex-1 min-h-0">
                <div className="grid h-full min-h-[480px] grid-cols-[minmax(0,1fr)_minmax(0,1fr)] gap-8">
                  {/* 左侧：基本设置表单 */}
                  <div className="flex flex-col gap-7 overflow-y-auto py-7 pl-2 pr-4 min-w-0">
                    {/* SQL 语句类型选择 */}
                    <div className="space-y-4">
                      <div className="flex items-center gap-2">
                        <label className="text-sm font-medium text-foreground">
                          {translate("notebook.export.sqlStatementType")}
                        </label>
                        <span className="text-danger text-sm">*</span>
                      </div>
                      <Tabs
                        selectedKey={sqlStatementType}
                        onSelectionChange={(key) =>
                          setSqlStatementType(key as string)
                        }
                        variant="solid"
                        color="primary"
                        size="lg"
                        classNames={{
                          base: "w-full",
                          tabList:
                            "gap-0 w-full relative rounded-xl p-1 bg-default-100",
                          cursor: "w-full bg-background shadow-md rounded-lg",
                          tab: "flex-1 px-6 h-12 min-w-0 w-1/2 font-medium",
                          tabContent:
                            "group-data-[selected=true]:text-primary text-center font-semibold",
                        }}
                      >
                        <Tab key="INSERT" title="INSERT" />
                        <Tab key="UPDATE" title="UPDATE" />
                      </Tabs>
                    </div>

                    {/* 表名输入 */}
                    <div className="space-y-3">
                      <Input
                        label={translate("notebook.export.tableName")}
                        placeholder={translate(
                          "notebook.export.tableNamePlaceholder"
                        )}
                        value={tableName}
                        onChange={(e) => setTableName(e.target.value)}
                        autoFocus
                        size="lg"
                        isRequired
                        variant="bordered"
                        labelPlacement="outside"
                        autoComplete="off"
                        autoCapitalize="off"
                        autoCorrect="off"
                        spellCheck="false"
                        classNames={{
                          input: "text-base",
                          label: "text-sm font-medium",
                          inputWrapper:
                            "border-default-200 hover:border-primary-300 focus-within:border-primary-500",
                        }}
                      />
                    </div>

                    {/* WHERE 字段选择 */}
                    <div className="space-y-3">
                      <Select
                        label={translate("notebook.export.whereColumn")}
                        placeholder={translate(
                          "notebook.export.whereColumnPlaceholder"
                        )}
                        selectedKeys={whereColumn ? [whereColumn] : []}
                        onSelectionChange={(keys) => {
                          const selectedKey = Array.from(keys)[0] as string;
                          setWhereColumn(selectedKey || "");
                        }}
                        size="lg"
                        variant="bordered"
                        labelPlacement="outside"
                        isDisabled={sqlStatementType === "INSERT"}
                        isRequired={sqlStatementType === "UPDATE"}
                        classNames={{
                          trigger:
                            "text-base border-default-200 hover:border-primary-300 focus-within:border-primary-500",
                          label: "text-sm font-medium",
                          value: "text-base",
                          listbox: "text-base",
                        }}
                      >
                        {data.header.map((column) => (
                          <SelectItem key={column} className="text-base">
                            {column}
                          </SelectItem>
                        ))}
                      </Select>
                      {sqlStatementType === "INSERT" && (
                        <p className="text-xs text-default-400">
                          {translate("notebook.export.whereColumnDisabledHint")}
                        </p>
                      )}
                    </div>

                    {/* 批次大小设置 */}
                    <div className="space-y-3">
                      <Input
                        label={translate("notebook.export.maxValuesPerInsert")}
                        placeholder={translate(
                          "notebook.export.maxValuesPerInsertPlaceholder"
                        )}
                        value={maxValuesPerInsert.toString()}
                        onChange={(e) => {
                          setMaxValuesPerInsert(parseInt(e.target.value) || 1000);
                        }}
                        size="lg"
                        type="number"
                        min="1"
                        variant="bordered"
                        labelPlacement="outside"
                        isDisabled={sqlStatementType === "UPDATE"}
                        isRequired={sqlStatementType === "INSERT"}
                        classNames={{
                          input: "text-base",
                          label: "text-sm font-medium",
                          inputWrapper:
                            "border-default-200 hover:border-primary-300 focus-within:border-primary-500",
                        }}
                      />
                      {sqlStatementType === "UPDATE" && (
                        <p className="text-xs text-default-400">
                          {translate("notebook.export.batchSizeDisabledHint")}
                        </p>
                      )}
                    </div>

                    {/* 数据库方言选择 */}
                    <div className="space-y-3">
                      <Select
                        label={translate("notebook.export.databaseDialect")}
                        placeholder={translate(
                          "notebook.export.databaseDialectPlaceholder"
                        )}
                        selectedKeys={databaseDialect ? [databaseDialect] : []}
                        onSelectionChange={(keys) => {
                          const selectedKey = Array.from(keys)[0] as string;
                          setDatabaseDialect(selectedKey || "MySQL");
                        }}
                        size="lg"
                        variant="bordered"
                        labelPlacement="outside"
                        defaultSelectedKeys={["MySQL"]}
                        classNames={{
                          trigger:
                            "text-base border-default-200 hover:border-primary-300 focus-within:border-primary-500",
                          label: "text-sm font-medium",
                          value: "text-base",
                          listbox: "text-base",
                        }}
                      >
                        <SelectItem key="MySQL" className="text-base">
                          {translate("notebook.export.mysql")}
                        </SelectItem>
                        <SelectItem key="PostgreSQL" className="text-base">
                          {translate("notebook.export.postgresql")}
                        </SelectItem>
                      </Select>
                    </div>
                  </div>

                  {/* 右侧：导出列类型配置 */}
                  <div className="min-w-0 border-l border-default-200 flex flex-col overflow-hidden bg-default-50/40">
                    <div className="shrink-0 px-6 pt-7 pb-5 border-b border-default-200/80 bg-background/60">
                      <div className="flex items-center justify-between gap-3">
                        <div className="flex items-center gap-2.5 min-w-0">
                          <label className="text-base font-semibold text-foreground">
                            {translate("notebook.export.columnTypes")}
                          </label>
                          {isLoadingColumnTypes && (
                            <FontAwesomeIcon
                              icon={faSpinner}
                              spin
                              className="text-default-400 text-base"
                            />
                          )}
                        </div>
                        {exportColumns.length > 0 && (
                          <span className="shrink-0 text-xs font-medium text-default-500 tabular-nums px-2.5 py-1 rounded-full bg-default-100">
                            {exportColumns.length}
                          </span>
                        )}
                      </div>
                      <p className="text-sm text-default-500 mt-3 leading-relaxed">
                        {translate("notebook.export.columnTypesDescription")}
                      </p>
                    </div>
                    <div className="flex-1 overflow-y-auto px-6 py-5 min-h-0">
                      {exportColumns.length > 0 ? (
                        <div className="space-y-5 pb-5">
                          {exportColumns.map((col) => {
                            const meta = columnTypes.find(
                              (c) => c.column_name === col.source_column_name
                            );
                            return (
                            <div
                              key={col.source_column_name}
                              className="rounded-xl border border-default-200 bg-background p-5 shadow-sm"
                            >
                              <div className="flex items-start justify-between gap-4 mb-5">
                                <div className="min-w-0 flex-1">
                                  <p className="text-xs font-medium uppercase tracking-wide text-default-400 mb-1.5">
                                    {translate("notebook.export.sourceColumn")}
                                  </p>
                                  <p
                                    className="text-base font-semibold text-foreground truncate"
                                    title={col.source_column_name}
                                  >
                                    {col.source_column_name}
                                  </p>
                                  {meta && (
                                    <p className="text-sm text-default-400 mt-1.5 truncate">
                                      {meta.arrow_type}
                                    </p>
                                  )}
                                </div>
                                <Button
                                  isIconOnly
                                  size="sm"
                                  variant="light"
                                  color="danger"
                                  aria-label={translate(
                                    "notebook.export.removeColumn"
                                  )}
                                  onPress={() =>
                                    removeExportColumn(col.source_column_name)
                                  }
                                  className="shrink-0"
                                >
                                  <FontAwesomeIcon
                                    icon={faTrash}
                                    className="text-sm"
                                  />
                                </Button>
                              </div>
                              <div className="space-y-5">
                                <div className="space-y-2.5">
                                  <label
                                    htmlFor={`export-col-${col.source_column_name}`}
                                    className="text-sm font-medium text-foreground"
                                  >
                                    {translate("notebook.export.exportColumnName")}
                                  </label>
                                  <Input
                                    id={`export-col-${col.source_column_name}`}
                                    aria-label={translate(
                                      "notebook.export.exportColumnName"
                                    )}
                                    value={col.export_column_name}
                                    onChange={(e) =>
                                      updateExportColumn(col.source_column_name, {
                                        export_column_name: e.target.value,
                                      })
                                    }
                                    size="lg"
                                    variant="bordered"
                                    labelPlacement="outside"
                                    autoComplete="off"
                                    autoCapitalize="off"
                                    autoCorrect="off"
                                    spellCheck="false"
                                    classNames={{
                                      base: "gap-1",
                                      input: "text-base",
                                      inputWrapper:
                                        "border-default-200 hover:border-primary-300 focus-within:border-primary-500 min-h-11",
                                    }}
                                  />
                                </div>
                                <div className="space-y-2.5">
                                  <label
                                    htmlFor={`sql-type-${col.source_column_name}`}
                                    className="text-sm font-medium text-foreground"
                                  >
                                    {translate("notebook.export.sqlType")}
                                  </label>
                                  <Select
                                    id={`sql-type-${col.source_column_name}`}
                                    aria-label={translate("notebook.export.sqlType")}
                                    selectedKeys={col.sql_type ? [col.sql_type] : []}
                                    onSelectionChange={(keys) => {
                                      const selectedKey = Array.from(keys)[0] as string;
                                      if (selectedKey) {
                                        updateExportColumn(col.source_column_name, {
                                          sql_type: selectedKey,
                                        });
                                      }
                                    }}
                                    size="lg"
                                    variant="bordered"
                                    labelPlacement="outside"
                                    classNames={{
                                      base: "gap-1",
                                      trigger:
                                        "text-base min-h-11 border-default-200 hover:border-primary-300 focus-within:border-primary-500",
                                      value: "text-base",
                                      listbox: "text-base",
                                    }}
                                  >
                                  {SQL_TYPE_OPTIONS.map((type) => (
                                    <SelectItem key={type} className="text-base">
                                      {type}
                                    </SelectItem>
                                  ))}
                                </Select>
                                </div>
                              </div>
                            </div>
                            );
                          })}
                        </div>
                      ) : (
                        <div className="flex flex-col items-center justify-center py-16 px-6 text-center">
                          <p className="text-base text-default-500">
                            {translate("notebook.export.noExportColumns")}
                          </p>
                        </div>
                      )}
                    </div>
                  </div>
                </div>
              </ModalBody>

              <ModalFooter className="gap-4 pt-6 pb-3 border-t border-default-200/60 shrink-0">
                <Button
                  color="default"
                  variant="light"
                  onPress={onClose}
                  size="lg"
                  className="font-medium"
                >
                  {translate("notebook.export.cancel")}
                </Button>
                <Button
                  color="primary"
                  onPress={() => {
                    confirmSqlExport();
                    onClose();
                  }}
                  isDisabled={isConfirmDisabled}
                  size="lg"
                  className="font-medium"
                >
                  {translate("notebook.export.confirmExport")}
                </Button>
              </ModalFooter>
            </>
          )}
        </ModalContent>
      </Modal>
    </div>
  );
}

export default memo(DataTable);
