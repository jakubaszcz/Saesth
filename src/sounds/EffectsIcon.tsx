import {CloudLightning, Origami, Sparkle, Triangle} from "lucide-react";

export const getSoundEffectIcon = (id: string, effect_id: string, iconSize = 24) => {
    if (id === "rain") {
        if (effect_id === "thunder") return <CloudLightning size={iconSize}/>
        if (effect_id === "triangle") return <Triangle size={iconSize}/>
    }
    if (id === "beach") {
        if (effect_id === "seagull") return <Origami size={iconSize}/>
    }

    return <Sparkle size={iconSize}/>
}