import React, { useState, useEffect } from "react";
import { FontSizeContextType, FontSizeContext } from "./FontSizeContextTypes";

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

  const contextValue: FontSizeContextType = {
    fontSize,
    setFontSize: handleSetFontSize,
  };

  return (
    <FontSizeContext.Provider value={contextValue}>
      {children}
    </FontSizeContext.Provider>
  );
};
