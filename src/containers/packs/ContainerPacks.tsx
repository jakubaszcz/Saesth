import { usePacks } from "../../hooks/packs/usePacks.ts";

export function ContainerPacks() {
    const { openPack } = usePacks();

    return (
        <button onClick={openPack}>Open</button>
    );
}