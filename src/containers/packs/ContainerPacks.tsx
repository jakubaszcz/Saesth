import { usePacks } from "../../hooks/packs/usePacks.ts";

export function ContainerPacks() {
    const { packs, openPack } = usePacks();

    return (
        <div className="h-full w-full flex flex-col">
            <div className="flex-1">
                {packs.map((pack) => (
                    <p key={pack.name}>{pack.name}</p>
                ))}
            </div>

            <footer className="w-full p-(--padding-md) flex mt-auto">
                <button
                    onClick={openPack}
                    className="ml-auto text-primary-500 bg-primary-800 p-2 w-30 rounded-xl transition duration-300 hover:text-primary-400"
                >
                    Open
                </button>
            </footer>
        </div>
    );
}