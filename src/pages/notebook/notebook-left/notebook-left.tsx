import FilterList from "@/components/common/filter-list";
import {
  faChevronLeft,
  faDatabase,
  faServer,
  faTable,
} from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  Button,
  Listbox,
  ListboxItem,
  Modal,
  ModalBody,
  ModalContent,
  ModalHeader,
  useDisclosure,
} from "@heroui/react";
import { memo, useState } from "react";

interface NotebookLeftProps {
  source: string;
  setSource: (source: string) => void;
}

interface Catalog {
  id: number;
  table_ref: string;
  table_path: string;
  table_schema: [
    {
      field: string;
      field_type: string;
      comment?: string;
    }
  ];
}

function NotebookLeft({ source, setSource }: NotebookLeftProps) {
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const [tables, setTables] = useState<Catalog[]>([]);

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  async function refreshTables() {
    // const catalog: Catalog[] = await get("/api/catalog");
    // setTables(catalog);
  }

  const { isOpen, onOpen, onOpenChange } = useDisclosure();
  const [selectTable, setSelectTable] = useState<Catalog | null>(null);

  const handleTableClick = (item: string) => {
    setSelectTable(tables.find((table) => table.table_ref === item) ?? null);
    onOpen();
  };

  return (
    <div
      style={{
        width: "250px",
        textAlign: "center",
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
          aria-label="Like"
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
        </Button>{" "}
      </div>
      {source === "" ? (
        <div>
          <p
            style={{
              fontSize: "16px",
              fontWeight: "bold",
              textAlign: "left",
              paddingLeft: "15px",
              paddingTop: "10px",
              color: "gray",
            }}
          >
            Sources
          </p>
          <Listbox
            aria-label="Dynamic Actions"
            onAction={(value) => {
              setSource(value.toString());
            }}
            disabledKeys={["MySQL", "PostgreSQL"]}
            style={{
              width: "100%",
              textAlign: "left",
            }}
          >
            <ListboxItem
              key="EasyDB"
              startContent={<FontAwesomeIcon icon={faServer} />}
              onPress={() => {
                alert("暂未实现");
              }}
            >
              EasyDB
            </ListboxItem>
            <ListboxItem
              key="MySQL"
              startContent={<FontAwesomeIcon icon={faServer} />}
            >
              MySQL
            </ListboxItem>
            <ListboxItem
              key="PostgreSQL"
              startContent={<FontAwesomeIcon icon={faServer} />}
            >
              PostgreSQL
            </ListboxItem>
          </Listbox>
        </div>
      ) : (
        <div>
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
            <FontAwesomeIcon icon={faDatabase} style={{ marginRight: "5px" }} />
            {source}
          </p>
          <p
            style={{
              fontSize: "16px",
              textAlign: "left",
              paddingLeft: "10px",
              fontWeight: "bold",
              color: "gray", // Set text color to gray
            }}
          >
            Tables
          </p>
          <FilterList
            items={tables.map((table) => table.table_ref)}
            icon={<FontAwesomeIcon icon={faTable} />}
            onSelect={(item) => {
              handleTableClick(item);
            }}
          />
        </div>
      )}
      <Modal
        isOpen={isOpen}
        onOpenChange={onOpenChange}
        style={{
          maxWidth: "40vw",
          position: "absolute",
          top: "0",
          left: "340px",
          height: "500px",
        }}
      >
        <ModalContent>
          <ModalHeader>
            <div
              style={{
                display: "flex",
                justifyContent: "left",
                alignItems: "center",
                width: "100%",
              }}
            >
              <FontAwesomeIcon icon={faTable} style={{ marginRight: "8px" }} />{" "}
              {selectTable?.table_ref}
            </div>
          </ModalHeader>
          <ModalBody>
            {selectTable && (
              <div
                style={{
                  height: "100%",
                }}
              >
                <h3
                  style={{
                    fontSize: "16px",
                    fontWeight: "bold",
                    marginBottom: "10px",
                  }}
                >
                  <div style={{ color: "#6b7280" }}>
                    Location: {selectTable.table_path}
                  </div>
                  Columns ({selectTable.table_schema.length})
                </h3>
                <div
                  style={{
                    maxHeight: "350px",
                    overflowY: "auto",
                    marginTop: "20px",
                  }}
                >
                  <table className="w-full border-collapse border border-gray-200">
                    <thead style={{ position: "sticky", top: 0, zIndex: 1 }}>
                      <tr className="bg-gray-50">
                        <th className="py-2 px-4 text-left font-medium">
                          Name
                        </th>
                        <th className="py-2 px-4 text-left font-medium">
                          Type
                        </th>
                        <th className="py-2 px-4 text-left font-medium">
                          Comment
                        </th>
                      </tr>
                    </thead>
                    <tbody>
                      {selectTable.table_schema.map((column) => (
                        <tr
                          key={column.field}
                          className="border-t border-gray-200"
                        >
                          <td className="py-2 px-4">{column.field}</td>
                          <td className="py-2 px-4">{column.field_type}</td>
                          <td className="py-2 px-4">{column.comment}</td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              </div>
            )}
          </ModalBody>
        </ModalContent>
      </Modal>
    </div>
  );
}

export default memo(NotebookLeft);
