import { useEffect, useState } from "react";
import {type Image} from "./Image";
import "../styles/gallery_styles.css"

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
                const userToken: string | null = localStorage.getItem('token');
                const response = await fetch(`${API_BASE}/api/images`, {
                    method: 'GET',
                    headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${userToken}`}
                });
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
    
    // TODO: style here
    return (
    <div className="gallery-wrapper">
        <h2 className="gallery-title">Image Gallery</h2>

        <div className="gallery-grid">
        {images.map((img) => (
            <div
            key={img.id}
            className="gallery-item"
            onClick={() => onImageClick(img)}
            >
            <img src={img.url} className="gallery-image" />
            </div>
        ))}
        </div>
    </div>
    );

}
