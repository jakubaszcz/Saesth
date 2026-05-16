import {Sound} from "../../../interfaces/sounds/interface_sounds.ts";

export interface Props {
    sound: Sound;
    onClick: (id: string) => void;
    onChange: (id: string, volume: number) => void;
    onOpen?: () => void;
}