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

    useEffect(() => {
        async function fetchImageTags() {
            // TODO: remove: MOCK TAGS
            setTags(["tag1", "tag2"]);

            const API_BASE = import.meta.env.VITE_API_URL;
            try {
            const response = await fetch(`${API_BASE}/api/image/:id`);
            if (!response.ok) throw new Error("Failed to fetch image");
            } catch (err) {
                console.error(err);
            }
        }
    fetchImageTags();
    }, [tags]);
    
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
                  {tag}
                </span>
              ))
            ) : (
              <p>Nenhuma tag adicionada ainda.</p>
            )}
        </div>
    </div>
    )
}