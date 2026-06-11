import { Tabs, Tab } from "@heroui/react";
import { useTranslation } from "@/i18n";
import { memo, useEffect, useState } from "react";
import DataTable from "./notebook-middle-table";
import QueryHistory from "./notebook-middle-history";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faCheckCircle,
  faTimesCircle,
} from "@fortawesome/free-solid-svg-icons";

interface ColumnTypeInfo {
  column_name: string;
  arrow_type: string;
  default_sql_type: string;
}

interface NotebookMiddleBottomProps {
  data: {
    header: string[];
    columns: ColumnTypeInfo[];
    rows: string[][];
    query_time: string;
  };
  isLoading: boolean;
  setSql: (sql: string) => void;
  sql: string;
  onClearData?: () => void;
  onLoadMore?: () => Promise<void>;
  hasMore?: boolean;
  isLoadingMore?: boolean;
}

function NotebookMiddleBottom({
  data,
  isLoading,
  setSql,
  sql,
  onClearData,
  onLoadMore,
  hasMore = false,
  isLoadingMore = false,
}: NotebookMiddleBottomProps) {
  const { t } = useTranslation();
  const [activeTab, setActiveTab] = useState("results");

  // Switch to results when a query starts so history-tab users see the output.
  useEffect(() => {
    if (isLoading) {
      setActiveTab("results");
    }
  }, [isLoading]);

  // 判断查询状态
  const getQueryStatus = () => {
    if (data.header.length === 0 && data.rows.length === 0) return null;
    if (
      data.header[0] === "Error" ||
      (data.header[0] === "Status" &&
        data.rows[0]?.[0]?.toLowerCase().includes("cancelled"))
    ) {
      return "failed";
    }
    return "success";
  };

  const queryStatus = getQueryStatus();

  const resultsTitle = (
    <span className="flex items-center gap-2">
      {t("notebook.resultsTab")}
      {queryStatus === "success" && (
        <FontAwesomeIcon
          icon={faCheckCircle}
          className="text-green-500"
          style={{ fontSize: "0.9em" }}
        />
      )}
      {queryStatus === "failed" && (
        <FontAwesomeIcon
          icon={faTimesCircle}
          className="text-red-500"
          style={{ fontSize: "0.9em" }}
        />
      )}
    </span>
  );

  const queryDuration = data.query_time?.trim();
  const showDuration =
    Boolean(queryDuration) && queryDuration !== "-" && !isLoading;

  return (
    <div className="relative flex w-full flex-col">
      <Tabs
        variant="underlined"
        selectedKey={activeTab}
        onSelectionChange={(key) => setActiveTab(String(key))}
        destroyInactiveTabPanel={false}
        shouldSelectOnPressUp={false}
        classNames={{
          base: "w-full",
          tabList: showDuration ? "pr-28" : undefined,
        }}
      >
        <Tab key="history" title={t("notebook.history.title")}>
          <QueryHistory
            setSql={setSql}
            isActive={activeTab === "history"}
          />
        </Tab>
        <Tab key="results" title={resultsTitle}>
          <DataTable
            data={data}
            isLoading={isLoading}
            sql={sql}
            onClear={onClearData}
            onLoadMore={onLoadMore}
            hasMore={hasMore}
            isLoadingMore={isLoadingMore}
          />
        </Tab>
      </Tabs>
      {showDuration && (
        <div
          className="pointer-events-none absolute right-2 top-0 flex h-10 items-center"
          aria-label={`${t("notebook.queryDuration")}: ${queryDuration}`}
        >
          <div className="pointer-events-auto flex items-center gap-1.5 whitespace-nowrap rounded-md border border-default-200 bg-default-50 px-2.5 py-1 text-xs dark:border-default-100 dark:bg-default-100/50">
            <span className="text-default-500">{t("notebook.queryDuration")}</span>
            <span className="font-mono text-sm font-medium text-success-600">
              {queryDuration}
            </span>
          </div>
        </div>
      )}
    </div>
  );
}

export default memo(NotebookMiddleBottom);
