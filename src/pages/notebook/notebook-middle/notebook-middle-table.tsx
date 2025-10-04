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
    maxValuesPerInsert?: number
  ) {
    try {
      setIsDownloading(true);
      setExportResult(null);

      const result = await invoke<ExportResult>("writer", {
        sql,
        fileType,
        tableName: fileType === "SQL" ? tableName : undefined,
        maxValuesPerInsert: fileType === "SQL" ? maxValuesPerInsert : undefined,
      });
      setExportResult(result);
    } catch (error) {
      console.error("Export failed:", error);
    } finally {
      setIsDownloading(false);
    }
  }

  async function handleSqlExport() {
    setIsTableNameModalOpen(true);
  }

  async function confirmSqlExport() {
    setIsTableNameModalOpen(false);
    await exportResults("SQL", tableName, maxValuesPerInsert);
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
      >
        <ModalContent>
          {(onClose) => (
            <>
              <ModalHeader
                className="flex flex-col gap-1"
                style={{ fontSize: "18px", fontWeight: "600" }}
              >
                {translate("notebook.export.sqlExportSettings")}
              </ModalHeader>
              <ModalBody style={{ fontSize: "16px" }}>
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
                  classNames={{
                    input: "text-lg",
                    label: "text-lg font-medium",
                  }}
                />
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
                  isRequired
                  classNames={{
                    input: "text-lg",
                    label: "text-lg font-medium",
                  }}
                />
              </ModalBody>
              <ModalFooter style={{ fontSize: "16px" }}>
                <Button
                  color="danger"
                  variant="light"
                  onPress={onClose}
                  size="lg"
                  className="text-lg font-medium"
                >
                  {translate("notebook.export.cancel")}
                </Button>
                <Button
                  color="primary"
                  onPress={() => {
                    confirmSqlExport();
                    onClose();
                  }}
                  isDisabled={!tableName.trim() || maxValuesPerInsert < 1}
                  size="lg"
                  className="text-lg font-medium"
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
