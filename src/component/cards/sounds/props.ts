import {Sound} from "../../../interfaces/sounds/interface_sounds.ts";

export interface Props {
    sound: Sound;
    onClick?: () => void;
    onOpen?: () => void;
    onChanged?: (volume: number) => void;
}