import {
    APIFetchPack,
    APIOpenPack, APISelectPack
} from "../../api/packs/packs.ts"
import {useEffect, useState} from "react";
import {Pack} from "../../structures/packs/packs.ts";

export const usePacks = () => {

    const [packs, setPacks] = useState<Pack[]>([]);

    useEffect(() => {
        async function loadPacks() {
            try {
                const response = await APIFetchPack();
                setPacks(response);
            } catch (error) {
                console.error("Failed to fetch packs:", error);
            }
        }

        loadPacks()
    })
    const openPack = async () => {
        try {
            await APIOpenPack();
        } catch (error) {
            console.error("Failed to open packs:", error);
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

    const selectPack = async (id: String) => {
        try {
            await APISelectPack(id);
        } catch (error) {
            console.error("Failed to select pack:", id);
        }
    }

    return { packs, openPack, loadPacks, selectPack };
};