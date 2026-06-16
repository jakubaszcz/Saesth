import {CloudRain, Flame, Volleyball, Volume2, ZodiacAquarius} from "lucide-react";

export const getSoundIcon = (id: string) => {
    const nid = id.toLowerCase();

    if (nid.includes("rain")) return <CloudRain size={20} />;
    if (nid.includes("beach")) return <Volleyball size={20} />;
    if (nid.includes("waterfall")) return <ZodiacAquarius size={20}/>
    if (nid.includes("fire")) return <Flame size={20}/>

    return <Volume2 size={24} />;
};