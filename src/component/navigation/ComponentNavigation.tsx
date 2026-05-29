import {AudioLines, BoltIcon, HeadsetIcon, LucideSparkle} from "lucide-react";
import {Navigation} from "../../structures/navigation/Navigation.ts";
import {Props} from "./props.ts";

export function ComponentNavigation({ changeNavigation }: Props) {

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
        <div>
            {Object.values(Navigation)
                .filter(item => item !== Navigation.Settings)
                .map((item) => (
                    <button
                        key={item}
                        onClick={() => changeNavigation(item)}
                    >
                        {RenderIcon(item)}
                    </button>
                ))}

            <div/>

            <button
                key={Navigation.Settings}
                onClick={() => changeNavigation(Navigation.Settings)}
            >
                {RenderIcon(Navigation.Settings)}
            </button>
        </div>
    );
}