import { invoke } from "@tauri-apps/api/core";
import { useCallback, useEffect, useState } from "react";
import NotebookHeader from "./notebook-header/notebook-header";
import NotebookLeft from "./notebook-left/notebook-left";
import NotebookMiddle from "./notebook-middle/notebook-middle";
import NotebookRight from "./notebook-right/notebook-right";
import { SavedQueryItem } from "./notebook-left/notebook-left-saved-queries";

const SQL_STORAGE_KEY = "notebook-sql";

export default function Notebook() {
  const [source, setSource] = useState<string>("");
  const [sql, setSql] = useState(() => {
    const saved = localStorage.getItem(SQL_STORAGE_KEY);
    return saved ?? "";
  });
  const [savedQueries, setSavedQueries] = useState<SavedQueryItem[]>([]);

  const refreshSavedQueries = useCallback(async () => {
    try {
      const items = await invoke<SavedQueryItem[]>("list_saved_queries");
      setSavedQueries(items);
    } catch (error) {
      console.error("Failed to load saved queries:", error);
    }
  }, []);

  const handleDeleteSavedQuery = useCallback(
    async (id: number) => {
      try {
        await invoke("delete_saved_query", { id });
        await refreshSavedQueries();
      } catch (error) {
        console.error("Failed to delete saved query:", error);
      }
    },
    [refreshSavedQueries],
  );

  useEffect(() => {
    refreshSavedQueries();
  }, [refreshSavedQueries]);

  useEffect(() => {
    const timer = setTimeout(() => {
      localStorage.setItem(SQL_STORAGE_KEY, sql);
    }, 500);
    return () => clearTimeout(timer);
  }, [sql]);

  return (
    <div>
      <NotebookHeader />
      <div
        style={{
          display: "flex",
          flexWrap: "wrap",
          justifyContent: "center",
          alignItems: "top",
          height: "calc(100vh - 65px)",
          gap: "0px",
        }}
      >
        <NotebookLeft
          source={source}
          setSource={setSource}
          setSql={setSql}
          savedQueries={savedQueries}
          onDeleteSavedQuery={handleDeleteSavedQuery}
        />
        <NotebookMiddle
          sql={sql}
          setSql={setSql}
          onQuerySaved={refreshSavedQueries}
        />
        <NotebookRight />
      </div>
    </div>
  );
}
