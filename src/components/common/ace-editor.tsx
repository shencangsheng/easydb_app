import AceEditor from "react-ace";
import "ace-builds/src-noconflict/theme-sqlserver";
import "ace-builds/src-noconflict/mode-sql";
import "ace-builds/src-noconflict/snippets/sql";
import { useEffect, useRef } from "react";
import langTools from "ace-builds/src-noconflict/ext-language_tools";
import { useTranslation } from "@/i18n";

interface AceEditorInstance {
  getSelectedText: () => string;
}

interface AceEditorProps {
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  fontSize?: number;
  height?: string;
  width?: string;
  showPrintMargin?: boolean;
  showGutter?: boolean;
  highlightActiveLine?: boolean;
  enableBasicAutocompletion?: boolean;
  enableLiveAutocompletion?: boolean;
  enableSnippets?: boolean;
  showLineNumbers?: boolean;
  tabSize?: number;
  onLoad?: (editor: AceEditorInstance) => void;
}

function CustomAceEditor({
  value,
  onChange,
  placeholder = "",
  fontSize = 16,
  height = "100%",
  width = "100%",
  showPrintMargin = true,
  showGutter = true,
  highlightActiveLine = true,
  enableBasicAutocompletion = true,
  enableLiveAutocompletion = true,
  enableSnippets = true,
  showLineNumbers = true,
  tabSize = 2,
  onLoad,
}: AceEditorProps) {
  const { translate } = useTranslation();
  const completerAdded = useRef(false);

  useEffect(() => {
    // 确保自动完成器只添加一次
    if (!completerAdded.current) {
      const customCompleter = {
        getCompletions: (
          editor: unknown,
          session: unknown,
          pos: unknown,
          prefix: string,
          callback: (error: unknown, results: unknown[]) => void
        ) => {
          const completions = [
            {
              caption: translate("functions.readCsv.name"),
              snippet: "read_csv(${1:path})",
              meta: translate("functions.readCsv.description"),
              value: "read_csv",
            },
            {
              caption: translate("functions.readTsv.name"),
              snippet: "read_tsv(${1:path})",
              meta: translate("functions.readTsv.description"),
              value: "read_tsv",
            },
            {
              caption: translate("functions.readJson.name"),
              snippet: "read_json(${1:path})",
              meta: translate("functions.readJson.description"),
              value: "read_json",
            },
            {
              caption: translate("functions.readNdjson.name"),
              snippet: "read_ndjson(${1:path})",
              meta: translate("functions.readNdjson.description"),
              value: "read_ndjson",
            },
            {
              caption: translate("functions.readExcel.name"),
              snippet: "read_excel(${1:path})",
              meta: translate("functions.readExcel.description"),
              value: "read_excel",
            },
            {
              caption: translate("functions.regexpLike.name"),
              snippet: "regexp_like(${1:column}, ${2:pattern})",
              meta: translate("functions.regexpLike.description"),
              value: "regexp_like",
            },
          ];

          callback(null, completions);
        },
      };

      langTools.addCompleter(customCompleter);
      completerAdded.current = true;
    }
  }, [translate]);

  return (
    <AceEditor
      mode="sql"
      theme="sqlserver"
      name="editor"
      width={width}
      height={height}
      fontSize={fontSize}
      showPrintMargin={showPrintMargin}
      showGutter={showGutter}
      highlightActiveLine={highlightActiveLine}
      placeholder={placeholder}
      value={value}
      onChange={onChange}
      onLoad={onLoad}
      setOptions={{
        enableBasicAutocompletion,
        enableLiveAutocompletion,
        enableSnippets,
        showLineNumbers,
        tabSize,
      }}
      editorProps={{ $blockScrolling: true }}
    />
  );
}

export default CustomAceEditor;
