import { useEffect, useState } from "react";
import {type Image} from "./Image";
import "../styles/gallery_styles.css"

interface GalleryProps {
    onImageClick: (image: Image) => void;
}

export default function Gallery({ onImageClick }: GalleryProps) {
    const [images, setImages] = useState<Image[]>([]);
    const [imageSrcs, setImageSrcs] = useState<Record<number, string>>({});
    const [loading, setLoading] = useState(true);
    
    useEffect(() => {
        const API_BASE = import.meta.env.VITE_API_URL;
        const objectUrls: string[] = [];
        async function fetchImages() {
            try {
                const userToken: string | null = localStorage.getItem('token');
                const response = await fetch(`${API_BASE}/api/images`, {
                    method: 'GET',
                    headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${userToken}`}
                });
            if (!response.ok) throw new Error("Failed to fetch images");
            const imgs: Image[] = await response.json();
            setImages(imgs);

            const srcMap: Record<number, string> = {};

            await Promise.all(
                imgs.map(async (img) => {
                    const imgRes = await fetch(img.url, {
                    headers: {
                        Authorization: `Bearer ${userToken}`,
                    },
                    });

                    if (!imgRes.ok) return;

                    const blob = await imgRes.blob();
                    const objectUrl = URL.createObjectURL(blob);

                    objectUrls.push(objectUrl);
                    srcMap[img.id] = objectUrl;

                    img.blob = objectUrl;
                })
            );
            setImageSrcs(srcMap);
            } catch (err) {
            console.error(err);
            } finally {
            setLoading(false);
            }
        }
    fetchImages();

    return () => {
      objectUrls.forEach((url) => URL.revokeObjectURL(url));
    };
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
                    {imageSrcs[img.id] ? (
                    <img
                        src={imageSrcs[img.id]}
                        className="gallery-image"
                    />
                    ) : (
                    <div className="image-placeholder">Loading...</div>
                    )}
                </div>
            ))}
        </div>
    </div>
    );

}
