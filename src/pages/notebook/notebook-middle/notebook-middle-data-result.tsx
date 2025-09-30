import { faTable } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  Modal,
  ModalBody,
  ModalContent,
  ModalHeader,
  Spinner,
  Table,
  TableBody,
  TableCell,
  TableColumn,
  TableHeader,
  TableRow,
  useDisclosure,
} from "@heroui/react";
import { memo, useState, useMemo, useCallback } from "react";

type TableRow = {
  id: string;
  values: string[];
  index: number;
};

interface DataResultProps {
  data: {
    header: string[];
    rows: string[][];
  };
  isLoading: boolean;
}

// 缓存样式对象，避免重复创建
const ROW_NUMBER_COLUMN_STYLE = {
  backgroundColor: "white",
  borderBottom: "1px solid rgba(17, 17, 17, 0.15)",
  width: "40px",
};

const HEADER_COLUMN_STYLE = {
  backgroundColor: "white",
  borderBottom: "1px solid rgba(17, 17, 17, 0.15)",
  fontWeight: "bold",
  fontSize: "0.9em",
};

const STICKY_CELL_STYLE = {
  position: "sticky" as const,
  left: 0,
  backgroundColor: "#f5f5f5",
  zIndex: 1,
};

const EVEN_ROW_STYLE = { backgroundColor: "#f5f5f5" };
const ODD_ROW_STYLE = { backgroundColor: "#ffffff" };
const CELL_BORDER_STYLE = { borderBottom: "1px solid rgba(17, 17, 17, 0.15)" };

const transformTableData = (rows: string[][]): TableRow[] => {
  return rows.map((row, index) => ({
    id: `row-${index}`,
    values: [`${index + 1}`, ...row],
    index: index,
  }));
};

// 使用 useMemo 缓存表头，避免重复渲染
const getHeaderColumns = (header: string[]) => {
  const headers = [
    <TableColumn key="row-number" style={ROW_NUMBER_COLUMN_STYLE}>
      #
    </TableColumn>,
  ];

  header?.forEach((column) => {
    headers.push(
      <TableColumn key={column} style={HEADER_COLUMN_STYLE}>
        {column}
      </TableColumn>
    );
  });

  return headers;
};

function DataResult({ data, isLoading }: DataResultProps) {
  const { isOpen, onOpen, onOpenChange } = useDisclosure();
  const [selectedItem, setSelectedItem] = useState<TableRow | null>(null);

  // 使用 useMemo 缓存转换后的数据，避免每次渲染都重新计算
  const tableData = useMemo(() => transformTableData(data.rows), [data.rows]);

  // 使用 useMemo 缓存表头，避免重复渲染
  const headerColumns = useMemo(
    () => getHeaderColumns(data.header),
    [data.header]
  );

  // 使用 useCallback 缓存事件处理函数
  const handleRowDoubleClick = useCallback(
    (item: TableRow) => {
      setSelectedItem(item);
      onOpen();
    },
    [onOpen]
  );

  // 优化单元格渲染函数
  const renderCell = useCallback(
    (value: string, index: number, item: TableRow) => {
      const isRowNumber = index === 0;
      const cellStyle = {
        whiteSpace: "nowrap" as const,
        ...(isRowNumber
          ? STICKY_CELL_STYLE
          : item.index % 2 === 0
          ? EVEN_ROW_STYLE
          : ODD_ROW_STYLE),
        ...CELL_BORDER_STYLE,
      };

      return (
        <TableCell key={`${item.id}-${index}`} style={cellStyle}>
          {value}
        </TableCell>
      );
    },
    []
  );

  return (
    <div className="w-full">
      <Table
        isVirtualized={true}
        maxTableHeight={500}
        rowHeight={40}
        radius={"none"}
        classNames={{
          wrapper: "shadow-none",
        }}
      >
        <TableHeader>{headerColumns}</TableHeader>
        <TableBody
          items={tableData}
          isLoading={isLoading}
          loadingContent={<Spinner label="Loading..." />}
          emptyContent={
            <div
              style={{
                padding: "20px",
                textAlign: "center",
                color: "#666",
                display: "flex",
                flexDirection: "column",
                alignItems: "center",
                gap: "10px",
              }}
            >
              <FontAwesomeIcon icon={faTable} size="2x" />
              <p>No data available</p>
              <p style={{ fontSize: "14px" }}>
                Run a query to see results here
              </p>
            </div>
          }
        >
          {(item: TableRow) => (
            <TableRow
              key={item.id}
              onDoubleClick={() => handleRowDoubleClick(item)}
            >
              {item.values.map((value, index) =>
                renderCell(value, index, item)
              )}
            </TableRow>
          )}
        </TableBody>
      </Table>

      <Modal
        isOpen={isOpen}
        onOpenChange={onOpenChange}
        style={{ maxWidth: "40vw" }}
      >
        <ModalContent>
          <ModalHeader>Row details</ModalHeader>
          <ModalBody>
            {selectedItem && (
              <div
                style={{
                  maxHeight: "80vh",
                  overflowY: "auto",
                }}
              >
                <table className="w-full border-collapse border border-gray-200">
                  <tbody>
                    {selectedItem.values
                      .filter((_value, index) => index !== 0)
                      .map((value, index) => (
                        <tr key={index} className="border-b border-gray-200">
                          <th className="py-2 px-4 text-left bg-gray-50 font-medium">
                            {data.header[index]}
                          </th>
                          <td className="py-2 px-4">{value}</td>
                        </tr>
                      ))}
                  </tbody>
                </table>
              </div>
            )}
          </ModalBody>
        </ModalContent>
      </Modal>
    </div>
  );
}

export default memo(DataResult);
