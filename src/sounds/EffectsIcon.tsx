import {CloudLightning, Sparkle} from "lucide-react";

export const getSoundEffectIcon = (id: string, effect_id: string, iconSize = 24) => {
    if (id === "rain") {
        if (effect_id === "thunder") return <CloudLightning size={iconSize}/>
    }

    return <Sparkle size={iconSize}/>
}