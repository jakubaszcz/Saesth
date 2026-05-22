import {Props} from "./props.ts";
import {X} from "lucide-react";
import {State} from "../state.ts";


export function EffectCard({ sound, onToggleSoundEffect, onSwitchState}: Props) {
    return (
        <div className="flex flex-col">

            <div className="flex flex-col items-end-safe">

                <button onClick={() => onSwitchState(State.Sound)}>

                    <X/>

                </button>

            </div>

            { sound.effects.map((effect) =>

                <div key={effect.effect_id}>

                    <div className="flex flex-row gap-4">

                        <p className="flex items-start">{effect.effect_id}</p>

                        <button className="flex items-end-safe" onClick={() => onToggleSoundEffect(sound.sound_id, effect.effect_id)}>

                            { effect.active ? "ON" : "OFF" }

                        </button>

                    </div>

                </div>

            ) }
        </div>
    )
}