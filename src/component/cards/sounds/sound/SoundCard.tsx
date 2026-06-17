import {getSoundIcon} from "../../../../sounds/SoundsIcon.tsx";
import {Pause, Play, Sparkle} from "lucide-react";
import {Props} from "./props.ts";
import {getSoundEffectIcon} from "../../../../sounds/EffectsIcon.tsx";

export function SoundCard({
    sound,
    onToggleSound,
    onToggleSoundEffect,
    onChangeVolume,
    }: Props) {
    return (
        <div>
            <div>
                <div className="flex items-center justify-between">
                    <div className="flex items-center gap-2">
                        <div className="text-primary-500">
                            {getSoundIcon(sound.sound_id)}
                        </div>

                        <h3 className="text-primary-200 font-secondary">
                            {sound.sound_id}
                        </h3>
                    </div>

                    {
                        sound.effects && sound.effects.length > 0 && (
                            <button className="text-primary-200 hover:text-primary-100 hover:scale-110 duration-300 transition-all">
                                <Sparkle size={20} />
                            </button>
                        )
                    }
                </div>
            </div>
            <div className="flex items-center gap-4 mt-2">
                <button className="text-primary-200 hover:text-primary-300 hover:scale-110 duration-300 transition-all flex-shrink-0"
                    onClick={() => onToggleSound(sound.sound_id)}>
                    {sound.play ? (
                        <Pause size={20} />
                    ) : (
                        <Play size={20} />
                    )}
                </button>

                <div className="grow">
                    <div className="flex justify-between items-center">
                        <span className="text-primary-300 text-sm font-medium">
                            Volume
                        </span>

                        <div className="flex items-center gap-1 border border-primary-700 rounded-md px-2 py-1">
                            <input
                                type="number"
                                min={0}
                                max={100}
                                value={Math.round(sound.volume * 100)}
                                onChange={(e) => {
                                    const value = Math.max(
                                        0,
                                        Math.min(100, Number(e.target.value))
                                    );

                                    onChangeVolume(sound.sound_id, value / 100);
                                }}
                                className="
            w-12
            bg-transparent
            text-primary-200
            text-sm
            font-semibold
            text-right
            outline-none
        "
                            />

                            <span className="text-primary-200 text-sm font-semibold">
                                %
                            </span>
                        </div>
                    </div>

                    <input
                        className="
                            w-full
                            h-1.5
                            bg-primary-900
                            rounded-lg
                            appearance-none
                            cursor-pointer
                            accent-primary-500
                            hover:accent-primary-400
                            transition-all
                            [&::-webkit-slider-thumb]:appearance-none
                            [&::-webkit-slider-thumb]:w-3
                            [&::-webkit-slider-thumb]:h-3
                            [&::-webkit-slider-thumb]:rounded-full
                            [&::-webkit-slider-thumb]:bg-primary-100
                            [&::-webkit-slider-thumb]:shadow-[0_0_10px_rgba(var(--color-primary-500),0.5)]
                            [&::-moz-range-thumb]:border-none
                            [&::-moz-range-thumb]:w-3
                            [&::-moz-range-thumb]:h-3
                            [&::-moz-range-thumb]:rounded-full
                            [&::-moz-range-thumb]:bg-primary-100
                        "
                        type="range"
                        min={0}
                        max={100}
                        step={1}
                        value={sound.volume * 100}
                        onChange={(e) =>
                            onChangeVolume(
                                sound.sound_id,
                                parseFloat(e.target.value) / 100
                            )
                        }
                    />
                </div>
            </div>
            <div>
                <h1 className="mt-2 font-secondary font-bold text-primary-200">
                    Effects
                </h1>

                {sound.effects.map((effect) => (
                    <div
                        key={effect.effect_id}
                        className="flex items-center justify-between mt-3"
                    >
                        <div className="flex items-center gap-3 text-primary-500">
                            {getSoundEffectIcon(sound.sound_id, effect.effect_id)}

                            <p className="text-primary-100 font-primary">
                                {effect.effect_id}
                            </p>
                        </div>

                        <button
                            onClick={() =>
                                onToggleSoundEffect(
                                    sound.sound_id,
                                    effect.effect_id
                                )
                            }
                            className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors duration-300 ${
                                effect.active
                                    ? "bg-primary-500"
                                    : "bg-primary-900"
                            }`}
                        >
                <span
                    className={`inline-block h-4 w-4 transform rounded-full bg-primary-100 transition-transform duration-300 ${
                        effect.active
                            ? "translate-x-6"
                            : "translate-x-1"
                    }`}
                />
                        </button>
                    </div>
                ))}
            </div>
        </div>
    )
}