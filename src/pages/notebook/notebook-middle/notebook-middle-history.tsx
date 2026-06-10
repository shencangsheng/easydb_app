import { formatRelativeTime } from "@/utils/date-util";
import { useTranslation } from "@/i18n";
import { invoke } from "@tauri-apps/api/core";
import { ask, message } from "@tauri-apps/plugin-dialog";
import { memo, useCallback, useEffect, useMemo, useRef, useState } from "react";

const HISTORY_LIMIT_STORAGE_KEY = "query-history-limit";
const HISTORY_LIMIT_OPTIONS = [50, 100, 200, 500, 0] as const;
const SEARCH_DEBOUNCE_MS = 300;

type HistoryItem = {
  sql: string;
  created_at: string;
  status: string;
};

interface QueryHistoryProps {
  setSql: (sql: string) => void;
  isActive: boolean;
}

function loadStoredLimit(): number {
  if (typeof window === "undefined") {
    return 50;
  }
  const saved = localStorage.getItem(HISTORY_LIMIT_STORAGE_KEY);
  if (saved === null) {
    return 50;
  }
  const parsed = Number(saved);
  return HISTORY_LIMIT_OPTIONS.includes(parsed as (typeof HISTORY_LIMIT_OPTIONS)[number])
    ? parsed
    : 50;
}

