import React, { useState, useEffect } from "react";
import { Theme, ThemeContextType, ThemeContext } from "./ThemeContextTypes";

interface ThemeProviderProps {
  children: React.ReactNode;
}

export const ThemeProvider: React.FC<ThemeProviderProps> = ({ children }) => {
  const [theme, setTheme] = useState<Theme>("system");

  // 从localStorage加载保存的主题
  useEffect(() => {
    const savedTheme = localStorage.getItem("app-theme") as Theme;
    if (savedTheme && ["light", "dark", "system"].includes(savedTheme)) {
      setTheme(savedTheme);
    }
  }, []);

  // 保存主题到localStorage
  const handleSetTheme = (newTheme: Theme) => {
    setTheme(newTheme);
    localStorage.setItem("app-theme", newTheme);
  };

  const contextValue: ThemeContextType = {
    theme,
    setTheme: handleSetTheme,
  };

  return (
    <ThemeContext.Provider value={contextValue}>
      {children}
    </ThemeContext.Provider>
  );
};
