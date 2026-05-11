export interface Sound {
    sound_id: string;
    play: boolean;
    volume: number;
    effects: Effect[];
}

export interface Effect {
    effect_id: string;
    active: boolean;
}