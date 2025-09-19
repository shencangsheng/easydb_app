import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import { HeroUIProvider } from "@heroui/react";
import App from "./App";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <HeroUIProvider>
      <main className="light text-foreground bg-background">
        <App />
      </main>
    </HeroUIProvider>
  </StrictMode>
);
