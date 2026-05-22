import {useState} from "react";
import {State} from "./state.ts";

export function useCard() {

    const [state, setState] = useState<State>(State.Sound)

    function changeState(state: State) {
        try {
            setState(state)
        } catch (e) {
            console.error("Failed to change state : ", e)
        }
    }

    return {
        state,
        changeState
    }
}