import { createContext } from "react";
import { Language } from "../i18n";

export interface LanguageContextType {
  language: Language;
  setLanguage: (language: Language) => Promise<void>;
}

export const LanguageContext = createContext<LanguageContextType | undefined>(
  undefined
);
