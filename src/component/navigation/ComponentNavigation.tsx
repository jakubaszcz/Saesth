import {AudioLines, BoltIcon, HeadsetIcon, LucideSparkle} from "lucide-react";
import {Navigation} from "../../structures/navigation/Navigation.ts";
import {Props} from "./props.ts";

export function ComponentNavigation({ navigation, changeNavigation }: Props) {

    function RenderIcon(item: Navigation) {
        switch (item) {
            case Navigation.Sounds:
                return <AudioLines/>;
            case Navigation.Setup:
                return <HeadsetIcon />;
            case Navigation.Settings:
                return <BoltIcon/>;
            default:
                return <LucideSparkle/>;
        }
    }

    return (
        <div className="h-full flex flex-col gap-4 px-2 pb-2">
            {Object.values(Navigation)
                .filter(item => item !== Navigation.Settings)
                .map((item) => (
                    <button
                        key={item}
                        onClick={() => changeNavigation(item)}
                        className={`
                        flex
                        transition
                        ${
                            item === navigation
                                ? " text-(--primary-100)"
                                : " text-(--primary-500)"
                        }
                    `}
                    >
                        {RenderIcon(item)}
                    </button>
                ))}

            <div className="flex-1"/>

            <button
                key={Navigation.Settings}
                onClick={() => changeNavigation(Navigation.Settings)}
                className={`
                    flex
                    transition
                    ${
                    navigation === Navigation.Settings
                        ? "text-(--primary-100)"
                        : "text-(--primary-500)"
                }
                `}
            >
                {RenderIcon(Navigation.Settings)}
            </button>
        </div>
    );
}