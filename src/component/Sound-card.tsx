import {Structures, SoundEffect} from "../interface/structures.ts";
import {
    Play, Pause, Sparkle
} from "lucide-react";
import {getSoundIcon} from "../sounds/SoundsIcon.tsx";
import {getSoundEffectIcon} from "../sounds/EffectsIcon.tsx";

interface SoundCardProps {
    id: string;
    data: Structures;
    effect: SoundEffect[] | undefined;
    effects: SoundEffect[] | undefined;
    onClick: () => void;
    onOpen: () => void;
    onChanged?: (volume: number) => void;
}

export const SoundCard = ({
                              id,
                              data,
                              effect,
                              effects,
                              onClick,
                              onOpen,
                              onChanged
                          }: SoundCardProps) => {

    const hasAvailableEffects = effects && effects.length > 0;
    const hasActiveEffects = effect && effect.length > 0;


    return (
        <div
            className="
      rounded-lg
      bg-white/5
      backdrop-blur-md
      border border-white/10
      shadow-[0_10px_40px_rgba(0,0,0,0.18)]
      p-5
      flex flex-col gap-5
      transition-all duration-300
      hover:bg-white/[0.07]
      hover:shadow-[0_14px_44px_rgba(0,0,0,0.24)]
    "
        >
            <div className="flex items-start justify-between gap-4">
                <div className="flex items-center gap-3">
                    <div className="text-[var(--primary-100)]">
                        {getSoundIcon(id)}
                    </div>

                    <div className="flex flex-col">
                        <h3 className="text-[var(--primary-100)] font-semibold text-xl capitalize">
                            {id}
                        </h3>
                    </div>
                </div>

                <div className="flex items-center gap-2">

                    {hasAvailableEffects && (
                        <button
                            onClick={onOpen}
                            className="
            w-11 h-11
            rounded-xl
            flex items-center justify-center
            bg-white/10
            border border-white/10
            text-[var(--primary-100)]
            transition-all duration-300
            hover:bg-white/20
            hover:scale-105
            active:scale-95
        "
                        >
                            <Sparkle size={20}/>
                        </button>
                    )}

                    <button
                        onClick={onClick}
                        className="
          w-11 h-11
          rounded-xl
          flex items-center justify-center
          bg-white/10
          border border-white/10
          text-[var(--primary-100)]
          transition-all duration-300
          hover:bg-white/20
          hover:scale-105
          active:scale-95
        "
                    >
                        {data.play ? (
                            <Pause size={20} />
                        ) : (
                            <Play size={20} />
                        )}
                    </button>

                </div>
            </div>

            {!hasAvailableEffects ? (
                <div
                    className="
            rounded-lg
            bg-white/5
            border border-white/10
            backdrop-blur-md
            px-3 py-2
            text-center
            text-sm text-[var(--primary-100)]/70
        "
                >
                    There is no effect available for this sound.
                </div>

            ) : hasActiveEffects ? (

                <div
                    className="
            flex items-center gap-2
            rounded-lg
            bg-white/5
            border border-white/10
            backdrop-blur-md
            px-3 py-2
            shadow-[0_8px_24px_rgba(0,0,0,0.14)]
            overflow-hidden
        "
                >
                    {effect.slice(0, 10).map((item) => (
                        <div
                            key={item.id}
                            title={item.id}
                            className="
                    text-[var(--primary-100)]
                    shrink-0
                    flex items-center justify-center
                "
                        >
                            {getSoundEffectIcon(data.id, item.id, 20)}
                        </div>
                    ))}

                    {effect.length > 10 && (
                        <div
                            className="
                    text-[var(--primary-100)]
                    text-sm
                    px-2
                    py-0.5
                    rounded-md
                    bg-white/10
                    border border-white/10
                "
                        >
                            +{effect.length - 10}
                        </div>
                    )}
                </div>

            ) : (

                <div
                    className="
            rounded-lg
            bg-white/5
            border border-white/10
            backdrop-blur-md
            px-3 py-2
            text-center
            text-sm text-[var(--primary-100)]/80
        "
                >
                    No effect selected.
                </div>
            )}
            <div className="flex flex-col gap-2">
                <div className="flex items-center justify-between text-sm uppercase tracking-wide text-[var(--primary-100)]">
                    <span>Volume</span>
                    <span>{Math.round(data.volume * 100)}%</span>
                </div>

                <input
                    type="range"
                    min={0}
                    max={100}
                    step={1}
                    value={data.volume * 100}
                    onChange={(e) =>
                        onChanged?.(parseFloat(e.target.value) / 100)
                    }
                    className="
    w-full
    h-2
    appearance-none
    rounded-full
    bg-white/10
    cursor-pointer

    [&::-webkit-slider-runnable-track]:h-2
    [&::-webkit-slider-runnable-track]:rounded-full
    [&::-webkit-slider-runnable-track]:bg-white/10

    [&::-webkit-slider-thumb]:appearance-none
    [&::-webkit-slider-thumb]:mt-[-4px]
    [&::-webkit-slider-thumb]:h-4
    [&::-webkit-slider-thumb]:w-4
    [&::-webkit-slider-thumb]:rounded-full
    [&::-webkit-slider-thumb]:bg-white/90
    [&::-webkit-slider-thumb]:shadow-md
    [&::-webkit-slider-thumb]:transition-all
    [&::-webkit-slider-thumb]:duration-200
    hover:[&::-webkit-slider-thumb]:scale-110

    [&::-moz-range-track]:h-2
    [&::-moz-range-track]:rounded-full
    [&::-moz-range-track]:bg-white/10

    [&::-moz-range-thumb]:h-4
    [&::-moz-range-thumb]:w-4
    [&::-moz-range-thumb]:rounded-full
    [&::-moz-range-thumb]:border-0
    [&::-moz-range-thumb]:bg-[var(--primary-100)]
  "
                />
            </div>
        </div>
    );
};