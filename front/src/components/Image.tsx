import { useEffect, useState } from "react";

export type Image = {
  id: number;
  title: string;
  url: string;
};

interface ImageDetailsProps {
    selectedImage: Image;
    onImageClick: () => void;
}

export default function ImageDetails({ selectedImage, onImageClick } : ImageDetailsProps) {

    const [tags, setTags] = useState<string[]>([]);

    const API_BASE = import.meta.env.VITE_API_URL;
    const imageUrlStart = `${API_BASE}/api/image`;

    useEffect(() => {
      async function fetchImageTags() {
        try {
          const imageUrl = `${imageUrlStart}/${selectedImage.id}`;

          const response = await fetch(imageUrl);

          if (!response.ok) throw new Error("Failed to fetch image");

          const data = await response.json();

          if (data.success && typeof data.tags === "string") {
            const parsedTags = data.tags.split(";").map((tag: string) => tag.trim()).filter(Boolean); // TODO
            setTags(parsedTags);
          } else {
            console.warn("Unexpected data format:", data);
            setTags([]);
          }
        } catch (err) {
            console.error(err);
        }
      }
    fetchImageTags();
    }, [imageUrlStart, selectedImage.id]);
    
    return(
    <div>
        <div>
            <button
            onClick={onImageClick}
            ></button>
        </div>
        <div>
            <img src={selectedImage.url} width={200} />
            <p>{selectedImage.title}</p>
        </div>
        <div className="flex flex-wrap gap-3 mb-6">
            {tags.length > 0 ? (
              tags.map((tag) => (
                <span>
                  {tag}<p></p>
                </span>
              ))
            ) : (
              <p>No tags yet.</p>
            )}
        </div>
    </div>
    )
}