import { Input } from "@/components/ui/input"
import ActionCard from "../ActionCard";
export default function ActionSelect() {
    return (
        <div className="action-select-container">
            <Input placeholder="Select an action" />
            <div className="action-list mt-4">
                <ul className="grid grid-cols-2 gap-4">
                    <li className="w-full"><ActionCard /></li>
                    <li className="w-full"><ActionCard /></li>
                    <li className="w-full"><ActionCard /></li>
                    <li className="w-full"><ActionCard /></li>
                    <li className="w-full"><ActionCard /></li>
                </ul>
            </div>
        </div>
    );
}