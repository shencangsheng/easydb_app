import { useTranslation } from "@/i18n";
import { faBookmark, faTrash } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { Button } from "@heroui/react";
import { memo, useCallback, useMemo, useState } from "react";
import { createPortal } from "react-dom";

export const SAVED_QUERIES_SOURCE = "__saved_queries__";

export interface SavedQueryItem {
  id: number;
  name: string;
  sql: string;
  created_at: string;
}

interface NotebookLeftSavedQueriesProps {
  items: SavedQueryItem[];
  setSql: (sql: string) => void;
  onDelete: (id: number) => void;
}

const SQL_PREVIEW_LENGTH = 80;

interface SqlTooltipState {
  sql: string;
  top: number;
  left: number;
}

function truncateSql(sql: string, maxLength: number): string {
  const normalized = sql.replace(/\s+/g, " ").trim();
  if (normalized.length <= maxLength) {
    return normalized;
  }
  return `${normalized.substring(0, maxLength)}...`;
}

function NotebookLeftSavedQueries({
  items,
  setSql,
  onDelete,
}: NotebookLeftSavedQueriesProps) {
  const { translate } = useTranslation();
  const [filterText, setFilterText] = useState("");
  const [sqlTooltip, setSqlTooltip] = useState<SqlTooltipState | null>(null);

  const filteredItems = useMemo(() => {
    if (!filterText.trim()) {
      return items;
    }
    const lower = filterText.toLowerCase();
    return items.filter(
      (item) =>
        item.name.toLowerCase().includes(lower) ||
        item.sql.toLowerCase().includes(lower),
    );
  }, [items, filterText]);

  const showSqlTooltip = useCallback(
    (sql: string, element: HTMLElement) => {
      const rect = element.getBoundingClientRect();
      const maxWidth = 420;
      const margin = 8;
      let left = rect.right + margin;
      if (left + maxWidth > window.innerWidth - margin) {
        left = Math.max(margin, rect.left - maxWidth - margin);
      }
      setSqlTooltip({
        sql,
        top: rect.top,
        left,
      });
    },
    [],
  );

  const hideSqlTooltip = useCallback(() => {
    setSqlTooltip(null);
  }, []);

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        minHeight: 0,
        flex: 1,
        overflow: "hidden",
      }}
    >
      {items.length > 0 && (
        <input
          type="text"
          value={filterText}
          onChange={(e) => setFilterText(e.target.value)}
          style={{
            width: "calc(100% - 20px)",
            padding: "8px",
            margin: "10px",
            border: "1px solid #ccc",
            borderRadius: "4px",
            fontSize: "14px",
          }}
        />
      )}
      <div style={{ flex: 1, overflowY: "auto", minHeight: 0 }}>
        {items.length === 0 ? (
          <p
            style={{
              fontSize: "13px",
              color: "#9ca3af",
              textAlign: "left",
              padding: "8px 15px",
            }}
          >
            {translate("notebook.savedQueries.emptyDescription")}
          </p>
        ) : filteredItems.length === 0 ? (
          <p
            style={{
              fontSize: "13px",
              color: "#9ca3af",
              textAlign: "left",
              padding: "8px 15px",
            }}
          >
            {translate("notebook.savedQueries.noResults")}
          </p>
        ) : (
          <div
            role="list"
            aria-label={translate("notebook.savedQueries.title")}
            style={{ width: "100%" }}
          >
            {filteredItems.map((item) => (
              <div
                key={item.id}
                role="listitem"
                tabIndex={0}
                onClick={() => setSql(item.sql)}
                onKeyDown={(e) => {
                  if (e.key === "Enter" || e.key === " ") {
                    e.preventDefault();
                    setSql(item.sql);
                  }
                }}
                onMouseEnter={(e) => showSqlTooltip(item.sql, e.currentTarget)}
                onMouseLeave={hideSqlTooltip}
                onFocus={(e) => showSqlTooltip(item.sql, e.currentTarget)}
                onBlur={hideSqlTooltip}
                style={{
                  display: "flex",
                  alignItems: "flex-start",
                  gap: "10px",
                  padding: "14px 12px",
                  minHeight: "88px",
                  cursor: "pointer",
                  borderBottom: "1px solid rgba(17, 17, 17, 0.08)",
                  transition: "background-color 0.15s",
                }}
                onMouseOver={(e) => {
                  e.currentTarget.style.backgroundColor = "#f5f5f5";
                }}
                onMouseOut={(e) => {
                  e.currentTarget.style.backgroundColor = "";
                }}
              >
                <FontAwesomeIcon
                  icon={faBookmark}
                  color="#87CEEB"
                  style={{ marginTop: "4px", fontSize: "16px" }}
                />
                <div
                  style={{
                    flex: 1,
                    minWidth: 0,
                    textAlign: "left",
                  }}
                >
                  <div
                    style={{
                      fontSize: "15px",
                      fontWeight: 600,
                      lineHeight: 1.35,
                      marginBottom: "6px",
                      wordBreak: "break-word",
                    }}
                  >
                    {item.name}
                  </div>
                  <div
                    style={{
                      fontSize: "13px",
                      color: "#6b7280",
                      lineHeight: 1.4,
                      marginBottom: "8px",
                      overflow: "hidden",
                      textOverflow: "ellipsis",
                      whiteSpace: "nowrap",
                    }}
                  >
                    {truncateSql(item.sql, SQL_PREVIEW_LENGTH)}
                  </div>
                  <div
                    style={{
                      fontSize: "12px",
                      color: "#9ca3af",
                    }}
                  >
                    {item.created_at}
                  </div>
                </div>
                <Button
                  isIconOnly
                  size="sm"
                  variant="light"
                  aria-label={translate("notebook.savedQueries.delete")}
                  onPress={(e) => {
                    e?.stopPropagation?.();
                    onDelete(item.id);
                  }}
                  onClick={(e) => e.stopPropagation()}
                  onMouseEnter={(e) => e.stopPropagation()}
                  style={{ minWidth: "32px", width: "32px", height: "32px" }}
                >
                  <FontAwesomeIcon icon={faTrash} color="#9ca3af" />
                </Button>
              </div>
            ))}
          </div>
        )}
      </div>
      {sqlTooltip &&
        createPortal(
          <div
            role="tooltip"
            style={{
              position: "fixed",
              top: sqlTooltip.top,
              left: sqlTooltip.left,
              zIndex: 10000,
              maxWidth: "420px",
              maxHeight: "min(360px, 70vh)",
              overflow: "auto",
              padding: "12px 14px",
              fontSize: "12px",
              lineHeight: 1.5,
              fontFamily:
                "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace",
              whiteSpace: "pre-wrap",
              wordBreak: "break-word",
              color: "#f9fafb",
              backgroundColor: "#1f2937",
              borderRadius: "8px",
              boxShadow: "0 8px 24px rgba(0, 0, 0, 0.18)",
              pointerEvents: "none",
            }}
          >
            {sqlTooltip.sql}
          </div>,
          document.body,
        )}
    </div>
  );
}

export default memo(NotebookLeftSavedQueries);
