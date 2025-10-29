import { useEffect, useState } from "react";

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
          const approvedFlags: number[] = data.tags_approved;

          // Combine both arrays into TagState[]
          const combined = names.map((name, i) => ({
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
    <div className="p-4">
      <div>
        <button onClick={onImageClick}>Back</button>
      </div>

      <div className="my-4">
        <img src={selectedImage.url} width={200} />
      </div>

      <div className="flex flex-wrap gap-3 mb-6">
        {tags.length > 0 ? (
            tags.map((tag, index) => (
              <div
                key={`${tag.name}-${index}`}
                className={`flex items-center gap-2 px-3 py-1 rounded-full ${
                  tag.approved ? "bg-gray-100" : "bg-yellow-100"
                }`}
              >
              {editingTag === tag.name ? (
                <>
                  <input
                    type="text"
                    value={editedValue}
                    onChange={(e) => setEditedValue(e.target.value)}
                    className="border px-2 rounded"
                  />
                  <button
                    onClick={handleSaveEdit}
                    className="text-green-600 hover:underline"
                  >
                    Save
                  </button>
                  <button
                    onClick={handleCancelEdit}
                    className="text-gray-500 hover:underline"
                  >
                    Cancel
                  </button>
                </>
              ) : (
                <>
                  <span>{tag.name}</span>
                  {!tag.approved && (
                    <span className="text-xs text-yellow-600 italic">
                      (pending)
                    </span>
                  )}
                  {tag.approved && (
                    <>
                      <button
                        onClick={() => handleEditTag(tag.name)}
                        className="text-blue-600 hover:underline"
                      >
                        Edit
                      </button>
                      <button
                        onClick={() => handleDeleteTag(tag.name)}
                        className="text-red-600 hover:underline"
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
        <button
          onClick={getAISuggestion}
          className="text-gray-500 hover:underline"
        >
          Get AI Suggestion
        </button>

        {aiSuggestion && (
          <div className="mt-2 p-2 border border-gray-300 rounded bg-gray-50">
            {aiSuggestion}
          </div>
        )}
      </div>

      <div className="flex gap-2">
        <input
          type="text"
          placeholder="Add new tag..."
          value={newTag}
          onChange={(e) => setNewTag(e.target.value)}
          className="border rounded px-2 py-1"
        />
        <button
          onClick={handleAddTag}
          className="bg-blue-600 text-white px-3 py-1 rounded"
        >
          Add
        </button>
      </div>
    </div>
  );
}
