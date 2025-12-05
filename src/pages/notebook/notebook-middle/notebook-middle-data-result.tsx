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
import { memo, useState, useRef } from "react";
import { useVirtualizer } from "@tanstack/react-virtual";

interface DataResultProps {
  data: {
    header: string[];
    rows: string[][];
  };
  isLoading: boolean;
}

const ROW_HEIGHT = 36;

function DataResult({ data, isLoading }: DataResultProps) {
  const { isOpen, onOpen, onOpenChange } = useDisclosure();
  const [selectedRowIndex, setSelectedRowIndex] = useState<number | null>(null);
  const parentRef = useRef<HTMLDivElement>(null);

  const virtualizer = useVirtualizer({
    count: data.rows.length,
    getScrollElement: () => parentRef.current,
    estimateSize: () => ROW_HEIGHT,
    overscan: 5,
  });

  const handleRowDoubleClick = (index: number) => {
    setSelectedRowIndex(index);
    onOpen();
  };

  const selectedRowData = selectedRowIndex !== null ? data.rows[selectedRowIndex] : null;

  if (isLoading) {
    return (
      <div className="flex items-center justify-center w-full" style={{ height: "calc(40vh - 50px)" }}>
        <Spinner label="Loading..." />
      </div>
    );
  }

  if (data.rows.length === 0) {
    return (
      <div className="w-full">
        <div style={{ height: "calc(40vh - 50px)" }} className="overflow-auto">
          <table className="border-collapse" style={{ minWidth: '100%' }}>
            <thead className="bg-default-100">
              <tr className="border-b border-default-200 font-semibold text-sm text-default-600">
                <th className="px-3 py-2 text-left border-r border-default-200 w-14">#</th>
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
            <p className="font-medium text-default-500">No data available</p>
            <p className="text-sm">Run a query to see results here</p>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="w-full">
      <div
        ref={parentRef}
        className="overflow-auto"
        style={{ height: "calc(40vh - 50px)" }}
      >
        <table className="border-collapse" style={{ minWidth: '100%' }}>
          {/* 表头 */}
          <thead className="sticky top-0 z-20 bg-default-100">
            <tr className="border-b border-default-200 font-semibold text-sm text-default-600">
              <th className="px-3 py-2 text-center border-r border-default-200 sticky left-0 bg-default-100 z-30 whitespace-nowrap w-14">
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

          {/* 数据区域 - 使用真正的 tr/td */}
          <tbody>
            {(() => {
              const virtualItems = virtualizer.getVirtualItems();
              if (virtualItems.length === 0) return null;

              const firstItem = virtualItems[0];
              const lastItem = virtualItems[virtualItems.length - 1];
              const paddingTop = firstItem.start;
              const paddingBottom = virtualizer.getTotalSize() - lastItem.end;

              return (
                <>
                  {/* 虚拟化占位 - 上方 */}
                  {paddingTop > 0 && (
                    <tr style={{ height: paddingTop }}>
                      <td colSpan={data.header.length + 1} />
                    </tr>
                  )}

                  {/* 可见行 */}
                  {virtualItems.map((virtualRow) => {
                    const row = data.rows[virtualRow.index];
                    if (!row) return null; // 边界检查
                    const isEven = virtualRow.index % 2 === 0;

                    return (
                      <tr
                        key={virtualRow.index}
                        className={`text-sm cursor-pointer hover:bg-primary-50 ${
                          isEven ? "bg-default-50" : "bg-white"
                        }`}
                        style={{ height: ROW_HEIGHT }}
                        onDoubleClick={() => handleRowDoubleClick(virtualRow.index)}
                      >
                        <td
                          className={`px-3 py-2 text-center text-default-500 border-r border-default-100 sticky left-0 z-10 whitespace-nowrap ${
                            isEven ? "bg-default-50" : "bg-white"
                          }`}
                        >
                          {virtualRow.index + 1}
                        </td>
                        {row.map((cell, i) => (
                          <td
                            key={i}
                            className="px-3 py-2 text-left whitespace-nowrap border-r border-default-100 last:border-r-0"
                          >
                            {cell}
                          </td>
                        ))}
                      </tr>
                    );
                  })}

                  {/* 虚拟化占位 - 下方 */}
                  {paddingBottom > 0 && (
                    <tr style={{ height: paddingBottom }}>
                      <td colSpan={data.header.length + 1} />
                    </tr>
                  )}
                </>
              );
            })()}
          </tbody>
        </table>
      </div>

      {/* 行详情 Modal */}
      <Modal isOpen={isOpen} onOpenChange={onOpenChange} size="lg">
        <ModalContent>
          <ModalHeader>Row Details</ModalHeader>
          <ModalBody className="pb-6">
            {selectedRowData && (
              <div className="max-h-[60vh] overflow-y-auto">
                <table className="w-full border-collapse">
                  <tbody>
                    {data.header.map((header, index) => (
                      <tr key={index} className="border-b border-gray-200">
                        <th className="py-2 px-4 text-left bg-gray-50 font-medium w-1/3">
                          {header}
                        </th>
                        <td className="py-2 px-4">{selectedRowData[index] ?? ''}</td>
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
