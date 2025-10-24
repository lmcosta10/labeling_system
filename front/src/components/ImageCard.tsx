import React from "react";

interface ImageCardProps {
  src: string;
  title: string;
  onClick: () => void;
}

const ImageCard: React.FC<ImageCardProps> = ({ src, title, onClick }) => {
  return (
    <div
      onClick={onClick}
      style={{
        cursor: "pointer",
        border: "2px solid #ccc",
        borderRadius: "10px",
        padding: "0.5rem",
        width: "200px",
        transition: "0.3s",
      }}
    >
      <img
        src={src}
        alt={title}
        style={{ width: "100%", borderRadius: "10px" }}
      />
      <p style={{ marginTop: "0.5rem" }}>{title}</p>
    </div>
  );
};

export default ImageCard;
