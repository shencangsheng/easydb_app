import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import "./i18n";
import { HeroUIProvider } from "@heroui/react";
import { ThemeProvider } from "./contexts/ThemeContext";
import { FontSizeProvider } from "./contexts/FontSizeContext";
import { LanguageProvider } from "./contexts/LanguageContext";
import App from "./App";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <LanguageProvider>
      <ThemeProvider>
        <FontSizeProvider>
          <HeroUIProvider>
            <main className="light text-foreground bg-background">
              <App />
            </main>
          </HeroUIProvider>
        </FontSizeProvider>
      </ThemeProvider>
    </LanguageProvider>
  </StrictMode>
);
