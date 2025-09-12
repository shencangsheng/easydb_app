import { StrictMode, useState } from "react";
import { createRoot } from "react-dom/client";
import { HeroUIProvider } from "@heroui/react";
import { call } from "./commands";

function App() {
  const [input, setInput] = useState("");
  const [result, setResult] = useState<unknown>(null);
  const [loading, setLoading] = useState(false);

  const handleConfirm = async () => {
    setLoading(true);
    try {
      const res = await call(input);
      setResult(res);
    } catch (e) {
      setResult("发生错误: " + (e as unknown)?.toString());
    } finally {
      setLoading(false);
    }
  };

  return (
    <main className="light text-foreground bg-background">
      <div
        style={{
          display: "flex",
          gap: "8px",
          alignItems: "center",
          padding: "16px",
        }}
      >
        <textarea
          placeholder="请输入内容"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          style={{
            padding: "12px",
            border: "1px solid #ccc",
            borderRadius: "4px",
            flex: "1",
            minHeight: "120px",
            resize: "vertical",
            fontSize: "18px",
            fontFamily: "monospace",
          }}
          autoCapitalize="none" // 禁用自动首字母大写
          autoCorrect="off" // 禁用自动校正
          spellCheck="false" // 禁用拼写检查
        />
        <button
          onClick={handleConfirm}
          disabled={loading}
          style={{
            padding: "8px 16px",
            background: "#6366f1",
            color: "#fff",
            border: "none",
            borderRadius: "4px",
            cursor: loading ? "not-allowed" : "pointer",
            opacity: loading ? 0.7 : 1,
          }}
        >
          {loading ? "处理中..." : "确认"}
        </button>
      </div>
      <div style={{ padding: "16px" }}>
        {result !== null && (
          <pre
            style={{
              background: "#f4f4f4",
              padding: "12px",
              borderRadius: "4px",
              whiteSpace: "pre-wrap",
              wordBreak: "break-all",
              marginTop: "8px",
            }}
          >
            {typeof result === "object"
              ? JSON.stringify(result, null, 2)
              : String(result)}
          </pre>
        )}
      </div>
    </main>
  );
}

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <HeroUIProvider>
      <App />
    </HeroUIProvider>
  </StrictMode>
);
