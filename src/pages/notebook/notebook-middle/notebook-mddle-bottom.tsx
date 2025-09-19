import { Tabs, Tab } from "@heroui/react";
import { memo, useState } from "react";
import DataTable from "./notebook-middle-table";
import QueryHistory from "./notebook-middle-history";

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

  return (
    <div className="flex w-full flex-col">
      <Tabs
        variant="underlined"
        defaultSelectedKey="results"
        onSelectionChange={async (key) => {
          if (key === "history") {
            alert("暂未实现");
          }
        }}
      >
        <Tab key="history" title="Query History">
          <QueryHistory setSql={setSql} data={queryHistory} />
        </Tab>
        <Tab key="results" title={`Results`}>
          <DataTable data={data} isLoading={isLoading} sql={sql} />
        </Tab>
        <Tab
          key="query_time"
          title={
            <span>
              Query Time ({" "}
              <span style={{ color: "green" }}>{data.query_time ?? "-"}</span> )
            </span>
          }
          disabled={true}
        >
          <div>
            <p>Query Time</p>
            <p>{data.query_time ?? "-"}</p>
          </div>
        </Tab>
      </Tabs>
    </div>
  );
}

export default memo(NotebookMiddleBottom);
