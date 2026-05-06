export interface Structures {
    id: string;
    play: boolean;
    path: string;
    volume: number;
}

export interface SoundFront {
    data: Structures;
    effects: SoundEffect[];
}

export interface SoundEffect {
    id: string;
    active: boolean;
}

export interface Setup {
    toggle: boolean;
    volume: number;
}