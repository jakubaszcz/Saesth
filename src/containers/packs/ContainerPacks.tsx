import type {usePacks} from "../../hooks/packs/usePacks.ts";
import type {useSounds} from "../../hooks/sounds/useSounds.ts";
import {Card} from "../../component/cards/packs/Card.tsx";
import {FolderOpen, PackagePlus} from "lucide-react";

export function ContainerPacks({soundsManager, packsManager}: {soundsManager: ReturnType<typeof useSounds>; packsManager: ReturnType<typeof usePacks>}) {
    const {packs, openPack} = packsManager;


    return (
        <div className="h-full w-full flex flex-col p-(--padding-md)">
            <div className="flex-1 grid grid-cols-1 md:grid-cols-2 xl:grid-cols-4 gap-2 overflow-y-auto content-start">
                {packs.map((pack) => (
                    <Card packsManager={packsManager} soundManager={soundsManager} key={pack.id} id={pack.id}
                          description={pack.description} icon={pack.icon} name={pack.name}/>
                ))}
                { packs.length === 0 && (
                    <section
                        aria-labelledby="empty-packs-title"
                        className="col-span-full mx-auto my-6 w-full max-w-xl rounded-2xl border border-primary-700 bg-primary-800 p-5 sm:p-8"
                    >
                        <div className="mb-5 flex h-12 w-12 items-center justify-center rounded-xl bg-primary-900 text-primary-300">
                            <PackagePlus size={26} aria-hidden="true" />
                        </div>
                        <h2 id="empty-packs-title" className="text-xl font-bold text-primary-100">
                            Add your first Saesth pack!
                        </h2>
                        <p className="mt-2 text-sm text-primary-200">
                            There's no pack in your system, but I'll show you how to put one.
                        </p>

                        <ol className="my-6 list-decimal space-y-4 pl-5 text-sm text-primary-200 marker:font-bold marker:text-primary-300">
                            <li className="pl-2">
                                <span className="font-semibold text-primary-100">Open packs.</span>
                                <p className="mt-1">Click on the open packs button.</p>
                            </li>
                            <li className="pl-2">
                                <span className="font-semibold text-primary-100">Put your pack into the folder.</span>
                                <p className="mt-1">Add your zip pack to the folder. Don't unzip it.</p>
                            </li>
                            <li className="pl-2">
                                <span className="font-semibold text-primary-100">Enjoy.</span>
                                <p className="mt-1">Restart Saesth then, select the pack you just added to enjoy its music.</p>
                            </li>
                        </ol>

                        <button
                            type="button"
                            onClick={openPack}
                            className="flex w-full items-center justify-center gap-2 rounded-xl bg-primary-600 px-4 py-3 text-sm font-semibold text-primary-50 transition-colors hover:bg-primary-500 focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-primary-300"
                        >
                            <FolderOpen size={18} className="shrink-0" aria-hidden="true" />
                            Open packs
                        </button>
                    </section>
                )}
            </div>

            {packs.length > 0 && <footer className="w-full p-(--padding-md) flex mt-auto">
                <button
                    onClick={openPack}
                    className="ml-auto text-primary-500 bg-primary-800 p-2 w-30 rounded-xl transition duration-300 hover:text-primary-400"
                >
                    Open
                </button>
            </footer>}
        </div>
    );
}
