import { ComponentSetup } from "../features/setup/ComponentSetup.tsx";
import {RenderSounds} from "../component/renders/sounds/RenderSounds.tsx";
import {HomeNavBar} from "../component/navbar/home/HomeNavBar.tsx";
import {useHomeNavBar} from "../hooks/navbar/home/useHomeNavBar.ts";
import {HomeNavigation} from "../structures/navbar/home/homeNavBar.ts";

export function PageHome() {

    const {
        navigation,
        changeNavigation
    } = useHomeNavBar()

    function RenderPage() {
        switch (navigation) {

            case HomeNavigation.Sounds:
                return <RenderSounds />;

            case HomeNavigation.Setup:
                return <ComponentSetup />;

            default:
                return null;
        }
    }

    return (
        <div className="flex h-screen w-full">
            <HomeNavBar
                navigation={navigation}
                changeNavigation={changeNavigation}
            />

            <main className="flex-1 w-full h-full overflow-y-auto">
                {RenderPage()}
            </main>
        </div>
    )
}
