import { invoke } from "@tauri-apps/api/core";
import {useEffect, useState} from "react";

export function DrawSettings() {

    const [hide, setHide] = useState<string>("true");

    const set_settings = async (id: String, value: String) => {
        await invoke("set_settings", { id, value });
    }

    const set_hide = async (id: String) => {
        const value = hide === "true" ? "false" : "true";

        setHide(value);

        await set_settings(id, value);
    }

    useEffect(() => {
        const get_settings = async (id: String) => {
            const value = await invoke("get_settings", { id });
            // @ts-ignore
            setHide(value)
        }
        get_settings("hide");
    })

    return (
        <div className="max-w-4xl w-full grid grid-cols-1 sm:grid-cols-2 gap-8">
            <button onClick={() => set_hide("hide")}>Test
            </button>
            <p>{hide === "true" ? "true" : "False"}</p>
        </div>
    )
}