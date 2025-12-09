import { useEffect, useState } from "react";
import "../../styles/tags_approval_styles.css"

// --- Types ---

/**
 * Defines the structure of a Tag object.
 */
interface TagRequest {
  req_key: number,
  img_url: string;
  operation: string;
  old_name: string;
  new_name: string;
  pending: boolean
}

/**
 * Defines the props for the Check icon component.
 */
interface CheckProps {
  className?: string; // Optional class names
}

/**
 * Defines the props for the X icon component.
 */
interface XProps {
  className?: string; // Optional class names
}


// --- Custom Components ---
// by Gemini 2.5 Pro

/**
 * A custom Check icon component using SVG.
 */
const Check: React.FC<CheckProps> = ({ className }) => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    fill="none"
    viewBox="0 0 24 24"
    strokeWidth={2.5}
    stroke="currentColor"
    className={className}
  >
    <path strokeLinecap="round" strokeLinejoin="round" d="m4.5 12.75 6 6 9-13.5" />
  </svg>
);

/**
 * A custom X icon component using SVG.
 */
const X: React.FC<XProps> = ({ className }) => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    fill="none"
    viewBox="0 0 24 24"
    strokeWidth={2.5}
    stroke="currentColor"
    className={className}
  >
    <path strokeLinecap="round" strokeLinejoin="round" d="M6 18 18 6M6 6l12 12" />
  </svg>
);

// -- Logics --

export default function TagsApprovalPage() {
  const [tags, setTags] = useState<TagRequest[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const API_BASE = import.meta.env.VITE_API_URL;
  const userToken: string | null = localStorage.getItem('token');

  async function fetchPendingTags() {
    try {
      setLoading(true);
      setError(null);
      const userToken: string | null = localStorage.getItem('token');
      const response = await fetch(`${API_BASE}/api/tags/pending`, {
            method: 'GET',
            headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${userToken}`}
        }
      );
      if (!response.ok) throw new Error("Failed to fetch tags");
      const data = await response.json();
      setTags(data);
    } catch (e) {
      if (e instanceof Error) {
        setError(e.message);
      } else {
        setError("An unknown error occurred");
      }
    } finally {
      setLoading(false);
    }
  }

  async function handleApprove(req_key: number) {
    await fetch(`${API_BASE}/api/tags/${req_key}/approve`, { method: "POST", headers: { 'Authorization': `Bearer ${userToken}` }});
    fetchPendingTags();
  }

  async function handleReject(req_key: number) {
    await fetch(`${API_BASE}/api/tags/${req_key}/reject`, { method: "POST", headers: { 'Authorization': `Bearer ${userToken}` }});
    fetchPendingTags();
  }

  useEffect(() => {
    fetchPendingTags();
  }, []);

  if (loading) return <p>Loading...</p>;
  if (error) return <p>{error}</p>;

  return (
    <div className="pending-wrapper">
      <h1 className="pending-title">Pending Tags</h1>

      <div className="pending-grid">
        {tags.filter((tag) => tag.pending).map((tag) => (
          <div key={tag.req_key} className="pending-card">
            <div className="pending-card-content">
              <p className="old-name">Old tag: {tag.old_name}</p>
              <p className="pending-name">New tag: {tag.new_name}</p>

              <img src={tag.img_url} className="pending-image" />

              <div className="pending-actions">
                <button
                  onClick={() => handleApprove(tag.req_key)}
                  className="pending-btn-approve"
                >
                  <Check className="icon-small" /> Approve
                </button>

                <button
                  onClick={() => handleReject(tag.req_key)}
                  className="pending-btn-reject"
                >
                  <X className="icon-small" /> Reject
                </button>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}