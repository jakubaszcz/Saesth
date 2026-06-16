import {getSoundIcon} from "../../../../sounds/SoundsIcon.tsx";
import {Pause, Play, Sparkle} from "lucide-react";
import {Props} from "./props.ts";
import {State} from "../state.ts";

export function SoundCard({
    sound,
    onToggleSound,
    onChangeVolume,
    onSwitchState
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

                    {onSwitchState !== undefined && (
                        <button onClick={() => onSwitchState(State.Effect)} className="text-primary-200 hover:text-primary-100 hover:scale-110 duration-300 transition-all">
                            <Sparkle size={20} />
                        </button>
                    )}
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

                <div className="flex-grow">
                    <div className="flex justify-between items-center">
                        <span className="text-primary-300 text-sm font-medium">
                            Volume
                        </span>

                        <span className="text-primary-200 text-sm font-semibold">
                            {Math.round(sound.volume * 100)}%
                        </span>
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
        </div>
    )
}