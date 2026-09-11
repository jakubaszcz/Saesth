import {getCurrentWindow} from "@tauri-apps/api/window";
import {X, Minus, Maximize2, Minimize2, Package} from "lucide-react";
import {useEffect, useState} from "react";
import type {usePacks} from "../hooks/packs/usePacks.ts";

type HeaderProps = {
  packsManager: ReturnType<typeof usePacks>;
};

export const Header = ({ packsManager }: HeaderProps) => {

  const {packs, selectedPack} = packsManager;
  const currentPackName = selectedPack === null
    ? "No pack selected"
    : packs.find(pack => pack.id === selectedPack)?.name || selectedPack;

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
    <header data-tauri-drag-region className="flex h-18 shrink-0 justify-between items-center gap-3 border-b border-primary-700/40 px-4 sm:px-6">
      <div data-tauri-drag-region className="flex min-w-0 items-center gap-3 p-(--padding-md)">
        <h1 data-tauri-drag-region className="shrink-0 font-secondary text-primary-50 text-2xl font-medium tracking-tight">Saesth</h1>
        <div
          data-tauri-drag-region
          role="status"
          title={`Current pack: ${currentPackName}`}
          className="flex min-w-0 items-center gap-2 rounded-full border border-primary-700/50 bg-primary-800/40 px-3 py-1.5 text-xs text-primary-200"
        >
          <Package size={14} className="pointer-events-none shrink-0" aria-hidden="true" />
          <span data-tauri-drag-region className="hidden sm:inline shrink-0">Current pack:</span>
          <span data-tauri-drag-region className="truncate font-semibold text-primary-100">{currentPackName}</span>
        </div>
      </div>

      <div className="flex shrink-0 gap-1">
        <button onClick={handleMinimize} aria-label="Minimize" className="quiet-icon-button">
          <Minus size={20}/>
        </button>
        {!isMaximized && (
            <button onClick={handleMaximize} aria-label={isMaximized ? "Restore window" : "Maximize"} className="quiet-icon-button">
              <Maximize2 size={20}/>
            </button>
        )}
        {isMaximized && (
            <button onClick={handleMaximize} aria-label={isMaximized ? "Restore window" : "Maximize"} className="quiet-icon-button">
              <Minimize2 size={20}/>
            </button>
        )}
        <button onClick={handleClose} aria-label="Close" className="quiet-icon-button"
        >
          <X size={20}/>
        </button>
      </div>
    </header>
  );
};
