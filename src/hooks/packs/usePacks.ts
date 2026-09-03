import {
    APIOpenPack
} from "../../api/packs/packs.ts"

export const usePacks = () => {
    const openPack = async () => {
        try {
            await APIOpenPack();
        } catch (error) {
            console.error("Failed to open packs:", error);
        }
    };

    return { openPack };
};