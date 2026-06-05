import { memo } from "react";
import NotebookLeftSource from "./notebook-left-source";
import NotebookLeftBottom from "./notebook-left-bottom";
import { SavedQueryItem } from "./notebook-left-saved-queries";

interface NotebookLeftProps {
  source: string;
  setSource: (source: string) => void;
  setSql: (sql: string) => void;
  savedQueries: SavedQueryItem[];
  onDeleteSavedQuery: (id: number) => void;
}

function NotebookLeft({
  source,
  setSource,
  setSql,
  savedQueries,
  onDeleteSavedQuery,
}: NotebookLeftProps) {
  return (
    <div style={{ display: "flex", flexDirection: "column", height: "100vh" }}>
      <div style={{ flex: "4", minHeight: 0, display: "flex", flexDirection: "column" }}>
        <NotebookLeftSource
          source={source}
          setSource={setSource}
          setSql={setSql}
          savedQueries={savedQueries}
          onDeleteSavedQuery={onDeleteSavedQuery}
        />
      </div>
      <div
        style={{
          height: "1px",
          backgroundColor: "rgba(17, 17, 17, 0.15)",
          width: "100%",
        }}
      />
      <div style={{ flex: "1" }}>
        <NotebookLeftBottom />
      </div>
    </div>
  );
}

export default memo(NotebookLeft);
