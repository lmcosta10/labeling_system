import { useEffect, useState } from "react";
import "../../styles/user_groups_styles.css"

interface UserGroups {
  usernames: Array<string>,
  group: number
}

export default function TagsApprovalPage() {
    const [usergroups, setUserGroups] = useState<UserGroups[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    const API_BASE = import.meta.env.VITE_API_URL;
    // const userToken: string | null = localStorage.getItem('token');

    async function fetchUserGroups() {
        try {
            setLoading(true);
            setError(null);
            const userToken: string | null = localStorage.getItem('token');
            const response = await fetch(`${API_BASE}/api/usergroups`, {
                method: 'GET',
                headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${userToken}`}
            }
            );
            if (!response.ok) throw new Error("Failed to fetch user groups");
            const data = await response.json();
            setUserGroups(data);
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

    useEffect(() => {
        fetchUserGroups();
    }, []);

    if (loading) return <p>Loading...</p>;
    if (error) return <p>{error}</p>;

    return (
        <div className="group-grid">
            {usergroups.map(usergroup => 
            <div key={usergroup.group} className="group-card">
                <div className="group-card-content">
                Group {usergroup.group}
                {usergroup.usernames.map(user =>
                    <p key={user} className="username">{user}</p>
                )}
                </div>
            </div>)}
        </div>
    )
}