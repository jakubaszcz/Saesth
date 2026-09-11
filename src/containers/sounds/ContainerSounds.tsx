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
        <div className="page-shell">
            <header>
                <p className="page-eyebrow">Your atmosphere</p>
                <h1 className="page-title">Let the world soften.</h1>
                <p className="page-description">Blend your favourite sounds and settle into your own rhythm.</p>
            </header>
            {sounds.length === 0 && <div className="quiet-panel text-sm text-primary-200">No sounds in this pack yet. Choose another pack from your library.</div>}
            <div className="grid grid-cols-1 gap-4 lg:grid-cols-2 xl:grid-cols-3">
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
        </div>
    )
}