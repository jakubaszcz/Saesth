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
    keyboard_toggle: boolean;
    keyboard_volume: number;
    mouse_toggle: boolean;
    mouse_volume: number;
}

export enum SetupUtilities {
    SETUP = "setup",
    KEYBOARD = "keyboard",
    MOUSE = "mouse",
}