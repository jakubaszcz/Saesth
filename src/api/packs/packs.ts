import {invoke} from "@tauri-apps/api/core";

export async function APIOpenPack() {
    return await invoke("open_packs");
}