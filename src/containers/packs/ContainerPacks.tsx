import { usePacks } from "../../hooks/packs/usePacks.ts";

export function ContainerPacks() {
    const { packs, openPack } = usePacks();

    return (
        <div>
            <button onClick={openPack}>Open</button>
            <div>
                {
                    packs.map((pack,) => (
                        <p>{pack.name}</p>
                    ))
                }
            </div>
        </div>
    )
}