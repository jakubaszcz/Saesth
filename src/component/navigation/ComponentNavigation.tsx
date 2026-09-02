import {AudioLines, BoltIcon, HeadsetIcon, LucideSparkle} from "lucide-react";
import {Navigation} from "../../structures/navigation/Navigation.ts";
import {Props} from "./props.ts";

export function ComponentNavigation({ navigation, changeNavigation }: Props) {

    function RenderIcon(item: Navigation, size: number = 20) {
        switch (item) {
            case Navigation.Sounds:
                return <AudioLines size={size}/>;
            case Navigation.Setup:
                return <HeadsetIcon size={size}/>;
            case Navigation.Settings:
                return <BoltIcon size={size}/>;
            default:
                return <LucideSparkle/>;
        }
    }

    return (
        <div className="flex flex-col h-full items-center gap-5 p-(--padding-md) pt-0">
            <div className="flex flex-col items-center gap-5">
                {Object.values(Navigation)
                    .filter(item => item !== Navigation.Settings)
                    .map((item) => (
                        <button
                            className={`p-2 rounded-xl transition-all duration-300 cursor-pointer flex items-center justify-center ${item === navigation ? "bg-primary-800 text-primary-200 scale-110 shadow-lg" : "text-primary-600 hover:bg-primary-800/50 hover:text-primary-400"}`}
                            key={item}
                            onClick={() => changeNavigation(item)}
                        >
                            {RenderIcon(item)}
                        </button>
                    ))}
            </div>

            <div className="mt-auto">
                <button
                    className={`p-2 rounded-xl transition-all duration-300 cursor-pointer flex items-center justify-center ${navigation === Navigation.Settings ? "bg-primary-800 text-primary-200 scale-110 shadow-lg" : "text-primary-600 hover:bg-primary-800/50 hover:text-primary-400"}`}
                    key={Navigation.Settings}
                    onClick={() => changeNavigation(Navigation.Settings)}
                >
                    {RenderIcon(Navigation.Settings)}
                </button>
            </div>
        </div>
    );
}