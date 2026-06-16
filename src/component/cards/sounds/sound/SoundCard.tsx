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

                <div className="p-2">
                    <button className="text-primary-200 hover:text-primary-100 hover:scale-110 duration-300 transition-all"
                        onClick={() => onToggleSound(sound.sound_id)}>
                        {sound.play ? (
                            <Pause size={20} />
                        ) : (
                            <Play size={20} />
                        )}
                    </button>
                </div>
            </div>
            <div>
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
            h-2
            cursor-pointer
            accent-primary-500
            rounded-full
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
    )
}