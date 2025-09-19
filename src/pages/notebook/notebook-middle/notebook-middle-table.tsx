import {
  faCode,
  faDownload,
  faFileCsv,
} from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  Button,
  Dropdown,
  DropdownItem,
  DropdownMenu,
  DropdownTrigger,
} from "@heroui/react";
import { memo } from "react";
import DataResult from "./notebook-middle-data-result";

interface TableProps {
  data: {
    header: string[];
    rows: string[][];
  };
  isLoading: boolean;
  sql: string;
}

function DataTable({ data, isLoading, sql }: TableProps) {
  async function exportResults(fileType: string) {
    window.alert("暂未实现");
  }

  return (
    <div
      style={{
        display: "flex",
        height: "calc(40vh - 50px)",
        width: "100%",
        overflow: "hidden",
        // border: "1px solid rgba(17, 17, 17, 0.15)",
      }}
    >
      <div
        style={{
          justifyContent: "flex-end",
          paddingTop: "15px",
        }}
      >
        <Dropdown placement="bottom-start" isDisabled={data.rows.length === 0}>
          <DropdownTrigger>
            <Button variant="light" isIconOnly>
              <FontAwesomeIcon icon={faDownload} />
            </Button>
          </DropdownTrigger>
          <DropdownMenu aria-label="Static Actions">
            <DropdownItem
              key="csv"
              onPress={async () => await exportResults("CSV")}
            >
              <FontAwesomeIcon
                icon={faFileCsv}
                style={{ marginRight: "5px" }}
              />
              CSV
            </DropdownItem>
            <DropdownItem
              key="json"
              onPress={async () => await exportResults("JSON")}
            >
              <FontAwesomeIcon icon={faCode} style={{ marginRight: "5px" }} />
              NdJson
            </DropdownItem>
            <DropdownItem
              key="tsv"
              onPress={async () => await exportResults("TSV")}
            >
              <FontAwesomeIcon
                icon={faFileCsv}
                style={{ marginRight: "5px" }}
              />
              TSV
            </DropdownItem>
          </DropdownMenu>
        </Dropdown>
      </div>

      <DataResult data={data} isLoading={isLoading} />
    </div>
  );
}

export default memo(DataTable);
