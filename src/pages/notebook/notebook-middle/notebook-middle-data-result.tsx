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

  // 虚拟化：只计算可见行，不创建任何中间数据结构
  const virtualizer = useVirtualizer({
    count: data.rows.length,
    getScrollElement: () => parentRef.current,
    estimateSize: () => ROW_HEIGHT,
    overscan: 5, // 预渲染额外 5 行
  });

  const handleRowDoubleClick = (index: number) => {
    setSelectedRowIndex(index);
    onOpen();
  };

  const selectedRowData = selectedRowIndex !== null ? data.rows[selectedRowIndex] : null;

  // Loading 状态
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

  // 空数据状态：显示表头和空状态提示
  if (data.rows.length === 0) {
    return (
      <div className="w-full">
        <div style={{ height: "calc(40vh - 50px)" }}>
          {/* 表头 */}
          <div className="flex bg-default-100 border-b border-default-200 font-semibold text-base text-default-600">
            <div 
              className="flex-shrink-0 px-3 py-2 text-center border-r border-default-200"
              style={{ width: 56 }}
            >
              #
            </div>
          </div>
          {/* 空状态提示 */}
          <div className="flex flex-col items-center justify-center gap-2 text-default-400 pt-16">
            <FontAwesomeIcon icon={faTable} size="2x" />
            <p className="font-medium text-default-500">No data available</p>
            <p className="text-sm">Run a query to see results here</p>
          </div>
        </div>
      </div>
    );
  }

  // 计算表格总宽度：行号列 + 所有数据列
  const columnWidth = 150;
  const indexColumnWidth = 56;
  const tableWidth = indexColumnWidth + data.header.length * columnWidth;

  return (
    <div className="w-full">
      {/* 统一滚动容器 */}
      <div
        ref={parentRef}
        className="overflow-auto"
        style={{ height: "calc(40vh - 50px)" }}
      >
        {/* 表格内容区域 - 固定宽度 */}
        <div style={{ width: tableWidth, minWidth: "100%" }}>
          {/* 固定表头 */}
          <div className="flex bg-default-100 border-b border-default-200 font-semibold text-sm text-default-600 sticky top-0 z-20">
            {/* 行号列 */}
            <div 
              className="flex-shrink-0 px-3 py-2 text-center border-r border-default-200 sticky left-0 bg-default-100 z-30"
              style={{ width: indexColumnWidth }}
            >
              #
            </div>
            {data.header.map((col, i) => (
              <div
                key={i}
                className="px-3 py-2 truncate border-r border-default-200 last:border-r-0"
                style={{ width: columnWidth }}
              >
                {col}
              </div>
            ))}
          </div>

          {/* 虚拟化行容器 */}
          <div
            style={{
              height: virtualizer.getTotalSize(),
              position: "relative",
            }}
          >
            {virtualizer.getVirtualItems().map((virtualRow) => {
              const row = data.rows[virtualRow.index];
              const isEven = virtualRow.index % 2 === 0;

              return (
                <div
                  key={virtualRow.index}
                  className={`flex text-sm cursor-pointer hover:bg-primary-50 ${
                    isEven ? "bg-default-50" : "bg-white"
                  }`}
                  style={{
                    position: "absolute",
                    top: 0,
                    left: 0,
                    width: "100%",
                    height: virtualRow.size,
                    transform: `translateY(${virtualRow.start}px)`,
                  }}
                  onDoubleClick={() => handleRowDoubleClick(virtualRow.index)}
                >
                  {/* 行号列 - sticky */}
                  <div
                    className={`flex-shrink-0 px-3 py-2 text-center text-default-500 border-r border-default-100 sticky left-0 z-10 ${
                      isEven ? "bg-default-50" : "bg-white"
                    }`}
                    style={{ width: indexColumnWidth }}
                  >
                    {virtualRow.index + 1}
                  </div>
                  {row.map((cell, i) => (
                    <div
                      key={i}
                      className="px-3 py-2 truncate border-r border-default-100 last:border-r-0"
                      style={{ width: columnWidth }}
                    >
                      {cell}
                    </div>
                  ))}
                </div>
              );
            })}
          </div>
        </div>
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
                        <td className="py-2 px-4">{selectedRowData[index]}</td>
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
