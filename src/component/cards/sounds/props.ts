import {Sound} from "../../../structures/sounds/sounds.ts";

export interface Props {
    sound: Sound;
    onToggleSound: (id: string) => void;
    onEffect: (sound_id: string, effect_id: string) => void;
    onChangeVolume: (id: string, volume: number) => void;
}