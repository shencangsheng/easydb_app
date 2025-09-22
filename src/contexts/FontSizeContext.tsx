import React, { createContext, useContext, useState, useEffect } from "react";

interface FontSizeContextType {
  fontSize: number;
  setFontSize: (size: number) => void;
}

const FontSizeContext = createContext<FontSizeContextType | undefined>(
  undefined
);

export const useFontSize = () => {
  const context = useContext(FontSizeContext);
  if (!context) {
    throw new Error("useFontSize must be used within a FontSizeProvider");
  }
  return context;
};

interface FontSizeProviderProps {
  children: React.ReactNode;
}

export const FontSizeProvider: React.FC<FontSizeProviderProps> = ({
  children,
}) => {
  const [fontSize, setFontSizeState] = useState<number>(16);

  // 从localStorage加载保存的字体大小
  useEffect(() => {
    const savedFontSize = localStorage.getItem("app-font-size");
    if (savedFontSize) {
      const size = parseInt(savedFontSize, 10);
      if (!isNaN(size) && size >= 10 && size <= 24) {
        setFontSizeState(size);
      }
    }
  }, []);

  // 应用字体大小到document
  useEffect(() => {
    document.documentElement.style.fontSize = `${fontSize}px`;
  }, [fontSize]);

  // 保存字体大小到localStorage
  const handleSetFontSize = (size: number) => {
    setFontSizeState(size);
    localStorage.setItem("app-font-size", size.toString());
  };

  return (
    <FontSizeContext.Provider
      value={{ fontSize, setFontSize: handleSetFontSize }}
    >
      {children}
    </FontSizeContext.Provider>
  );
};
