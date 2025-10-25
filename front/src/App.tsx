//import { useState } from "react";
import Gallery from "./components/Gallery";

const App: React.FC = () => {
  return (
    <div style={{ padding: "2rem", fontFamily: "sans-serif", textAlign: "center" }}>
      <h1>Page</h1>

      <Gallery />
    </div>
  );
};

export default App;
