import { useState } from "react";
import TagsApprovalPage from "./TagsApproval";
import UserGroupsPage from "./UserGroups";

export default function AdminPage() {
    const handleUserGroupsClick = () => {
        setView("usergroups");
    };

    const handleTagsApprovalClick = () => {
        setView("tagsapproval");
    };

    const [view, setView] = useState<"tagsapproval" | "usergroups">("tagsapproval");

    return (
        <div>
            {view === "tagsapproval" ? (
                <div>
                    <button onClick={(handleUserGroupsClick)}>User Groups</button>
                    <TagsApprovalPage />
                </div>
            ) : (
                <div>
                    <button onClick={(handleTagsApprovalClick)}>Tags Approval</button>
                    <UserGroupsPage />
                </div>
            )}
        </div>
    );
};
