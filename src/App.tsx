import "./App.css";
import {Header} from "./component/Header.tsx";
import {DrawSettings} from "./pages/DrawSettings.tsx";
import {useNavigation} from "./hooks/navbar/useNavigation.ts";
import {ComponentNavigation} from "./component/navigation/ComponentNavigation.tsx";
import {Navigation} from "./structures/navigation/Navigation.ts";
import {ComponentSetup} from "./features/setup/ComponentSetup.tsx";
import {ContainerSounds} from "./containers/sounds/ContainerSounds.tsx";
import {ContainerPacks} from "./containers/packs/ContainerPacks.tsx";
import {useSounds} from "./hooks/sounds/useSounds.ts";
import {usePacks} from "./hooks/packs/usePacks.ts";

function App() {


    const soundsManager = useSounds();
    const packsManager = usePacks();

    const {
        navigation,
        changeNavigation
    } = useNavigation()

    const { selectedPack } = packsManager;
    const activeNavigation =
        !selectedPack &&
        (navigation === Navigation.Sounds || navigation === Navigation.Setup)
            ? Navigation.Pack
            : navigation;

    function RenderPage(soundManager: any, pack: string | null) {
        switch (activeNavigation) {
            case Navigation.Sounds:
                return pack
                    ? <ContainerSounds soundsManager={soundManager} />
                    : null;

            case Navigation.Setup:
                return pack
                    ? <ComponentSetup />
                    : null;

            case Navigation.Pack:
                return (
                    <ContainerPacks
                        soundsManager={soundManager}
                        packsManager={packsManager}
                    />
                );

            case Navigation.Settings:
                return <DrawSettings />;

            default:
                return null;
        }
    }

  return (
      <main className="app-shell h-screen w-screen flex flex-col">
          <div className="flex-none">
              <Header packsManager={packsManager}/>
          </div>

          <div className="flex flex-1 overflow-hidden">
              <aside className="h-full flex-none border-r border-primary-700/40">
                  <ComponentNavigation
                      navigation={activeNavigation}
                      packsManager={packsManager}
                      changeNavigation={changeNavigation}
                  />
              </aside>

              <div className="min-w-0 flex-1 overflow-y-auto">
                  {RenderPage(soundsManager, selectedPack)}
              </div>
          </div>
      </main>
  );
}

export default App;
