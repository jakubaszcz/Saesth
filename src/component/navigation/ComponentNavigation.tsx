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
        <div className="flex flex-col h-full items-center gap-5 p-(--padding-md)">
            <div className="flex flex-col items-center gap-5">
                {Object.values(Navigation)
                    .filter(item => item !== Navigation.Settings)
                    .map((item) => (
                        <button
                            className={`text-primary-700 hover:text-primary-600 transition-all duration-300 cursor-pointer ${item === navigation ? "text-primary-400 scale-110" : ""}`}
                            key={item}
                            onClick={() => changeNavigation(item)}
                        >
                            {RenderIcon(item)}
                        </button>
                    ))}
            </div>

            <div className="mt-auto">
                <button
                    className={`text-(--color-primary-700) hover:text-(--color-primary-600) transition-all duration-300 cursor-pointer ${navigation === Navigation.Settings ? "text-primary-400 scale-110" : ""}`}
                    key={Navigation.Settings}
                    onClick={() => changeNavigation(Navigation.Settings)}
                >
                    {RenderIcon(Navigation.Settings)}
                </button>
            </div>
        </div>
    );
}