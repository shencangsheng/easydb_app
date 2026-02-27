import { faTable } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  Modal,
  ModalBody,
  ModalContent,
  ModalHeader,
  Spinner,
  useDisclosure,
} from "@heroui/react";
import { memo, useState, useRef, useMemo, useCallback, useEffect } from "react";
import { useVirtualizer } from "@tanstack/react-virtual";
import {
  useReactTable,
  getCoreRowModel,
  flexRender,
  ColumnDef,
} from "@tanstack/react-table";

interface DataResultProps {
  data: {
    header: string[];
    rows: string[][];
  };
  isLoading: boolean;
}

const ROW_HEIGHT = 36;
const DEFAULT_COLUMN_WIDTH = 150;

const MODAL_MIN_WIDTH = 400;
const MODAL_MIN_HEIGHT = 300;
const MODAL_DEFAULT_WIDTH = 512;
const MODAL_DEFAULT_HEIGHT = 400;

function DataResult({ data, isLoading }: DataResultProps) {
  const { isOpen, onOpen, onOpenChange } = useDisclosure();
  const [selectedRowIndex, setSelectedRowIndex] = useState<number | null>(null);
  const [modalSize, setModalSize] = useState({
    width: MODAL_DEFAULT_WIDTH,
    height: MODAL_DEFAULT_HEIGHT,
  });
  const [lastColumnExtraWidth, setLastColumnExtraWidth] = useState(0);
  const parentRef = useRef<HTMLDivElement>(null);

  // Dynamically generate column definitions
  const columns = useMemo<ColumnDef<string[]>[]>(() => {
    // Row number column
    const indexColumn: ColumnDef<string[]> = {
      id: "_index",
      header: "#",
      size: 56,
      minSize: 56,
      maxSize: 56,
      enableResizing: false,
      cell: ({ row }) => row.index + 1,
    };

    // Data columns
    const isSingleColumn = data.header.length === 1;

    const dataColumns: ColumnDef<string[]>[] = data.header.map(
      (header, index) => ({
        id: `col_${index}`,
        accessorFn: (row: string[]) => row[index],
        header: header,
        size: isSingleColumn ? 9999 : DEFAULT_COLUMN_WIDTH,
        minSize: 50,
      })
    );

    return [indexColumn, ...dataColumns];
  }, [data.header]);

  // Table instance
  const table = useReactTable({
    data: data.rows,
    columns,
    columnResizeMode: "onChange",
    enableColumnResizing: true,
    getCoreRowModel: getCoreRowModel(),
  });

  // 当列宽总和小于容器宽度时，将最后一列扩展以填满表格
  const tableRef = useRef(table);
  tableRef.current = table;
  useEffect(() => {
    const el = parentRef.current;
    if (!el || data.header.length === 0) return;

    const updateLastColumnExtraWidth = () => {
      const containerWidth = el.clientWidth;
      const totalColumnWidth = tableRef.current.getTotalSize();
      if (totalColumnWidth < containerWidth) {
        setLastColumnExtraWidth(containerWidth - totalColumnWidth);
      } else {
        setLastColumnExtraWidth(0);
      }
    };

    updateLastColumnExtraWidth();
    const observer = new ResizeObserver(updateLastColumnExtraWidth);
    observer.observe(el);
    return () => observer.disconnect();
  }, [data.header.length, data.rows.length]);

  // Virtualization
  const virtualizer = useVirtualizer({
    count: data.rows.length,
    getScrollElement: () => parentRef.current,
    estimateSize: () => ROW_HEIGHT,
    overscan: 5,
  });

  // Reset selected state when data changes to prevent showing old data
  useEffect(() => {
    setSelectedRowIndex(null);
  }, [data.rows]);

  // Use useCallback to avoid unnecessary re-renders
  const handleRowDoubleClick = useCallback(
    (index: number) => {
      setSelectedRowIndex(index);
      onOpen();
    },
    [onOpen]
  );

  // Handle column width adjustment, prevent selection effect
  const handleResize = useCallback(
    (
      e: React.MouseEvent | React.TouchEvent,
      handler: (e: MouseEvent | TouchEvent) => void
    ) => {
      e.preventDefault();
      e.stopPropagation();
      handler(e.nativeEvent);
    },
    []
  );

  // Modal resize: add listeners on mousedown, remove on mouseup
  const handleModalResizeStart = useCallback(
    (e: React.MouseEvent) => {
      e.preventDefault();
      e.stopPropagation();
      const startX = e.clientX;
      const startY = e.clientY;
      const startW = modalSize.width;
      const startH = modalSize.height;

      const onMouseMove = (ev: MouseEvent) => {
        const dx = ev.clientX - startX;
        const dy = ev.clientY - startY;
        setModalSize({
          width: Math.max(MODAL_MIN_WIDTH, startW + dx),
          height: Math.max(MODAL_MIN_HEIGHT, startH + dy),
        });
      };

      const onMouseUp = () => {
        window.removeEventListener("mousemove", onMouseMove);
        window.removeEventListener("mouseup", onMouseUp);
      };

      window.addEventListener("mousemove", onMouseMove);
      window.addEventListener("mouseup", onMouseUp);
    },
    [modalSize]
  );

  // Safely get selected row data to prevent index out of bounds
  const selectedRowData =
    selectedRowIndex !== null && selectedRowIndex < data.rows.length
      ? data.rows[selectedRowIndex]
      : null;

  if (isLoading) {
    return (
      <div
        className="flex items-center justify-center w-full"
        style={{ height: "calc(40vh - 50px)" }}
      >
        <Spinner label="Loading..." />
      </div>
    );
  }

  if (data.rows.length === 0) {
    return (
      <div className="w-full">
        <div style={{ height: "calc(40vh - 50px)" }} className="overflow-auto">
          <table className="border-collapse" style={{ minWidth: "100%" }}>
            <thead className="bg-default-100">
              <tr className="border-b border-default-200 font-semibold text-sm text-default-600">
                <th className="px-3 py-2 text-left border-r border-default-200 w-14">
                  #
                </th>
                {data.header.map((col, i) => (
                  <th
                    key={i}
                    className="px-3 py-2 text-left font-semibold whitespace-nowrap border-r border-default-200 last:border-r-0"
                    style={{ minWidth: 100 }}
                  >
                    {col}
                  </th>
                ))}
              </tr>
            </thead>
          </table>
          <div className="flex flex-col items-center justify-center gap-2 text-default-400 pt-16">
            <FontAwesomeIcon icon={faTable} size="2x" />
            {data.header.length > 0 ? (
              <p className="font-medium text-default-500">No records found</p>
            ) : (
              <>
                <p className="font-medium text-default-500">
                  No data available
                </p>
                <p className="text-sm">Run a query to see results here</p>
              </>
            )}
          </div>
        </div>
      </div>
    );
  }

  const virtualItems = virtualizer.getVirtualItems();
  const totalSize = virtualizer.getTotalSize();
  const { rows: tableRows } = table.getRowModel();
  const baseTableWidth = table.getTotalSize();
  const tableWidth = baseTableWidth + lastColumnExtraWidth;
  const isSingleColumn = data.header.length === 1;

  return (
    <div className="w-full">
      <div
        ref={parentRef}
        className="overflow-auto"
        style={{ height: "calc(40vh - 50px)" }}
      >
        <div style={{ width: tableWidth, minWidth: "100%" }}>
          {/* Fixed table header */}
          <div
            className="sticky top-0 z-20 bg-default-100 select-none"
            style={{ width: tableWidth }}
          >
            {table.getHeaderGroups().map((headerGroup) => (
              <div
                key={headerGroup.id}
                className="flex border-b border-default-200"
              >
                {headerGroup.headers.map((header) => {
                  const isLastColumn =
                    header.id ===
                    headerGroup.headers[headerGroup.headers.length - 1].id;
                  const baseWidth = header.getSize();
                  const width =
                    isLastColumn && header.id !== "_index"
                      ? baseWidth + lastColumnExtraWidth
                      : baseWidth;
                  return (
                  <div
                    key={header.id}
                    className={`relative px-3 py-2 font-semibold text-sm text-default-600 border-r border-default-200 last:border-r-0 whitespace-nowrap overflow-hidden text-ellipsis ${
                      header.id === "_index"
                        ? "text-center sticky left-0 bg-default-100 z-30"
                        : "text-left"
                    }`}
                    style={
                      isSingleColumn && header.id !== "_index"
                        ? { flex: 1 }
                        : { width }
                    }
                  >
                    {flexRender(
                      header.column.columnDef.header,
                      header.getContext()
                    )}
                    {/* Drag handle for column width adjustment */}
                    {header.column.getCanResize() && (
                      <div
                        onMouseDown={(e) =>
                          handleResize(e, header.getResizeHandler())
                        }
                        onTouchStart={(e) =>
                          handleResize(e, header.getResizeHandler())
                        }
                        className={`absolute right-0 top-0 h-full w-1 cursor-col-resize select-none touch-none hover:bg-primary-400 ${
                          header.column.getIsResizing() ? "bg-primary-500" : ""
                        }`}
                        style={{ userSelect: "none" }}
                      />
                    )}
                  </div>
                  );
                })}
              </div>
            ))}
          </div>

          {/* Virtualized data row container */}
          <div style={{ height: totalSize, position: "relative" }}>
            {virtualItems.map((virtualRow) => {
              const row = tableRows[virtualRow.index];
              if (!row) return null;
              const isEven = virtualRow.index % 2 === 0;

              return (
                <div
                  key={virtualRow.index}
                  className={`absolute left-0 flex text-sm cursor-pointer hover:bg-primary-50 ${
                    isEven ? "bg-default-50" : "bg-white"
                  }`}
                  style={{
                    height: ROW_HEIGHT,
                    width: tableWidth,
                    transform: `translateY(${virtualRow.start}px)`,
                  }}
                  onDoubleClick={() => handleRowDoubleClick(virtualRow.index)}
                >
                  {row.getVisibleCells().map((cell) => {
                    const isLastCell =
                      cell.column.id ===
                      row.getVisibleCells()[row.getVisibleCells().length - 1]
                        .column.id;
                    const baseSize = cell.column.getSize();
                    const cellWidth =
                      isLastCell && cell.column.id !== "_index"
                        ? baseSize + lastColumnExtraWidth
                        : baseSize;
                    return (
                    <div
                      key={cell.id}
                      className={`px-3 py-2 border-r border-default-100 last:border-r-0 whitespace-nowrap overflow-hidden text-ellipsis ${
                        cell.column.id === "_index"
                          ? `text-center text-default-500 sticky left-0 z-10 ${
                              isEven ? "bg-default-50" : "bg-white"
                            }`
                          : "text-left"
                      }`}
                      style={
                        isSingleColumn && cell.column.id !== "_index"
                          ? { flex: 1 }
                          : { width: cellWidth }
                      }
                    >
                      {flexRender(
                        cell.column.columnDef.cell,
                        cell.getContext()
                      )}
                    </div>
                  );
                  })}
                </div>
              );
            })}
          </div>
        </div>
      </div>

      {/* Row details Modal */}
      <Modal
        isOpen={isOpen}
        onOpenChange={onOpenChange}
        size="lg"
        classNames={{ base: "max-w-[95vw]" }}
      >
        <ModalContent
          className="relative overflow-visible"
          style={{
            width: modalSize.width,
            height: modalSize.height,
            minWidth: MODAL_MIN_WIDTH,
            minHeight: MODAL_MIN_HEIGHT,
          }}
        >
          <ModalHeader>Row Details</ModalHeader>
          <ModalBody className="pb-6 flex-1 overflow-hidden">
            {selectedRowData && (
              <div
                className="overflow-y-auto"
                style={{ maxHeight: modalSize.height - 120 }}
              >
                <table className="w-full border-collapse">
                  <tbody>
                    {data.header.map((header, index) => (
                      <tr key={index} className="border-b border-gray-200">
                        <th className="py-2 px-4 text-left bg-gray-50 font-medium w-1/3">
                          {header}
                        </th>
                        <td className="py-2 px-4">
                          {selectedRowData[index] ?? ""}
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            )}
          </ModalBody>
          {/* Resize handle */}
          <div
            onMouseDown={handleModalResizeStart}
            className="absolute bottom-0 right-0 w-5 h-5 cursor-se-resize select-none touch-none z-[100] pointer-events-auto hover:opacity-80"
            style={{
              background:
                "linear-gradient(135deg, transparent 50%, rgba(0,0,0,0.2) 50%)",
            }}
            aria-label="Resize modal"
          />
        </ModalContent>
      </Modal>
    </div>
  );
}

export default memo(DataResult);
