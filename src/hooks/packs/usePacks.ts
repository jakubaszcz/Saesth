import {listen} from "@tauri-apps/api/event";
import {
    APIFetchPack,
    APIOpenPack, APISelectPack, APIGetSelectedPack, APIDeselectPack, APIOpenTempPack
} from "../../api/packs/packs.ts"
import {useEffect, useState} from "react";
import {Pack} from "../../structures/packs/packs.ts";

export const usePacks = () => {

    const [packs, setPacks] = useState<Pack[]>([]);
    const [tempPack, setTempPack] = useState<Pack | null>(null);
    const [selectedPack, setSelectedPack] = useState<string | null>(null);

    useEffect(() => {
        async function loadPacks() {
            try {
                const response = await APIFetchPack();
                setPacks(response);
                setSelectedPack(await APIGetSelectedPack());
            } catch (error) {
                console.error("Failed to fetch packs:", error);
            }
        }

        let disposed = false;
        const subscription = listen("packs-changed", () => {
            if (!disposed) void loadPacks();
        });
        subscription.then(unlisten => {
            if (disposed) unlisten();
            else void loadPacks();
        }).catch(error => console.error("Failed to watch packs:", error));
        return () => {
            disposed = true;
            void subscription.then(unlisten => unlisten()).catch(() => {});
        };
    }, [])
    const openPack = async () => {
        try {
            await APIOpenPack();
        } catch (error) {
            console.error("Failed to open packs:", error);
        }
    };

    const openTempPack = async () => {
        try {
            const response = await APIOpenTempPack();
            if (!response) return false;
            setTempPack(response);
            setSelectedPack(response.id);
            return true;
        } catch (error) {
            console.error("Failed to open temporary packs:", error);
            return false;
        }
    };

    const loadPacks = async () => {
        try {
            const response = await APIFetchPack();
            setPacks(response);
        } catch (error) {
            console.error("Failed to fetch packs:", error);
        }
    };

    const selectPack = async (id: string) => {
        try {
            await APISelectPack(id);
            setTempPack(null);
            setSelectedPack(id);
        } catch (error) {
            console.error("Failed to select pack:", id);
        }
    }

    const deselectPack = async () => {
        try {
            await APIDeselectPack();
            setTempPack(null);
            setSelectedPack(null);
        } catch (error) {
            console.error("Failed to deselect pack:", error);
        }
    };

    return { packs, tempPack, openPack, openTempPack, loadPacks, selectPack, deselectPack, selectedPack };
};