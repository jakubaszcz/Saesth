import type {usePacks} from "../../../hooks/packs/usePacks.ts";
import type {useSounds} from "../../../hooks/sounds/useSounds.ts";
import {Props} from "./props.ts";
import {convertFileSrc} from "@tauri-apps/api/core";
import {Check, Headphones} from "lucide-react";

type CardProps = Props & {
    soundManager: ReturnType<typeof useSounds>;
    packsManager: ReturnType<typeof usePacks>;
};

export const Card = ( {
                         soundManager,
                          packsManager,
                         name,
                         id,
                         description,
                         icon
                     }: CardProps) => {
    const { selectPack, deselectPack, selectedPack } = packsManager;
    const isSelected = !packsManager.tempPack && selectedPack === id;
    const { fetchSound } = soundManager;

    async function select(id: string) {
        if (isSelected) {
            await deselectPack();
        } else {
            await selectPack(id);
        }
        await fetchSound();
    }

    return (
        <article className={`flex h-full flex-col gap-5 rounded-3xl border p-5 transition-colors duration-300 motion-reduce:transition-none ${isSelected ? "border-primary-400/60 bg-primary-800/80" : "border-primary-700/50 bg-primary-800/30 hover:border-primary-600 hover:bg-primary-800/60"}`}>
            <div className="flex items-center justify-between gap-3">
                <img src={convertFileSrc(icon)} alt="" className="h-14 w-14 rounded-2xl bg-primary-900 object-cover" />
                {isSelected && <span className="inline-flex items-center gap-1.5 rounded-full bg-primary-700/60 px-3 py-1 text-xs font-semibold text-primary-100"><Check size={13} aria-hidden="true" />Selected</span>}
            </div>
            <div className="flex flex-1 flex-col gap-2 min-w-0">
                <h3 className="break-words font-secondary text-xl font-medium text-primary-100">
                    {name}
                </h3>

                <p className="text-sm leading-6 text-primary-200 line-clamp-3">
                    {description}
                </p>

            </div>
                <button
                    type="button"
                    aria-pressed={isSelected}
                    aria-label={`${isSelected ? "Unselect" : "Select"} ${name}`}
                    onClick={() => select(id)}
                    className="flex w-full items-center justify-center gap-2 rounded-2xl border border-primary-600/40 bg-primary-900/40 px-4 py-2.5 text-sm font-semibold text-primary-100 transition-colors duration-300 hover:bg-primary-700/60 focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-primary-300 motion-reduce:transition-none"
                >
                    <Headphones size={16} aria-hidden="true" />
                    {isSelected ? "Unselect" : "Select"}
                </button>
        </article>
    );
};
