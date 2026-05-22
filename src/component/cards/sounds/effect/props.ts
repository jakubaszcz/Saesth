import {Sound} from "../../../../structures/sounds/sounds.ts";
import {State} from "../state.ts";

export interface Props {
    sound: Sound,
    onToggleSoundEffect: (sound_id: string, effect_id: string) => void,
    onSwitchState: (state: State) => void,
}