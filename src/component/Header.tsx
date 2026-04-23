import {getCurrentWindow} from "@tauri-apps/api/window";
import {BoltIcon, Minimize, X, House} from "lucide-react";
import {Pages} from "../pages/pages.ts";

type Props = {
  tab: Pages;
  setTab: React.Dispatch<React.SetStateAction<Pages>>;
};

export const Header = ({ tab, setTab }: Props) => {

  const appWindow = getCurrentWindow();

  const handleClose = async () => {
    console.log("close")

    try {
      await appWindow.close();
    } catch (error) {
      console.error("Failed to close window:", error);
    }
  };

  const handleMinimize = async () => {
    console.log("minimize")
    try {
      await appWindow.minimize();
    } catch (error) {
      console.error("Failed to minimize window:", error);
    }
  };

  return (
    <header data-tauri-drag-region className="
    w-full flex justify-between items-center h-14 px-8
    flex items-center justify-between">
      <div className="flex items-center gap-2">
        <div className="
        font-manrope
text-[var(--primary-500)]
text-2xl
tracking-wide
select-none
drop-shadow-sm
">
          Saesth
        </div>
      </div>

      <div className="flex gap-2 text-[var(--primary-500)]">
        { tab === Pages.HOME && (
            <button
                onClick={() => setTab(Pages.SETTINGS)}
                aria-label="Go to settings"
            >
              <BoltIcon size={20} className="transition-all duration-300 ease-out hover:scale-125 hover:text-[var(--primary-600)] cursor-pointer"/>
            </button>
        )}
        { tab === Pages.SETTINGS && (
            <button onClick={() => setTab(Pages.HOME)} aria-label="Go to home">
              <House size={20} className="transition-all duration-300 ease-out hover:scale-125 hover:text-[var(--primary-600)] cursor-pointer"/>
            </button>
        )}
        <button onClick={handleMinimize} aria-label="Minimize">
          <Minimize size={20} className="transition-all duration-300 ease-out hover:scale-125 hover:text-[var(--primary-600)] cursor-pointer"/>
        </button>
        <button onClick={handleClose} aria-label="Close"
        >
          <X size={20} className="transition-all duration-300 ease-out hover:scale-125 hover:text-[var(--primary-600)] cursor-pointer"/>
        </button>
      </div>
    </header>
  );
};
