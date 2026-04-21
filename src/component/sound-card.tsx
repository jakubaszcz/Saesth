import { SoundData } from "../interface/sound-data.ts";

interface SoundCardProps {
    id: string;
    data: SoundData;
    onClick: () => void;
    onChanged?: (volume: number) => void;
}

export const SoundCard = ({id,  data, onClick, onChanged} : SoundCardProps) => {
    return (
        <div>
            <h3>{id}</h3>
            <p>Path : {data.path}</p>
            <p>Play : {data.play ? "Yes" : "No"}</p>
            <button onClick={onClick}>Toggle Play</button>
            <p>Change volume</p>
            <input
            type="range"
            min={0}
            max={100}
            step={1}
            value={data.volume * 100}
            onChange={(e) => onChanged?.(parseFloat(e.target.value) / 100)}
            />
        </div>
    )
}