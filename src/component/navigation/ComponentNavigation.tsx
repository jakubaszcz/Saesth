import {AudioLines, HeadsetIcon, LucideSparkle} from "lucide-react";
import {Navigation} from "../../structures/navigation/Navigation.ts";
import {Props} from "./props.ts";

export function ComponentNavigation({ navigation, changeNavigation }: Props) {

    function RenderIcon(item: Navigation) {
        switch (item) {
            case Navigation.Sounds:
                return <AudioLines/>;
            case Navigation.Setup:
                return <HeadsetIcon />;
            default:
                return <LucideSparkle/>;
        }
    }

    return (
        <div className="h-screen flex flex-col gap-4 px-2">
            {Object.values(Navigation).map((item) => (
                <button
                    key={item}
                    onClick={() => changeNavigation(item)}
                    className={`
                        p-3 rounded-xl
                        flex
                        transition
                        ${
                        item === navigation
                            ? "bg-white/10 border border-white/10 text-(--primary-100)"
                            : "bg-transparent text-(--primary-500) hover:bg-white/5"
                    }
                    `}
                >
                    {RenderIcon(item)}
                </button>
            ))}
        </div>
    );
}