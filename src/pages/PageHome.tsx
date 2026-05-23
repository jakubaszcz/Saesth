import { ComponentSetup } from "../features/setup/ComponentSetup.tsx";
import {RenderSounds} from "../component/renders/sounds/RenderSounds.tsx";
import {ComponentNavigation} from "../component/navigation/ComponentNavigation.tsx";
import {useNavigation} from "../hooks/navbar/useNavigation.ts";
import {Navigation} from "../structures/navigation/Navigation.ts";

export function PageHome() {

    const {
        navigation,
        changeNavigation
    } = useNavigation()

    function RenderPage() {
        switch (navigation) {

            case Navigation.Sounds:
                return <RenderSounds />;

            case Navigation.Setup:
                return <ComponentSetup />;

            default:
                return null;
        }
    }

    return (
        <div className="flex h-screen w-full">
            <ComponentNavigation
                navigation={navigation}
                changeNavigation={changeNavigation}
            />

            <main className="w-full h-full overflow-y-auto px-4">
                {RenderPage()}
            </main>
        </div>
    )
}
