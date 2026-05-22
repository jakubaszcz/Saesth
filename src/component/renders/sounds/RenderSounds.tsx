import {useSounds} from "../../../hooks/sounds/useSounds.ts";
import {Card} from "../../cards/sounds/Card.tsx";

export function RenderSounds() {
    const {
        sounds,
        toggleSound,
        volumeSound,
        toggleSoundEffect
    } = useSounds()

    return (
        <div>
            <div className="w-full h-full grid grid-cols-2 lg:grid-cols-4 gap-4 font-manrope">
                {sounds.map((data,) => (
                    <Card
                        key={data.sound_id}
                        sound={data}
                        onToggleSound={toggleSound}
                        onChangeVolume={volumeSound}
                        onEffect={toggleSoundEffect}
                    />
                ))}
            </div>
        </div>
    )
}