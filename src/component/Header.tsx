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
    <header data-tauri-drag-region className="flex justify-between items-center h-(--header-height) px-2">
      <div className="p-(--padding-md)">
        <div className="flex flex-col items-start">
          <h1 className="font-secondary text-(--color-primary-100) font-bold space-y-10 text-medium">Saesth</h1>
        </div>
      </div>

      <div className="flex gap-5">
        <button onClick={handleMinimize} aria-label="Minimize" className="text-(--color-primary-700)
        hover:text-primary-600 hover:scale-110 transition-all duration-300">
          <Minus size={20}/>
        </button>
        {!isMaximized && (
            <button onClick={handleMaximize} aria-label="Minimize" className="text-(--color-primary-700)
        hover:text-primary-600 hover:scale-110 transition-all duration-300">
              <Maximize2 size={20}/>
            </button>
        )}
        {isMaximized && (
            <button onClick={handleMaximize} aria-label="Minimize" className="text-(--color-primary-700)
        hover:text-primary-600 hover:scale-110 transition-all duration-300">
              <Minimize2 size={20}/>
            </button>
        )}
        <button onClick={handleClose} aria-label="Close" className="text-(--color-primary-700)
        hover:text-primary-600 hover:scale-110 transition-all duration-300"
        >
          <X size={20}/>
        </button>
      </div>
    </header>
  );
};
