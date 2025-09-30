import { Tabs, Tab } from "@heroui/react";
import { memo, useState, useCallback, useMemo } from "react";
import DataTable from "./notebook-middle-table";
import QueryHistory from "./notebook-middle-history";
import { invoke } from "@tauri-apps/api/core";

interface NotebookMiddleBottomProps {
  data: {
    header: string[];
    rows: string[][];
    query_time: string;
  };
  isLoading: boolean;
  setSql: (sql: string) => void;
  sql: string;
}

function NotebookMiddleBottom({
  data,
  isLoading,
  setSql,
  sql,
}: NotebookMiddleBottomProps) {
  const [queryHistory, setQueryHistory] = useState<
    {
      sql: string;
      created_at: string;
      status: string;
    }[]
  >([]);
  const [isHistoryLoaded, setIsHistoryLoaded] = useState(false);

  // 使用 useCallback 缓存历史数据获取函数
  const loadQueryHistory = useCallback(async () => {
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
  }, [isHistoryLoaded]);

  // 使用 useCallback 缓存标签页切换处理函数
  const handleTabChange = useCallback(
    async (key: string | number) => {
      if (key === "history") {
        await loadQueryHistory();
      }
    },
    [loadQueryHistory]
  );

  // 使用 useMemo 缓存查询时间显示
  const queryTimeTitle = useMemo(
    () => (
      <span className="text-gray-500 cursor-default">
        Query Time (
        <span className="text-green-600 font-medium">
          {data.query_time ?? "-"}
        </span>
        )
      </span>
    ),
    [data.query_time]
  );

  return (
    <div className="flex w-full flex-col">
      <Tabs
        variant="underlined"
        defaultSelectedKey="results"
        onSelectionChange={handleTabChange}
      >
        <Tab key="history" title="Query History">
          <QueryHistory setSql={setSql} data={queryHistory} />
        </Tab>
        <Tab key="results" title={`Results`}>
          <DataTable data={data} isLoading={isLoading} sql={sql} />
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
        ></Tab>
      </Tabs>
    </div>
  );
}

export default memo(NotebookMiddleBottom);
