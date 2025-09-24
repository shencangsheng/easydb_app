import { BrowserRouter as Router, Route, Routes } from "react-router-dom";

import Notebook from "./pages/notebook";

// Changed to default import
function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<Notebook />} />
      </Routes>
    </Router>
  );
}

export default App;
