import { useState } from "react";
import NotebookHeader from "./notebook-header/notebook-header";
import NotebookLeft from "./notebook-left/notebook-left";
import NotebookMiddle from "./notebook-middle/notebook-middle";
import NotebookRight from "./notebook-right/notebook-right";
export default function Notebook() {
  const [source, setSource] = useState<string>("");

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
        <NotebookLeft source={source} setSource={setSource} />
        <NotebookMiddle source={source} />
        <NotebookRight />
      </div>
    </div>
  );
}