function QueryHistory({ setSql, isActive }: QueryHistoryProps) {
  const { t } = useTranslation();
  const [data, setData] = useState<HistoryItem[]>([]);
  const [searchText, setSearchText] = useState("");
  const [limit, setLimit] = useState(loadStoredLimit);
  const [isLoading, setIsLoading] = useState(false);
  const [showDeleteMenu, setShowDeleteMenu] = useState(false);
  const deleteMenuRef = useRef<HTMLDivElement>(null);
  const searchDebounceRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const requestCountRef = useRef(0);

  const fetchHistory = useCallback(
    async (keyword: string, displayLimit: number) => {
      const requestId = ++requestCountRef.current;
      setIsLoading(true);
      try {
        const isSearching = keyword.trim().length > 0;
        const history = (await invoke("sql_history", {
          limit: isSearching ? 0 : displayLimit,
          keyword: isSearching ? keyword.trim() : null,
        })) as HistoryItem[];
        if (requestId === requestCountRef.current) {
          setData(history);
        }
      } catch (error) {
        console.error("Failed to load query history:", error);
      } finally {
        if (requestId === requestCountRef.current) {
          setIsLoading(false);
        }
      }
    },
    []
  );

  useEffect(() => {
    if (!isActive) {
      return;
    }
    if (searchDebounceRef.current) {
      clearTimeout(searchDebounceRef.current);
    }
    searchDebounceRef.current = setTimeout(() => {
      void fetchHistory(searchText, limit);
    }, searchText.trim() ? SEARCH_DEBOUNCE_MS : 0);

    return () => {
      if (searchDebounceRef.current) {
        clearTimeout(searchDebounceRef.current);
      }
    };
  }, [isActive, searchText, limit, fetchHistory]);

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (
        deleteMenuRef.current &&
        !deleteMenuRef.current.contains(event.target as Node)
      ) {
        setShowDeleteMenu(false);
      }
    };
    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  const handleLimitChange = useCallback((value: number) => {
    setLimit(value);
    localStorage.setItem(HISTORY_LIMIT_STORAGE_KEY, String(value));
  }, []);

  const handleRowClick = useCallback(
    (sql: string) => {
      setSql(sql);
    },
    [setSql]
  );

  const handleDelete = useCallback(
    async (daysAgo: number | null) => {
      setShowDeleteMenu(false);

      const confirmed = await ask(
        daysAgo === null
          ? t("notebook.history.deleteConfirmAll")
          : t("notebook.history.deleteConfirmBefore").replace(
              "{{days}}",
              String(daysAgo)
            ),
        {
          title: t("notebook.history.deleteHistory"),
          kind: "warning",
          okLabel: t("notebook.savedQueries.confirm"),
          cancelLabel: t("notebook.savedQueries.cancel"),
        }
      );

      if (!confirmed) {
        return;
      }

      try {
        const deleted = (await invoke("delete_sql_history_before", {
          days_ago: daysAgo,
        })) as number;
        await fetchHistory(searchText, limit);
        if (deleted > 0) {
          await message(
            t("notebook.history.deleteSuccess").replace(
              "{{count}}",
              String(deleted)
            ),
            {
              title: t("notebook.history.deleteHistory"),
              kind: "info",
              okLabel: t("notebook.savedQueries.confirm"),
            }
          );
        }
      } catch (error) {
        console.error("Failed to delete query history:", error);
      }
    },
    [t, fetchHistory, searchText, limit]
  );

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

  const historyRows = useMemo(
    () =>
      data.map((value, index) => (
        <tr
          key={`${value.created_at}-${index}`}
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

  const isSearching = searchText.trim().length > 0;

  return (
    <div
      style={{
        height: "calc(40vh - 50px)",
        display: "flex",
        flexDirection: "column",
      }}
    >
      <div className="p-2 border-b border-gray-200">
        <div className="flex items-center gap-2">
          <input
            type="text"
            placeholder={
              isSearching
                ? t("notebook.history.searchAllHint")
                : t("notebook.history.searchPlaceholder")
            }
            value={searchText}
            onChange={(e) => setSearchText(e.target.value)}
            autoComplete="off"
            spellCheck="false"
            inputMode="search"
            className="flex-1 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            style={{ fontSize: "14px" }}
          />
          <div className="flex items-center gap-2 shrink-0 text-sm text-gray-600">
            <span>{t("notebook.history.limitLabel")}:</span>
            <select
              value={limit}
              onChange={(e) => handleLimitChange(Number(e.target.value))}
              disabled={isSearching}
              className="px-2 py-1 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {HISTORY_LIMIT_OPTIONS.map((option) => (
                <option key={option} value={option}>
                  {option === 0 ? t("notebook.history.limitAll") : option}
                </option>
              ))}
            </select>
            <div className="relative" ref={deleteMenuRef}>
              <button
                type="button"
                onClick={() => setShowDeleteMenu((prev) => !prev)}
                className="px-3 py-2 text-sm border border-gray-300 rounded-md hover:bg-gray-50 whitespace-nowrap"
              >
                {t("notebook.history.deleteHistory")}
              </button>
              {showDeleteMenu && (
                <div className="absolute right-0 top-full mt-1 z-10 bg-white border border-gray-200 rounded-md shadow-lg min-w-[140px]">
                  <button
                    type="button"
                    className="w-full text-left px-3 py-2 text-sm hover:bg-gray-50"
                    onClick={() => void handleDelete(7)}
                  >
                    {t("notebook.history.deleteBefore7Days")}
                  </button>
                  <button
                    type="button"
                    className="w-full text-left px-3 py-2 text-sm hover:bg-gray-50"
                    onClick={() => void handleDelete(30)}
                  >
                    {t("notebook.history.deleteBefore30Days")}
                  </button>
                  <button
                    type="button"
                    className="w-full text-left px-3 py-2 text-sm hover:bg-gray-50"
                    onClick={() => void handleDelete(90)}
                  >
                    {t("notebook.history.deleteBefore90Days")}
                  </button>
                  <button
                    type="button"
                    className="w-full text-left px-3 py-2 text-sm text-red-600 hover:bg-red-50 border-t border-gray-100"
                    onClick={() => void handleDelete(null)}
                  >
                    {t("notebook.history.deleteAll")}
                  </button>
                </div>
              )}
            </div>
          </div>
        </div>
      </div>

      <div
        style={{
          flex: 1,
          overflow: "auto",
        }}
      >
        {isLoading ? (
          <div className="flex items-center justify-center h-full text-gray-400 text-sm">
            {t("notebook.history.loading")}
          </div>
        ) : data.length === 0 ? (
          searchText.trim() ? (
            <div className="flex flex-col items-center justify-center h-full text-gray-500">
              <div className="text-4xl mb-4">🔍</div>
              <div className="text-lg font-medium mb-2">
                {t("notebook.history.noResults")}
              </div>
              <div className="text-sm text-gray-400">
                {t("notebook.history.noResultsDescription")}
              </div>
            </div>
          ) : (
            emptyStateContent
          )
        ) : (
          <table className="w-full border-collapse border border-gray-200">
            <tbody>{historyRows}</tbody>
          </table>
        )}
      </div>
    </div>
  );
}

export default memo(QueryHistory);
