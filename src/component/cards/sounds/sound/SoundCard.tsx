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
            <p>Name : { sound.sound_id}</p>
            <div>
                <div>
                    <div>
                        {getSoundIcon(sound.sound_id)}
                    </div>

                    <div>
                        <h3>
                            {sound.sound_id}
                        </h3>
                    </div>
                </div>

                <div>
                    <button
                        onClick={() => onToggleSound(sound.sound_id)}>
                        {sound.play ? (
                            <Pause size={20} />
                        ) : (
                            <Play size={20} />
                        )}
                    </button>
                    {
                        onSwitchState !== undefined
                            ? <div>

                                <button onClick={() => onSwitchState(State.Effect)}>
                                    <Sparkle size={20}/>
                                </button>

                            </div>
                            : null
                    }
                </div>
            </div>
            <div>
                <div>
                    <span>Volume</span>
                    <span>{Math.round(sound.volume * 100)}%</span>
                </div>

                <input
                    type="range"
                    min={0}
                    max={100}
                    step={1}
                    value={sound.volume * 100}
                    onChange={(e) =>
                        onChangeVolume(sound.sound_id, parseFloat(e.target.value) / 100)
                    }/>
            </div>
        </div>
    )
}