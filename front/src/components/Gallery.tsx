import { useEffect, useState } from "react";
import {type Image} from "./Image";

interface GalleryProps {
    onImageClick: (image: Image) => void;
}

export default function Gallery({ onImageClick }: GalleryProps) {
    const [images, setImages] = useState<Image[]>([]);
    const [loading, setLoading] = useState(true);
    
    useEffect(() => {
        async function fetchImages() {
            const API_BASE = import.meta.env.VITE_API_URL;
            try {
            const response = await fetch(`${API_BASE}/api/images`);
            if (!response.ok) throw new Error("Failed to fetch images");
            const data = await response.json();
            setImages(data);
            } catch (err) {
            console.error(err);
            } finally {
            setLoading(false);
            }
        }
    fetchImages();
    }, []); // empty dependency array => run once on mount

    if (loading) return <p>Loading...</p>;

    return (
    <div>
        <h2>Image Gallery</h2>
        {images.map((img) => (
        <div key={img.id} onClick={() => onImageClick(img)}>
            <img src={img.url} width={200} />
            <p>{img.title}</p>
        </div>
        ))}
    </div>
    );
}
