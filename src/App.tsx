import "./App.css";
import {Header} from "./component/Header.tsx";
import { DrawHome } from "./pages/DrawHome.tsx";
import {useState} from "react";
import {Pages} from "./pages/pages.ts";
import {DrawSettings} from "./pages/DrawSettings.tsx";
function App() {

  const [tab, setTab] = useState<Pages>(Pages.HOME);

  return (
      <main
          className="
    h-screen
    w-full
    flex flex-col
    overflow-hidden
    bg-radial-[at_50%_20%]
    from-[var(--primary-400)]
    via-[var(--primary-700)]
    to-[var(--primary-950)]
    to-90%">
          <div className="shrink-0">
              <Header tab={tab} setTab={setTab} />
          </div>

          <div className="flex-1 min-h-0 overflow-y-auto hide-scrollbar p-6">
              {tab === Pages.HOME && <DrawHome />}
              {tab === Pages.SETTINGS && <DrawSettings />}
          </div>
      </main>
  );
}

export default App;
