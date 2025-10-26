import React, { useState, useEffect } from "react";
import Gallery from "./components/Gallery";
import Login from "./components/Login";

const App: React.FC = () => {
  const [loggedIn, setLoggedIn] = useState(false);

  useEffect(() => {
    // check saved login status
    const logged = localStorage.getItem("loggedIn") === "true";

    setLoggedIn(logged);
  }, []);

  const handleLoginSuccess = () => {
    setLoggedIn(true);
  };

  const handleLogout = () => {
    localStorage.clear();
    setLoggedIn(false);
  };

  return (
    <div>
      <h1>Page</h1>

      {!loggedIn ? (
        <Login onLoginSuccess={handleLoginSuccess} />
      ) : (
        <>
          <p>Welcome User!</p>
          <Gallery />
          <button onClick={handleLogout}>Logout</button>
        </>
      )}
    </div>
  );
};

export default App;
