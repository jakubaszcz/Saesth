import "./App.css";
import {Header} from "./component/Header.tsx";
import {DrawSettings} from "./pages/DrawSettings.tsx";
import {useNavigation} from "./hooks/navbar/useNavigation.ts";
import {ComponentNavigation} from "./component/navigation/ComponentNavigation.tsx";
import {Navigation} from "./structures/navigation/Navigation.ts";
import {ComponentSetup} from "./features/setup/ComponentSetup.tsx";
import {ContainerSounds} from "./containers/sounds/ContainerSounds.tsx";
function App() {

    const {
        navigation,
        changeNavigation
    } = useNavigation()

    function RenderPage() {
        switch (navigation) {

            case Navigation.Sounds:
                return <ContainerSounds />;

            case Navigation.Setup:
                return <ComponentSetup />;

            case Navigation.Settings:
                return <DrawSettings />;

            default:
                return null;
        }
    }

  return (
      <main
          className="
    h-screen
    w-screen
    flex flex-col
    overflow-hidden
    bg-radial-[at_50%_20%]
    from-(--primary-400)
    via-(--primary-700)
    to-(--primary-950)
    to-90%">
          <div className="shrink-0">
              <Header/>
          </div>

          <div className="flex flex-1 overflow-hidden">
              <aside className="shrink-0">
                  <ComponentNavigation
                      navigation={navigation}
                      changeNavigation={changeNavigation}
                  />
              </aside>

              <div className="flex-1 overflow-y-auto hide-scrollbar px-2">
                  {RenderPage()}
              </div>
          </div>
      </main>
  );
}

export default App;
