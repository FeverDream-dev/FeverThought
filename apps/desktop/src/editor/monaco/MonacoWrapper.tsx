import Editor from "@monaco-editor/react";
import { useRef } from "react";
import { useAppStore } from "../../stores/appStore";

interface MonacoWrapperProps {
  path: string;
  language: string;
  value: string;
  onChange: (value: string | undefined) => void;
}

const FRUTIGER_THEME = {
  base: "vs" as const,
  inherit: true,
  rules: [
    { token: "comment", foreground: "6a9955", fontStyle: "italic" },
    { token: "keyword", foreground: "0078d4" },
    { token: "string", foreground: "a31515" },
    { token: "number", foreground: "098658" },
    { token: "type", foreground: "267f99" },
    { token: "function", foreground: "795e26" },
    { token: "variable", foreground: "001080" },
    { token: "operator", foreground: "000000" },
  ],
  colors: {
    "editor.background": "#ffffff",
    "editor.foreground": "#1a1a2e",
    "editor.lineHighlightBackground": "#f0f4f8",
    "editor.selectionBackground": "#add6ff80",
    "editorLineNumber.foreground": "#237893",
    "editorLineNumber.activeForeground": "#0078d4",
    "editor.inactiveSelectionBackground": "#add6ff30",
    "editorCursor.foreground": "#0078d4",
    "editorWhitespace.foreground": "#d3d3d3",
    "editorIndentGuide.background": "#d3d3d360",
    "editorIndentGuide.activeBackground": "#0078d440",
  },
};

export function MonacoWrapper({ path, language, value, onChange }: MonacoWrapperProps) {
  const editorRef = useRef<any>(null);

  const handleMount = (editor: any, monaco: any) => {
    editorRef.current = editor;
    monaco.editor.defineTheme("frutiger", FRUTIGER_THEME as any);
    monaco.editor.setTheme("frutiger");
    editor.focus();
  };

  return (
    <Editor
      height="100%"
      language={language}
      value={value}
      onChange={onChange}
      onMount={handleMount}
      path={path}
      theme="frutiger"
      options={{
        fontSize: 14,
        fontFamily: "'Fira Code', 'Cascadia Code', monospace",
        fontLigatures: true,
        minimap: { enabled: true },
        lineNumbers: "on",
        renderLineHighlight: "all",
        bracketPairColorization: { enabled: true },
        autoClosingBrackets: "always",
        autoClosingQuotes: "always",
        formatOnPaste: true,
        scrollBeyondLastLine: false,
        smoothScrolling: true,
        cursorBlinking: "smooth",
        cursorSmoothCaretAnimation: "on",
        padding: { top: 8 },
        suggest: {
          showMethods: true,
          showFunctions: true,
          showConstants: true,
          showProperties: true,
        },
      }}
    />
  );
}
