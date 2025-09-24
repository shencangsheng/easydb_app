import { createContext } from "react";

export interface FontSizeContextType {
  fontSize: number;
  setFontSize: (size: number) => void;
}

export const FontSizeContext = createContext<FontSizeContextType | undefined>(
  undefined
);
