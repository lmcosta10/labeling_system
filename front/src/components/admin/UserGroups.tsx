import { useEffect, useState } from "react";
// import "../../styles/user_groups_styles.css"

interface UserGroups {
  username: string,
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
            console.log(data)
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
        <div>
            {usergroups.map(usergroup => 
            <div>
                <p>{usergroup.username}</p>
                <p>{usergroup.group}</p>
            </div>)}
        </div>
    )
}