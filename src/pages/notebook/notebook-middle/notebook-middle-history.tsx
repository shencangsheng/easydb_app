import { formatRelativeTime } from "@/utils/date-util";
import { memo, useCallback, useMemo } from "react";
import { useTranslation } from "@/i18n";

interface QueryHistoryProps {
  setSql: (sql: string) => void;
  data: {
    sql: string;
    created_at: string;
    status: string;
  }[];
}

function QueryHistory({ setSql, data }: QueryHistoryProps) {
  const { t } = useTranslation();

  // 使用 useCallback 缓存点击处理函数
  const handleRowClick = useCallback(
    (sql: string) => {
      setSql(sql);
    },
    [setSql]
  );

  // 使用 useMemo 缓存空状态内容
  const emptyStateContent = useMemo(
    () => (
      <div className="flex flex-col items-center justify-center h-full text-gray-500">
        <div className="text-6xl mb-4">
          {t("notebook.history.emptyState.icon")}
        </div>
        <div className="text-lg font-medium mb-2">
          {t("notebook.history.emptyState.title")}
        </div>
        <div className="text-sm text-gray-400">
          {t("notebook.history.emptyState.description")}
        </div>
      </div>
    ),
    [t]
  );

  // 使用 useMemo 缓存历史记录行
  const historyRows = useMemo(
    () =>
      data.map((value, index) => (
        <tr
          key={index}
          className="border-b border-gray-200"
          onClick={() => handleRowClick(value.sql)}
          style={{
            cursor: "pointer",
            transition: "background-color 0.2s",
          }}
          onMouseOver={(e) => {
            e.currentTarget.style.backgroundColor = "#f5f5f5";
          }}
          onMouseOut={(e) => {
            e.currentTarget.style.backgroundColor = "";
          }}
        >
          <td className="py-2 px-4 text-left bg-gray-50 font-medium">
            {formatRelativeTime(value.created_at)}
          </td>
          <td className="py-2 px-4 text-left">
            <span
              style={{
                display: "inline-flex",
                alignItems: "center",
                marginRight: "8px",
              }}
            >
              {value.status === "successful" ? (
                <span style={{ color: "green" }}>✓</span>
              ) : value.status === "fail" ? (
                <span style={{ color: "red" }}>✗</span>
              ) : (
                <span style={{ color: "orange" }}>!</span>
              )}
            </span>
          </td>
          <td
            className="py-2 px-4 text-left"
            style={{
              cursor: "pointer",
              transition: "background-color 0.2s",
            }}
            title={t("notebook.history.clickToUse")}
          >
            {value.sql.length > 500
              ? value.sql.substring(0, 500) + "..."
              : value.sql}
          </td>
        </tr>
      )),
    [data, handleRowClick, t]
  );

  return (
    <div
      style={{
        height: "calc(40vh - 50px)",
        overflow: "auto",
      }}
    >
      {data.length === 0 ? (
        emptyStateContent
      ) : (
        <table className="w-full border-collapse border border-gray-200">
          <tbody>{historyRows}</tbody>
        </table>
      )}
    </div>
  );
}

export default memo(QueryHistory);
