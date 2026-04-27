import {CloudRain, Volume2} from "lucide-react";

export const getSoundIcon = (id: string) => {
    const nid = id.toLowerCase();

    if (nid.includes("rain")) return <CloudRain size={24} />;

    return <Volume2 size={24} />;
};