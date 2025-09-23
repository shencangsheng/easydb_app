import { BrowserRouter as Router, Route, Routes } from "react-router-dom";

import Notebook from "./pages/notebook";
import SettingsPage from "./pages/settings";
import AboutPage from "./pages/about";

// Changed to default import
function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<Notebook />} />
        <Route path="/settings" element={<SettingsPage />} />
        <Route path="/about" element={<AboutPage />} />
      </Routes>
    </Router>
  );
}

export default App;
