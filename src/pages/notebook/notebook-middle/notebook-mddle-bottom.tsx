import { Tabs, Tab } from "@heroui/react";
import { memo, useState } from "react";
import DataTable from "./notebook-middle-table";
import QueryHistory from "./notebook-middle-history";
import { invoke } from "@tauri-apps/api/core";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faCheckCircle,
  faTimesCircle,
} from "@fortawesome/free-solid-svg-icons";

interface NotebookMiddleBottomProps {
  data: {
    header: string[];
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
  const [queryHistory, setQueryHistory] = useState<
    { sql: string; created_at: string; status: string }[]
  >([]);
  const [isHistoryLoaded, setIsHistoryLoaded] = useState(false);

  const loadQueryHistory = async () => {
    if (!isHistoryLoaded) {
      try {
        const history = (await invoke("sql_history", {})) as {
          sql: string;
          created_at: string;
          status: string;
        }[];
        setQueryHistory(history);
        setIsHistoryLoaded(true);
      } catch (error) {
        console.error("Failed to load query history:", error);
      }
    }
  };

  const handleTabChange = async (key: string | number) => {
    if (key === "history") {
      await loadQueryHistory();
    }
  };

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
      Results
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

  const queryTimeTitle = (
    <span className="text-gray-500 cursor-default">
      Query Time (
      <span className="text-green-600 font-medium">
        {data.query_time ?? "-"}
      </span>
      )
    </span>
  );

  return (
    <div className="flex w-full flex-col">
      <Tabs
        variant="underlined"
        defaultSelectedKey="results"
        onSelectionChange={handleTabChange}
        destroyInactiveTabPanel={false}
      >
        <Tab key="history" title="Query History">
          <QueryHistory setSql={setSql} data={queryHistory} />
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
        <Tab
          key="query_time"
          title={queryTimeTitle}
          disabled={true}
          className={`pointer-events-none ${
            data.query_time && data.query_time !== "-"
              ? "opacity-100"
              : "opacity-60"
          }`}
        />
      </Tabs>
    </div>
  );
}

export default memo(NotebookMiddleBottom);
