import {invoke} from "@tauri-apps/api/core";
import {Pack} from "../../structures/packs/packs.ts";

export async function APIOpenPack() {
    return await invoke("open_packs");
}

export async function APIFetchPack(): Promise<Pack[]> {
    return await invoke<Pack[]>("fetch_packs");
}