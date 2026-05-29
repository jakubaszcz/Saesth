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
            return <div>
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