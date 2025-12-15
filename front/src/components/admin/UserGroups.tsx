import { useEffect, useState } from "react";
import "../../styles/user_groups_styles.css"

interface UserGroups {
  usernames: Array<string>,
  group: number
}

export default function UserGroupsPage() {
    const [usergroups, setUserGroups] = useState<UserGroups[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [newUser, setNewUser] = useState("");

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

    const addUser = async (group: number) => {
        try {
            const userToken: string | null = localStorage.getItem('token');

            const response = await fetch(`${API_BASE}/api/usergroups/adduser`, {
                method: "POST",
                headers: { "Content-Type": "application/json", 'Authorization': `Bearer ${userToken}` },
                body: JSON.stringify({
                    group,
                    user: newUser
                }),
            });

            if (!response.ok) throw new Error("Failed to send user addition to group");

            const data = await response.json();
            if (data.success) {
                fetchUserGroups();
            }
            else {
                console.warn("Server did not confirm success: ", data);
            }
        } catch (err) {
                console.error("Error sending user addition to group: ", err);
        }
    };

    const removeUser = async (group: number, user: string) => {
        try {
            const userToken: string | null = localStorage.getItem('token');

            const response = await fetch(`${API_BASE}/api/usergroups/removeuser`, {
                method: "POST",
                headers: { "Content-Type": "application/json", 'Authorization': `Bearer ${userToken}` },
                body: JSON.stringify({
                    group,
                    user
                }),
            });

            if (!response.ok) throw new Error("Failed to send user deletion from group");

            const data = await response.json();
            if (data.success) {
                fetchUserGroups();
            }
            else {
                console.warn("Server did not confirm success: ", data);
            }
        } catch (err) {
                console.error("Error sending user deletion from group: ", err);
        }
    };

    if (loading) return <p>Loading...</p>;
    if (error) return <p>{error}</p>;

    return (
        <div className="group-grid">
            {usergroups.map(usergroup => 
            <div key={usergroup.group} className="group-card">
                <div className="group-card-content">
                    Group {usergroup.group}
                    {usergroup.usernames.map(user =>
                        <div key={user} className="username">
                            {user}
                            <button onClick={() => removeUser(usergroup.group, user)} className="removeu-btn">Remove</button>
                        </div>
                    )}
                    <div className="addu-row">
                        <input
                            type="text"
                            placeholder="Add user"
                            onChange={(e) => setNewUser(e.target.value)}
                            className="addu-input"
                            />

                            <button onClick={() => addUser(usergroup.group)} className="addu-button">
                            Add
                            </button>
                    </div>
                </div>
            </div>)}
        </div>
    )
}