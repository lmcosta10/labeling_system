import { useEffect, useState } from "react";
import "../styles/image_page_styles.css"

export type Image = {
  id: number;
  url: string;
};

interface ImageDetailsProps {
  selectedImage: Image;
  onImageClick: () => void;
}

type TagState = {
  name: string;
  approved?: boolean;
  action?: "add" | "edit" | "delete";
};

export default function ImageDetails({ selectedImage, onImageClick }: ImageDetailsProps) {
  const [tags, setTags] = useState<TagState[]>([]);
  const [newTag, setNewTag] = useState("");
  const [editingTag, setEditingTag] = useState<string | null>(null);
  const [editedValue, setEditedValue] = useState("");
  const [aiSuggestion, setAISuggestion] = useState<string | null>(null);

  const API_BASE = import.meta.env.VITE_API_URL;
  const imageUrlStart = `${API_BASE}/api/image`;

  // Fetch tags from server
  useEffect(() => {
    async function fetchImageTags() {
      try {
        const imageUrl = `${imageUrlStart}/${selectedImage.id}`;
        const response = await fetch(imageUrl);

        if (!response.ok) throw new Error("Failed to fetch image");

        const data = await response.json();

        if (data.success) {
          const names: string[] = data.tags_names;

          // Combine both arrays into TagState[]
          const combined = names.map((name) => ({
            name,
            approved: true, // TODO
          }));

          setTags(combined);
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

  // Send approved tag modification to server
  async function sendTagForApproval(action: "add" | "edit" | "delete", tagName: string, newName?: string) {
    try {
      const response = await fetch(`${imageUrlStart}/${selectedImage.id}/tags`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          action,
          tag: tagName,
          ...(newName ? { newName } : {}),
        }),
      });

      if (!response.ok) throw new Error("Failed to send tag for approval");

      const data = await response.json();
      if (!data.success) console.warn("Server did not confirm success:", data);
    } catch (err) {
      console.error("Error sending tag for approval:", err);
    }
  }

  // Get AI suggestion
  async function getAISuggestion() {
    try {
      const response = await fetch(`${imageUrlStart}/${selectedImage.id}/ai`, {
        method: "GET"
      });

      if (!response.ok) throw new Error("Failed to get AI suggestion");

      const data = await response.json();
      console.log(data.ai_response);

      if (!data.success) {
        console.warn("Server did not confirm success:", data);
      }

      setAISuggestion(data.ai_response ?? "No suggestion found");
    } catch (err) {
      console.error("Error getting AI suggestion:", err);
      setAISuggestion("Error getting suggestion");
    }
  };


  // Add new tag (not approved / pending)
  const handleAddTag = async () => {
    const trimmed = newTag.trim();
    if (!trimmed || tags.some((t) => t.name === trimmed)) return;

    const newTagObj: TagState = { name: trimmed, approved: false, action: "add" }; // this "approved" serves frontend purposes only,
    // and can be manipulated by the user, but the database should not be affected
    setTags((prev) => [...prev, newTagObj]);
    setNewTag("");
    await sendTagForApproval("add", trimmed);
  };

  // Delete tag (not approved / pending)
  const handleDeleteTag = async (tagToDelete: string) => {
    setTags((prev) =>
      prev.map((t) =>
        t.name === tagToDelete ? { ...t, approved: false, action: "delete" } : t  // this "approved" serves frontend purposes only,
        // and can be manipulated by the user, but the database should not be affected
      )
    );
    await sendTagForApproval("delete", tagToDelete);
  };

  // Start editing
  const handleEditTag = (tag: string) => {
    setEditingTag(tag);
    setEditedValue(tag);
  };

  // Save edit (approved)
  const handleSaveEdit = async () => {
    if (!editingTag) return;
    const newName = editedValue.trim();
    if (!newName) return;

    setTags((prev) =>
      prev.map((t) =>
        t.name === editingTag
          ? { ...t, approved: false, action: "edit", name: newName }
          : t
      )
    );
    setEditingTag(null);
    await sendTagForApproval("edit", editingTag, newName);
  };

  const handleCancelEdit = () => {
    setEditingTag(null);
    setEditedValue("");
  };

  return (
    <div className="image-page-wrapper">
      <div>
        <button onClick={onImageClick}>Back</button>
      </div>

      <div className="image-container">
        <img src={selectedImage.url} width={200} className="selected-img" />
      </div>

      <div className="tags-wrapper">
        {tags.length > 0 ? (
          tags.map((tag, index) => (
            <div
              key={`${tag.name}-${index}`}
              className={`tag-item ${
                tag.approved ? "tag-approved" : "tag-pending"
              }`}
            >
              {editingTag === tag.name ? (
                <>
                  <input
                    type="text"
                    value={editedValue}
                    onChange={(e) => setEditedValue(e.target.value)}
                    className="tag-edit-input"
                  />

                  <button onClick={handleSaveEdit}>
                    Save
                  </button>

                  <button onClick={handleCancelEdit}>
                    Cancel
                  </button>
                </>
              ) : (
                <>
                  <span>{tag.name}</span>

                  {!tag.approved && (
                    <span className="tag-pending-text">(pending)</span>
                  )}

                  {tag.approved && (
                    <>
                      <button
                        onClick={() => handleEditTag(tag.name)}
                        className="btn-edit"
                      >
                        Edit
                      </button>

                      <button
                        onClick={() => handleDeleteTag(tag.name)}
                        className="btn-delete"
                      >
                        X
                      </button>
                    </>
                  )}
                </>
              )}
            </div>
          ))
        ) : (
          <p>No tags yet.</p>
        )}
      </div>

      <div className="ai-suggestion-wrapper">
        <button onClick={getAISuggestion} className="btn-ai">
          Get AI Suggestion
        </button>

        {aiSuggestion && <p className="ai-box">{aiSuggestion}</p>}
      </div>

      <div className="add-tag-row">
        <input
          type="text"
          placeholder="Add new tag..."
          value={newTag}
          onChange={(e) => setNewTag(e.target.value)}
          className="add-tag-input"
        />

        <button onClick={handleAddTag} className="add-tag-button">
          Add
        </button>
      </div>
    </div>
  );
}
