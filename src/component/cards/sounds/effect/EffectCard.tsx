import {Props} from "./props.ts";
import {X} from "lucide-react";
import {State} from "../state.ts";


export function EffectCard({ sound, onToggleSoundEffect, onSwitchState}: Props) {
    return (
        <div>

            <div>

                <button onClick={() => onSwitchState(State.Sound)}>

                    <X/>

                </button>

            </div>

            { sound.effects.map((effect) =>

                <div key={effect.effect_id}>

                    <div>

                        <p>{effect.effect_id}</p>

                        <button onClick={() => onToggleSoundEffect(sound.sound_id, effect.effect_id)}>

                            { effect.active ? "ON" : "OFF" }

                        </button>

                    </div>

                </div>

            ) }
        </div>
    )
}