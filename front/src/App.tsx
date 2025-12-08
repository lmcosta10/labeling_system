import React, { useState, useEffect } from "react";
import ImageDetails, { type Image } from "./components/Image";
import Gallery from "./components/Gallery";
import Login from "./components/Login";
import TagsApprovalPage from "./components/TagsApproval";
import "./styles/styles.css"

const App: React.FC = () => {
  const [loggedIn, setLoggedIn] = useState(false);
  const [isAdmin, setIsAdmin] = useState(false);
  const [selectedImage, setSelectedImage] = useState<Image | null>(null);
  const [view, setView] = useState<"gallery" | "details" | "admin">("gallery");

  useEffect(() => {
    const logged = localStorage.getItem("loggedIn") === "true";
    setLoggedIn(logged);

    const admin = localStorage.getItem("isAdmin") === "true";
    setIsAdmin(admin);
  }, []);

  const handleLoginSuccess = () => {
    setLoggedIn(true);
    const admin = localStorage.getItem("isAdmin") === "true";
    setIsAdmin(admin);
  };

  const handleLogout = () => {
    localStorage.clear();
    setLoggedIn(false);
    setIsAdmin(false);
    setView("gallery");
  };

  const handleImageClick = (image: Image) => {
    setSelectedImage(image);
    setView("details");
  };

  const handleBackToGallery = () => {
    setSelectedImage(null);
    setView("gallery");
  };

  const goToAdminPanel = () => {
    if (isAdmin) setView("admin");
  };

  return (
    <div>
      <h1>Labeling System</h1>

      {!loggedIn ? (
        <Login onLoginSuccess={handleLoginSuccess} />
      ) : view === "admin" && isAdmin ? (
        <>
          <button onClick={() => setView("gallery")}>Back</button>
          <TagsApprovalPage />
          <button onClick={handleLogout}>Logout</button>
        </>
      ) : view === "details" && selectedImage ? (
        <>
          <ImageDetails selectedImage={selectedImage} onImageClick={handleBackToGallery} />
          <button onClick={handleLogout}>Logout</button>
        </>
      ) : (
        <>
          <p>Welcome!</p>
          <Gallery onImageClick={handleImageClick} />
          {isAdmin && (
            <button onClick={goToAdminPanel}>Go to Admin Panel</button>
          )}
          <div>
            <button onClick={handleLogout}>Logout</button>
          </div>
        </>
      )}
    </div>
  );
};

export default App;
