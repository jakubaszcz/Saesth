import {getCurrentWindow} from "@tauri-apps/api/window";
import {X, Minus, Maximize2, Minimize2} from "lucide-react";
import {useEffect, useState} from "react";

export const Header = () => {

  const appWindow = getCurrentWindow();

  const [isMaximized, setIsMaximized] = useState(false);

  const handleClose = async () => {
    try {
      await appWindow.close();
    } catch (error) {
      console.error("Failed to close window:", error);
    }
  };

  const handleMinimize = async () => {
    try {
      await appWindow.minimize();
    } catch (error) {
      console.error("Failed to minimize window:", error);
    }
  };

  const handleMaximize = async () => {
    const maximized = await appWindow.isMaximized();

    if (maximized) {
      await appWindow.unmaximize();
    } else {
      await appWindow.maximize();
    }
  };

  useEffect(() => {
    const load = async () => {
      setIsMaximized(await appWindow.isMaximized());
    };

    load();

    const unlistenPromise = appWindow.onResized(async () => {
      setIsMaximized(await appWindow.isMaximized());
    });

    return () => {
      unlistenPromise.then(unlisten => unlisten());
    };
  }, []);

  return (
    <header data-tauri-drag-region>
      <div>
        <div>
          <h1>Saesth</h1>
        </div>
      </div>

      <div>
        <button onClick={handleMinimize} aria-label="Minimize">
          <Minus size={20}/>
        </button>
        {!isMaximized && (
            <button onClick={handleMaximize} aria-label="Minimize">
              <Maximize2 size={20}/>
            </button>
        )}
        {isMaximized && (
            <button onClick={handleMaximize} aria-label="Minimize">
              <Minimize2 size={20}/>
            </button>
        )}
        <button onClick={handleClose} aria-label="Close"
        >
          <X size={20}/>
        </button>
      </div>
    </header>
  );
};
