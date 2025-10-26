import React, { useState, useEffect } from "react";
import ImageDetails, {type Image} from "./components/Image";
import Gallery from "./components/Gallery";
import Login from "./components/Login";

const App: React.FC = () => {
  const [loggedIn, setLoggedIn] = useState(false);
  const [selectedImage, setSelectedImage] = useState<Image | null>(null);

  useEffect(() => {
    // TODO: use token
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

  const handleImageClick = (image: Image) => {
    setSelectedImage(image);
  };

  const handleBackToGallery = () => {
    setSelectedImage(null);
  };

  return (
    <div>
      <h1>Page</h1>

      {!loggedIn ? (
        <Login onLoginSuccess={handleLoginSuccess} />
      ) : !selectedImage? (
        <>
          <p>Welcome User!</p>
          <Gallery onImageClick={handleImageClick}/>
          <button onClick={handleLogout}>Logout</button>
          
        </>
      ) : (
        <ImageDetails selectedImage={selectedImage} onImageClick={handleBackToGallery}/>
      )}
    </div>
  );
};

export default App;
