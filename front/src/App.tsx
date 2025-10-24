import { useState } from "react";
import ImageCard from "./components/ImageCard";

const App: React.FC = () => {
  const [name, setName] = useState("");
  const [clickedImage, setClickedImage] = useState<string | null>(null);

  const images = [
    { id: 1, src: "https://picsum.photos/200?random=1", title: "Lizard" },
    { id: 2, src: "https://picsum.photos/200?random=2", title: "Forest" },
    { id: 3, src: "https://picsum.photos/200?random=3", title: "Ocean" },
  ];

  const handleImageClick = (title: string) => {
    setClickedImage(title);
  };

  const handleSubmit = () => {
    alert(`Hello ${name || "stranger"}! You clicked on: ${clickedImage || "nothing"}.`);
  };

  return (
    <div style={{ padding: "2rem", fontFamily: "sans-serif", textAlign: "center" }}>
      <h1>My Simple React + TypeScript Page</h1>

      <div style={{ marginBottom: "1rem" }}>
        <input
          type="text"
          placeholder="Enter your name"
          value={name}
          onChange={(e) => setName(e.target.value)}
          style={{ padding: "0.5rem", marginRight: "0.5rem" }}
        />
        <button onClick={handleSubmit}>Submit</button>
      </div>

      <div
        style={{
          display: "flex",
          justifyContent: "center",
          gap: "1rem",
          flexWrap: "wrap",
        }}
      >
        {images.map((img) => (
          <ImageCard
            key={img.id}
            src={img.src}
            title={img.title}
            onClick={() => handleImageClick(img.title)}
          />
        ))}
      </div>

      {clickedImage && (
        <p style={{ marginTop: "1rem" }}>
          You clicked on: <strong>{clickedImage}</strong>
        </p>
      )}
    </div>
  );
};

export default App;
