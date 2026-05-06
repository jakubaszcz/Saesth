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
    setup_global_toggle: boolean;
    setup_global_volume: number;
    setup_keyboard_toggle: boolean;
    setup_keyboard_volume: number;
    setup_mouse_toggle: boolean;
    setup_mouse_volume: number;
}

export enum SetupKeys {
    SETUP_GLOBAL_TOGGLE = "setup_global_toggle",
    SETUP_GLOBAL_VOLUME = "setup_global_volume",
    SETUP_KEYBOARD_TOGGLE = "setup_keyboard_toggle",
    SETUP_KEYBOARD_VOLUME = "setup_keyboard_volume",
    SETUP_MOUSE_TOGGLE = "setup_mouse_toggle",
    SETUP_MOUSE_VOLUME = "setup_mouse_volume",
}