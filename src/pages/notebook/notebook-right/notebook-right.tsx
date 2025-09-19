import { memo } from "react";

function NotebookRight() {
  return (
    <div
      style={{
        width: "120px",
        textAlign: "center",
      }}
    >
      <div
        style={{
          height: 60,
          borderBottom: "1px solid rgba(17, 17, 17, 0.15)",
        }}
      ></div>
    </div>
  );
}

export default memo(NotebookRight);
