import type {usePacks} from "../../hooks/packs/usePacks.ts";
import type {useSounds} from "../../hooks/sounds/useSounds.ts";
import {Card} from "../../component/cards/packs/Card.tsx";
import {ArrowUpRight, FolderOpen, Headphones, Moon, PackagePlus, Sparkles} from "lucide-react";

const steps = [
    {icon: FolderOpen, title: "Open your packs folder", description: "Use the button below to find the right place for your packs."},
    {icon: PackagePlus, title: "Drop in a little atmosphere", description: "Copy a Saesth pack .zip into this folder. Keep it zipped."},
    {icon: Headphones, title: "Make yourself comfortable", description: "Restart Saesth, then select your new pack. Your sounds are ready."},
];

export function ContainerPacks({soundsManager, packsManager}: {soundsManager: ReturnType<typeof useSounds>; packsManager: ReturnType<typeof usePacks>}) {
    const {packs, openPack} = packsManager;
    const isEmpty = packs.length === 0;
    const folderButtonClass = "inline-flex items-center justify-center gap-2 rounded-2xl border border-primary-500/30 bg-primary-700/50 px-5 py-3 text-sm font-semibold text-primary-100 transition-colors duration-300 hover:bg-primary-700 focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-primary-300 motion-reduce:transition-none";

    return (
        <div className="mx-auto flex min-h-full w-full max-w-6xl flex-col gap-8 px-4 py-6 font-primary sm:px-8 sm:py-10">
            <header className="flex flex-wrap items-center justify-between gap-5">
                <div>
                    <p className="mb-2 text-xs font-semibold uppercase tracking-[0.2em] text-primary-300">Your sound library</p>
                    <h1 className="font-secondary text-3xl font-medium tracking-tight text-primary-50">A space to settle into.</h1>
                    <p className="mt-2 text-sm leading-relaxed text-primary-200">Little collections of sound, for moments that are yours.</p>
                </div>
                {!isEmpty && (
                    <button type="button" onClick={openPack} className={folderButtonClass}>
                        <FolderOpen size={17} aria-hidden="true" />
                        Open packs folder
                    </button>
                )}
            </header>

            {isEmpty ? (
                <section aria-labelledby="empty-packs-title" className="relative isolate overflow-hidden rounded-3xl border border-primary-700/60 bg-primary-800/35 p-6 sm:p-10">
                    <div aria-hidden="true" className="pointer-events-none absolute inset-0 -z-10 bg-[radial-gradient(ellipse_at_top_right,rgba(152,180,208,0.10),transparent_65%)]" />
                    <div className="max-w-lg">
                        <div className="mb-7 flex h-16 w-16 items-center justify-center rounded-3xl border border-primary-600/30 bg-primary-900/60 text-primary-200">
                            <Moon size={28} strokeWidth={1.4} aria-hidden="true" />
                        </div>
                        <p className="mb-2 text-xs font-semibold uppercase tracking-[0.18em] text-primary-300">A quiet beginning</p>
                        <h2 id="empty-packs-title" className="font-secondary text-2xl font-medium text-primary-50 sm:text-3xl">Your first pack awaits.</h2>
                        <p className="mt-3 text-sm leading-7 text-primary-200">It's a little quiet here. Add your first Saesth pack and find a sound that feels like home.</p>
                    </div>

                    <ol className="my-8 grid gap-3 lg:grid-cols-3">
                        {steps.map(({icon: Icon, title, description}, index) => (
                            <li key={title} className="rounded-2xl border border-primary-700/40 bg-primary-900/40 p-5">
                                <div className="mb-5 flex items-center justify-between text-primary-300">
                                    <Icon size={20} strokeWidth={1.5} aria-hidden="true" />
                                    <span className="font-secondary text-xs tracking-widest" aria-hidden="true">0{index + 1}</span>
                                </div>
                                <h3 className="text-sm font-bold text-primary-100">{title}</h3>
                                <p className="mt-2 text-sm leading-6 text-primary-200">{description}</p>
                            </li>
                        ))}
                    </ol>

                    <div className="flex flex-wrap items-center gap-4">
                        <button type="button" onClick={openPack} className={folderButtonClass}>
                            <FolderOpen size={17} aria-hidden="true" />
                            Open packs folder
                            <ArrowUpRight size={15} className="text-primary-300" aria-hidden="true" />
                        </button>
                        <span className="text-xs text-primary-300">One pack is all you need to get started.</span>
                    </div>
                </section>
            ) : (
                <section aria-labelledby="packs-library-title">
                    <div className="mb-4 flex items-center gap-3">
                        <h2 id="packs-library-title" className="text-sm font-semibold text-primary-200">Your packs</h2>
                        <span className="rounded-full bg-primary-800 px-2.5 py-0.5 text-xs text-primary-200">{packs.length}</span>
                        <div className="h-px flex-1 bg-primary-700/50" />
                    </div>
                    <div className="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-3">
                        {packs.map((pack) => (
                            <Card packsManager={packsManager} soundManager={soundsManager} key={pack.id} id={pack.id}
                                  description={pack.description} icon={pack.icon} name={pack.name}/>
                        ))}
                    </div>
                    <p className="mt-5 text-xs leading-6 text-primary-300">Adding a new pack? Place its .zip in the packs folder, then restart Saesth.</p>
                </section>
            )}

            <footer className="mt-auto flex items-center justify-center gap-2 pt-4 text-xs text-primary-300">
                <Sparkles size={14} strokeWidth={1.5} aria-hidden="true" />
                Find your atmosphere. Take your time.
            </footer>
        </div>
    );
}
