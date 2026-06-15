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
      <main className="background-color h-screen w-screen">
          <div>
              <Header/>
          </div>

          <div className="flex">
              <aside>
                  <ComponentNavigation
                      navigation={navigation}
                      changeNavigation={changeNavigation}
                  />
              </aside>

              <div>
                  {RenderPage()}
              </div>
          </div>
      </main>
  );
}

export default App;
