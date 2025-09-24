import { useContext } from "react";
import { FontSizeContext } from "../contexts/FontSizeContextTypes";

export const useFontSize = () => {
  const context = useContext(FontSizeContext);
  if (!context) {
    throw new Error("useFontSize must be used within a FontSizeProvider");
  }
  return context;
};
