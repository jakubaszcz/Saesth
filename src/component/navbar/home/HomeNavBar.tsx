import {X} from "lucide-react";
import {HomeNavigation} from "../../../structures/navbar/home/homeNavBar.ts";
import {Props} from "./Props.ts";

export function HomeNavBar({ navigation, changeNavigation }: Props) {
    return (
        <aside className="h-screen flex flex-col gap-4">
            {Object.values(HomeNavigation).map((item) => (
                <button
                    key={item}
                    onClick={() => changeNavigation(item)}
                    className={`
                        p-3 rounded-xl
                        flex items-center justify-center
                        transition
                        ${
                        item === navigation
                            ? "bg-white/10 border border-white/10 text-(--primary-100)"
                            : "bg-transparent text-(--primary-500) hover:bg-white/5"
                    }
                    `}
                >
                    <X />
                </button>
            ))}
        </aside>
    );
}