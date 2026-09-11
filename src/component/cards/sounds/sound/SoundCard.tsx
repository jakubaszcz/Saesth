import {CloudRain, Pause, Play, Sparkle, X, Zap, Volleyball, WavesIcon, FlameIcon, Triangle, Bird} from "lucide-react";
import {Props} from "./props.ts";
import {ReactNode, useState} from "react";

const SOUND_METADATA: Record<string, { title: string, icon: ReactNode}> = {
    "sound_rain": {
        title: "Rain",
        icon: <CloudRain/>
    },
    "sound_beach": {
        title: "Beach",
        icon: <Volleyball/>
    },
    "sound_waterfall": {
        title: "Waterfall",
        icon: <WavesIcon/>
    },
    "sound_fire": {
        title: "Fire",
        icon: <FlameIcon/>
    }
};

const SOUND_EFFECT_METADATA: Record<string, { title: string, icon: ReactNode }> = {
    "sound_effect_thunder": {
        title: "Thunder",
        icon: <Zap/>
    },
    "sound_effect_triangle": {
        title: "Triangle",
        icon: <Triangle/>
    },
    "sound_effect_seagull": {
        title: "Seagull",
        icon: <Bird/>
    }
};

export function SoundCard({
    sound,
    onToggleSound,
    onToggleSoundEffect,
    onChangeVolume,
    }: Props) {

    const [isEffect, setIsEffect] = useState(false);

    function toggleEffect() {
        setIsEffect(!isEffect);
    }

    return (
        <div>
            <div>
                <div className="flex items-center justify-between">
                    <div className="flex items-center gap-2">
                            <div className="flex h-11 w-11 items-center justify-center rounded-2xl bg-primary-900/60 text-primary-300">
                            {SOUND_METADATA[sound.sound_id]?.icon || <X size={20} />}
                        </div>

                        <h3 className="text-primary-100 font-secondary text-xl font-medium">
                            {SOUND_METADATA[sound.sound_id]?.title || "Unknown"}
                        </h3>
                    </div>

                    {
                        sound.effects && sound.effects.length > 0 && (
                            <button onClick={toggleEffect} aria-label="Sound effects" aria-expanded={isEffect} className="quiet-icon-button">
                                <Sparkle size={20} />
                            </button>
                        )
                    }
                </div>
            </div>
            <div className="flex items-center gap-4 mt-6">
                <button aria-label={`${sound.play ? "Pause" : "Play"} ${SOUND_METADATA[sound.sound_id]?.title || sound.sound_id}`} aria-pressed={sound.play} className="quiet-icon-button border border-primary-600/40 bg-primary-700/40"
                    onClick={() => onToggleSound(sound.sound_id)}>
                    {sound.play ? (
                        <Pause size={20} />
                    ) : (
                        <Play size={20} />
                    )}
                </button>

                <div className="grow">
                    <div className="flex justify-between items-center">
                        <span className="text-primary-300 text-sm font-medium font-primary">
                            Volume
                        </span>

                        <div className="volume-value">
                            <input
                                aria-label={`${SOUND_METADATA[sound.sound_id]?.title || sound.sound_id} volume percentage`}
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
                        aria-label={`${SOUND_METADATA[sound.sound_id]?.title || sound.sound_id} volume`}
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
            <div className={`grid transition-all duration-300 ease-in-out ${
                isEffect && sound.effects && sound.effects.length > 0
                    ? "grid-rows-[1fr] opacity-100 mt-2"
                    : "grid-rows-[0fr] opacity-0"
            }`}>
                <div className="overflow-hidden" inert={!isEffect}>
                    {
                        sound.effects && sound.effects.length > 0 && (
                            <div className="mt-3 p-4 border border-primary-700/50 rounded-2xl bg-primary-900/40">
                                <h4 className="text-sm font-semibold text-primary-200">
                                    Effects
                                </h4>

                                {sound.effects.map((effect) => (
                                    <div
                                        key={effect.effect_id}
                                        className="flex items-center justify-between mt-3"
                                    >
                                        <div className="flex items-center gap-3 text-primary-300">
                                            {SOUND_EFFECT_METADATA[effect.effect_id]?.icon || <Sparkle size={20} />}

                                            <p className="text-primary-100 font-primary">
                                                {SOUND_EFFECT_METADATA[effect.effect_id]?.title || "Unknown"}
                                            </p>
                                        </div>

                                        <button
                                            role="switch"
                                            aria-checked={effect.active}
                                            aria-label={SOUND_EFFECT_METADATA[effect.effect_id]?.title || effect.effect_id}
                                            onClick={() =>
                                                onToggleSoundEffect(
                                                    sound.sound_id,
                                                    effect.effect_id
                                                )
                                            }
                                            className={`cursor-pointer relative inline-flex h-6 w-11 items-center rounded-full transition-colors duration-300 ${
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
                        )
                    }
                </div>
            </div>
        </div>
    )
}