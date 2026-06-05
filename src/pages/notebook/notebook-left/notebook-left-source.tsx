import { useTranslation } from "@/i18n";
import {
  faBookmark,
  faChevronLeft,
  faDatabase,
} from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { Button, Listbox, ListboxItem } from "@heroui/react";
import { memo } from "react";
import NotebookLeftSavedQueries, {
  SavedQueryItem,
  SAVED_QUERIES_SOURCE,
} from "./notebook-left-saved-queries";

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
  const { translate } = useTranslation();

  return (
    <div
      style={{
        width: "250px",
        textAlign: "center",
        height: "100%",
        display: "flex",
        flexDirection: "column",
        minHeight: 0,
      }}
    >
      <div
        style={{
          height: 60,
          borderBottom: "1px solid rgba(17, 17, 17, 0.15)",
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
          paddingLeft: "15px",
          paddingRight: "15px",
        }}
      >
        <Button
          isIconOnly
          aria-label="Database"
          style={{
            backgroundColor: "transparent",
            fontSize: "21px",
            padding: "10px",
          }}
        >
          <FontAwesomeIcon
            icon={faDatabase}
            size="lg"
            color={"#000000"}
            className="hover:text-black"
          />
        </Button>
      </div>
      <div
        style={{
          flex: 1,
          minHeight: 0,
          display: "flex",
          flexDirection: "column",
        }}
      >
        {source === "" ? (
          <div>
            <p
              style={{
                fontSize: "15px",
                fontWeight: "bold",
                textAlign: "left",
                paddingLeft: "15px",
                paddingTop: "12px",
                paddingBottom: "8px",
                color: "gray",
              }}
            >
              {translate("notebook.sidebar.title")}
            </p>
            <Listbox
              aria-label={translate("notebook.sidebar.title")}
              onAction={(value) => {
                setSource(value.toString());
              }}
              style={{
                width: "100%",
                textAlign: "left",
              }}
              itemClasses={{
                base: "min-h-[48px] px-3 py-2 rounded-lg data-[hover=true]:bg-default-100",
                title: "text-[16px] font-medium",
              }}
            >
              <ListboxItem
                key={SAVED_QUERIES_SOURCE}
                startContent={
                  <FontAwesomeIcon
                    icon={faBookmark}
                    style={{ fontSize: "20px", width: "24px" }}
                  />
                }
              >
                {translate("notebook.savedQueries.title")}
              </ListboxItem>
            </Listbox>
          </div>
        ) : (
          <div
            style={{
              display: "flex",
              flexDirection: "column",
              flex: 1,
              minHeight: 0,
            }}
          >
            <p
              style={{
                fontSize: "16px",
                textAlign: "left",
                paddingLeft: "1px",
                paddingTop: "3px",
                display: "flex",
                alignItems: "center",
              }}
            >
              <Button
                isIconOnly
                onPress={() => {
                  setSource("");
                }}
                style={{
                  backgroundColor: "transparent",
                  fontSize: "16px",
                }}
              >
                <FontAwesomeIcon icon={faChevronLeft} />
              </Button>
              <FontAwesomeIcon icon={faBookmark} style={{ marginRight: "5px" }} />
              {translate("notebook.savedQueries.title")}
            </p>
            <NotebookLeftSavedQueries
              items={savedQueries}
              setSql={setSql}
              onDelete={onDeleteSavedQuery}
            />
          </div>
        )}
      </div>
    </div>
  );
}

export default memo(NotebookLeft);
