import {Sound} from "../../../../structures/sounds/sounds.ts";
import {State} from "../state.ts";

export interface Props {
    sound: Sound,
    onToggleSound: (id: string) => void,
    onChangeVolume: (id: string, volume: number) => void,
    onSwitchState?: (state: State) => void,
}