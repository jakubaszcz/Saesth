import {useSounds} from "../../hooks/sounds/useSounds.ts";
import {Card} from "../../component/cards/sounds/Card.tsx";

export function ContainerSounds() {
    const {
        sounds,
        toggleSound,
        volumeSound,
        toggleSoundEffect
    } = useSounds()

    return (
        <div className="flex flex-col gap-2">
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
    )
}