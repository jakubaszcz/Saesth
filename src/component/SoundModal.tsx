import { SoundFront } from "../interface/sound-data.ts";
import { X } from "lucide-react";

interface SoundCardProps {
    data: SoundFront;
    onClose: () => void;
    isClose: boolean;
    onToggleEffect: (id: string, effect_id: string) => void;
}

export const SoundModal = ({
                               data,
                               onClose,
                               onToggleEffect,
                           }: SoundCardProps) => {
    if (!data) return null;

    return (
        <div
            className="fixed inset-0 z-50 flex items-center justify-center bg-black/40 backdrop-blur-sm px-4"
            onClick={onClose}
        >
            <div
                onClick={(e) => e.stopPropagation()}
                className="
                    w-full max-w-md
                    rounded-lg
                    bg-white/5
                    backdrop-blur-md
                    border border-white/10
                    shadow-[0_10px_40px_rgba(0,0,0,0.18)]
                    p-5
                    flex flex-col gap-5
                    transition-all duration-300
                "
            >
                <div className="flex items-center justify-between">
                    <div>
                        <h2 className="text-[var(--primary-100)] text-xl font-semibold capitalize">
                            {data.data.id}
                        </h2>

                        <p className="mt-1 text-sm text-[var(--primary-100)]/70">
                            Sound effects settings
                        </p>
                    </div>

                    <button
                        onClick={onClose}
                        className="
                            flex items-center justify-center
                            w-9 h-9 rounded-xl
                            bg-white/10
                            border border-white/10
                            text-white/80
                            hover:bg-white/20
                            transition-all duration-200
                        "
                    >
                        <X size={20} />
                    </button>
                </div>

                <div className="flex flex-col gap-4">
                    {data.effects.map((effect) => {
                        const isEnabled = effect.active;

                        return (
                            <div
                                key={effect.id}
                                className="
                                    rounded-lg
                                    border border-white/10
                                    bg-white/5
                                    backdrop-blur-md
                                    shadow-[0_10px_40px_rgba(0,0,0,0.18)]
                                    transition-all duration-300
                                    hover:bg-white/[0.07]
                                    hover:shadow-[0_14px_44px_rgba(0,0,0,0.24)]
                                    p-5
                                "
                            >
                                <div className="flex items-center justify-between gap-4">
                                    <div className="min-w-0">
                                        <p className="font-inter text-base font-semibold text-[var(--primary-100)] capitalize">
                                            {effect.id}
                                        </p>
                                    </div>

                                    <button
                                        type="button"
                                        role="switch"
                                        aria-checked={isEnabled}
                                        aria-label={`Toggle ${effect.id}`}
                                        onClick={() =>
                                            onToggleEffect(data.data.id, effect.id)
                                        }
                                        className={`
                                            relative h-8 w-14 shrink-0 rounded-full
                                            transition-all duration-300
                                            focus:outline-none focus:ring-2 focus:ring-white/20
                                            ${
                                            isEnabled
                                                ? "bg-[var(--primary-300)]/80 shadow-[0_0_18px_rgba(255,255,255,0.08)]"
                                                : "bg-white/10"
                                        }
                                            cursor-pointer
                                        `}
                                    >
                                        <span
                                            className={`
                                                absolute top-1 h-6 w-6 rounded-full
                                                bg-[var(--primary-100)] shadow-md
                                                transition-all duration-300
                                                ${isEnabled ? "left-7" : "left-1"}
                                            `}
                                        />
                                    </button>
                                </div>
                            </div>
                        );
                    })}
                </div>
            </div>
        </div>
    );
};