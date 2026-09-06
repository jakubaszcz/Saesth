import {invoke} from "@tauri-apps/api/core";
import {Pack} from "../../structures/packs/packs.ts";

export async function APIOpenPack() {
    return await invoke("open_packs");
}

export async function APIFetchPack(): Promise<Pack[]> {
    return await invoke<Pack[]>("fetch_packs");
}

export async function APISelectPack(id: String) {
    return await invoke("select_pack", {id})
}

export async function APIHasActivePack() {
    return await invoke<boolean>("has_active_pack");
}