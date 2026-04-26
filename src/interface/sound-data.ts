export interface SoundData {
    id: string;
    play: boolean;
    path: string;
    volume: number;
}

export interface SoundFront {
    data: SoundData;
    effects: SoundEffect[];
}

export interface SoundEffect {
    id: string;
    active: boolean;
}