import { Props } from "./props.ts"
import {SoundCard} from "./sound/SoundCard.tsx";

export const Card = ({sound, onToggleSound, onChangeVolume, onEffect}: Props) => {

    return (
        <div className={`quiet-panel transition-colors duration-300 ${sound.play ? "border-primary-400/60 bg-primary-800/70" : "hover:border-primary-600"}`}>
            <SoundCard
                sound={sound}
                onToggleSound={onToggleSound}
                onChangeVolume={onChangeVolume}
                onToggleSoundEffect={onEffect}
            />
        </div>
    )
};