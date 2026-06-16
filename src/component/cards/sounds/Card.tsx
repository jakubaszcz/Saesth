import { Props } from "./props.ts"
import {useCard} from "./useCard.ts";
import {State} from "./state.ts";
import {SoundCard} from "./sound/SoundCard.tsx";
import {EffectCard} from "./effect/EffectCard.tsx";

export const Card = ({sound, onToggleSound, onChangeVolume, onEffect}: Props) => {
    const {
        state,
        changeState,
    } = useCard()

    switch (state) {
        case State.Sound:
            return <div className="p-5 rounded-lg bg-primary-800 hover:bg-primary-700 duration-300 transition-all">
                <SoundCard
                    sound={sound}
                    onToggleSound={onToggleSound}
                    onChangeVolume={onChangeVolume}
                    onSwitchState={
                        sound.effects.length > 0
                            ? changeState
                            : undefined
                    }
                />
            </div>;

        case State.Effect:
            return <div>
                <EffectCard
                    sound={sound}
                    onToggleSoundEffect={onEffect}
                    onSwitchState={changeState}
                />
            </div>;
    }
};