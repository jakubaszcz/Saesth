import {
    APIFetchPack,
    APIOpenPack, APISelectPack, APIGetSelectedPack, APIDeselectPack, APIOpenTempPack
} from "../../api/packs/packs.ts"
import {useEffect, useState} from "react";
import {Pack} from "../../structures/packs/packs.ts";

export const usePacks = () => {

    const [packs, setPacks] = useState<Pack[]>([]);
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

        loadPacks()
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
            await APIOpenTempPack();
        } catch (error) {
            console.error("Failed to open temporary packs:", error);
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
            setSelectedPack(id);
        } catch (error) {
            console.error("Failed to select pack:", id);
        }
    }

    const deselectPack = async () => {
        try {
            await APIDeselectPack();
            setSelectedPack(null);
        } catch (error) {
            console.error("Failed to deselect pack:", error);
        }
    };

    return { packs, openPack, openTempPack, loadPacks, selectPack, deselectPack, selectedPack };
};