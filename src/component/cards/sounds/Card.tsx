import { Props } from "./props.ts"
import {useCard} from "./useCard.ts";
import {State} from "./state.ts";
import {SoundCard} from "./sound/SoundCard.tsx";
import {EffectCard} from "./effect/EffectCard.tsx";

export const Card = ({sound, onToggleSound, onChangeVolume}: Props) => {
    const {
        state,
        changeState,
    } = useCard()

    switch (state) {
        case State.Sound:
            return <div className="
rounded-lg
w-full
bg-white/5
backdrop-blur-md
border border-white/10
shadow-[0_10px_40px_rgba(0,0,0,0.18)]
p-5
flex flex-col gap-5
transition-all duration-300
hover:bg-white/[0.07]
hover:shadow-[0_14px_44px_rgba(0,0,0,0.24)]
">
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
            return <div className="
rounded-lg
bg-white/5
backdrop-blur-md
border border-white/10
shadow-[0_10px_40px_rgba(0,0,0,0.18)]
p-5
flex flex-col gap-5
transition-all duration-300
hover:bg-white/[0.07]
hover:shadow-[0_14px_44px_rgba(0,0,0,0.24)]
">
                <EffectCard
                    sound={sound}
                    onToggleSoundEffect={onToggleSound}
                    onSwitchState={changeState}
                />
            </div>;
    }
};