import { usePacks } from "../../hooks/packs/usePacks.ts";
import {Card} from "../../component/cards/packs/Card.tsx";

export function ContainerPacks() {
    const { packs, openPack } = usePacks();


    return (
        <div className="h-full w-full flex flex-col p-(--padding-md)">
            <div className="flex-1 grid grid-cols-1 md:grid-cols-2 xl:grid-cols-4 gap-2 overflow-y-auto content-start">
                {packs.map((pack) => (
                    <Card key={pack.name} name={pack.name} description={pack.description} icon={pack.icon}/>
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