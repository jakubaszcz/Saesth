import {usePacks} from "../../../hooks/packs/usePacks.ts";
import {Props} from "./props.ts";
import {convertFileSrc} from "@tauri-apps/api/core";

type CardProps = Props & {
    soundManager: any;
};

export const Card = ( {
                         soundManager,
                         name,
                         id,
                         description,
                         icon
                     }: CardProps) => {
    const { selectPack } = usePacks();
    const { fetchSound } = soundManager;

    async function select(id: string) {
        await selectPack(id);
        await fetchSound();
    }

    return (
        <div className="p-4 rounded-2xl bg-primary-800 hover:bg-primary-700 duration-300 transition-all flex flex-row items-center gap-4 group">
            <div className="flex flex-col gap-1 flex-1 min-w-0">
                <h1 className="text-lg font-bold text-primary-500 truncate">
                    {name}
                </h1>

                <p className="text-sm text-primary-200 line-clamp-2">
                    {description}
                </p>

                <button onClick={() => select(id)}>
                    Select
                </button>
            </div>

            <div className="relative overflow-hidden rounded-xl w-16 h-16 shrink-0 bg-primary-900">
                <img
                    className="object-cover w-full h-full transition-transform duration-500 group-hover:scale-110"
                    src={convertFileSrc(icon)}
                    alt={name}
                />
            </div>
        </div>
    );
};