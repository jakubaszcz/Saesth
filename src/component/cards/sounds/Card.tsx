import { Props } from "./props.ts"
import {SoundCard} from "./sound/SoundCard.tsx";

export const Card = ({sound, onToggleSound, onChangeVolume, onEffect}: Props) => {

    return (
        <div className="p-5 rounded-lg bg-primary-800 hover:bg-primary-700 duration-300 transition-all">
            <SoundCard
                sound={sound}
                onToggleSound={onToggleSound}
                onChangeVolume={onChangeVolume}
                onToggleSoundEffect={onEffect}
            />
        </div>
    )
};