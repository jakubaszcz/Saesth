import {Card} from "../../component/cards/sounds/Card.tsx";
import { Sound } from "../../structures/sounds/sounds.ts";

export function ContainerSounds({soundsManager}: any) {

    const {
        sounds,
        toggleSound,
        volumeSound,
        toggleSoundEffect
    } = soundsManager

    return (
        <div className="flex flex-col gap-2 lg:grid lg:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4">
            {sounds.map((data: Sound,) => (
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