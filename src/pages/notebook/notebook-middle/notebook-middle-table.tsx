import {
  faDownload,
  faFileCsv,
  faSpinner,
  faCheckCircle,
  faTimes,
  faFileCode,
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
}

interface ExportResult {
  query_time: string;
  file_name: string;
}

function DataTable({ data, isLoading, sql }: TableProps) {
  const [isDownloading, setIsDownloading] = useState(false);
  const [exportResult, setExportResult] = useState<ExportResult | null>(null);
  const [isTableNameModalOpen, setIsTableNameModalOpen] = useState(false);
  const [tableName, setTableName] = useState("table_name");
  const [maxValuesPerInsert, setMaxValuesPerInsert] = useState(1000);
  const [sqlStatementType, setSqlStatementType] = useState("INSERT");
  const [whereColumn, setWhereColumn] = useState("");
  const [databaseDialect, setDatabaseDialect] = useState("MySQL");
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
    dialect?: string
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
      });
      setExportResult(result);
    } catch (error) {
      console.error("Export failed:", error);
    } finally {
      setIsDownloading(false);
    }
  }

  async function handleSqlExport() {
    // 重置所有状态到初始值
    setSqlStatementType("INSERT");
    setTableName("table_name");
    setMaxValuesPerInsert(1000);
    setWhereColumn("");
    setDatabaseDialect("MySQL");
    setIsTableNameModalOpen(true);
  }

  async function confirmSqlExport() {
    setIsTableNameModalOpen(false);
    await exportResults(
      "SQL",
      tableName,
      maxValuesPerInsert,
      sqlStatementType,
      whereColumn,
      databaseDialect
    );
  }

  return (
    <div
      style={{
        display: "flex",
        height: "calc(40vh - 50px)",
        width: "100%",
        overflow: "hidden",
        // border: "1px solid rgba(17, 17, 17, 0.15)",
      }}
    >
      {/* 下载完成提示 */}
      {exportResult && (
        <div
          style={{
            position: "absolute",
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
          }}
        >
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
            style={{
              background: "none",
              border: "none",
              color: "#1e40af",
              cursor: "pointer",
              padding: "4px",
              borderRadius: "4px",
              display: "flex",
              alignItems: "center",
              justifyContent: "center",
            }}
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

      <div
        style={{
          justifyContent: "flex-end",
          paddingTop: "15px",
        }}
      >
        <Dropdown
          placement="bottom-start"
          isDisabled={data.rows.length === 0 || isDownloading}
        >
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
          <DropdownMenu aria-label="Static Actions">
            <DropdownItem
              key="csv"
              onPress={async () => await exportResults("CSV")}
            >
              <FontAwesomeIcon
                icon={faFileCsv}
                style={{ marginRight: "5px" }}
              />
              CSV
            </DropdownItem>
            <DropdownItem
              key="tsv"
              onPress={async () => await exportResults("TSV")}
            >
              <FontAwesomeIcon
                icon={faFileCsv}
                style={{ marginRight: "5px" }}
              />
              TSV
            </DropdownItem>
            <DropdownItem key="sql" onPress={handleSqlExport}>
              <FontAwesomeIcon
                icon={faFileCode}
                style={{ marginRight: "5px" }}
              />
              SQL
            </DropdownItem>
          </DropdownMenu>
        </Dropdown>
      </div>

      <DataResult data={data} isLoading={isLoading} />

      {/* 表名输入模态框 */}
      <Modal
        isOpen={isTableNameModalOpen}
        onOpenChange={setIsTableNameModalOpen}
        placement="center"
        size="lg"
        classNames={{
          base: "bg-background",
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

              <ModalBody className="gap-6 py-6">
                {/* SQL 语句类型选择 */}
                <div className="space-y-3">
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
                <div className="space-y-2">
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
                <div className="space-y-2">
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
                <div className="space-y-2">
                  <Input
                    label={translate("notebook.export.maxValuesPerInsert")}
                    placeholder={translate(
                      "notebook.export.maxValuesPerInsertPlaceholder"
                    )}
                    value={maxValuesPerInsert.toString()}
                    onChange={(e) =>
                      setMaxValuesPerInsert(parseInt(e.target.value) || 1000)
                    }
                    size="lg"
                    type="number"
                    min="1"
                    variant="bordered"
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
                <div className="space-y-2">
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
              </ModalBody>

              <ModalFooter className="gap-3 pt-4">
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
                  isDisabled={
                    !tableName.trim() ||
                    (sqlStatementType === "INSERT" && maxValuesPerInsert < 1) ||
                    (sqlStatementType === "UPDATE" && !whereColumn.trim())
                  }
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
