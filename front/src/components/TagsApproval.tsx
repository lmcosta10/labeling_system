import { useEffect, useState, type ReactNode, type MouseEventHandler } from "react";

// --- Types ---
// by Gemini 2.5 Pro

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
 * Defines the props for the Button component.
 */
interface ButtonProps {
  onClick: MouseEventHandler<HTMLButtonElement>;
  className?: string; // Optional class names
  variant?: 'default' | 'destructive';
  children: ReactNode;
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
 * A custom Button component that mimics shadcn's variants.
 */
const Button: React.FC<ButtonProps> = ({ onClick, className, variant = 'default', children }) => {
  // Base styles for all buttons
  const baseStyles =
    "inline-flex items-center justify-center rounded-md text-sm font-medium px-3 py-1.5 transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2";

  // Variant-specific styles
  const variantStyles = {
    default: "bg-gray-900 text-white hover:bg-gray-800 focus:ring-gray-900",
    destructive: "bg-red-600 text-white hover:bg-red-700 focus:ring-red-600",
  };

  // Apply the 'destructive' style if passed, otherwise use 'default'
  const style = variantStyles[variant] || variantStyles.default;

  return (
    <button onClick={onClick} className={`${baseStyles} ${style} ${className || ''}`}>
      {children}
    </button>
  );
};

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
      console.log(data)
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

  if (loading) return <p className="p-4">Loading...</p>;
  if (error) return <p className="p-4 text-red-500">{error}</p>;

  // by Gemini 2.5 Pro:
  return (
    <div>
      <h1 className="text-2xl font-semibold mb-4">Pending Tags</h1>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {tags.filter(tag => tag.pending).map((tag) => (
            // Replaced <Card> with a <div>, adding bg-white and border for a card-like feel
            <div
                key={tag.req_key} // Using req_key for a stable key
                className="shadow-md rounded-2xl bg-white border border-gray-200"
                >
                {/* Replaced <CardContent> with a <div> */}
                <div className="p-4">
                <p className="text-lg font-medium mb-2">{tag.new_name}</p>
                <img src={tag.img_url} />
                <div className="flex gap-2">
                    {/* Now using our custom Button and Check components */}
                    <Button
                    onClick={() => handleApprove(tag.req_key)}
                    className="flex items-center gap-1"
                    >
                    <Check className="w-4 h-4" /> Approve
                    </Button>
                    {/* Now using our custom Button and X components */}
                    <Button
                    onClick={() => handleReject(tag.req_key)}
                    variant="destructive"
                    className="flex items-center gap-1"
                    >
                    <X className="w-4 h-4" /> Reject
                    </Button>
                </div>
                </div>
            </div>
        ))}
      </div>
    </div>
  );
}