import {AudioLines, SlidersHorizontal, Headphones, Moon, Package} from "lucide-react";
import {Navigation} from "../../structures/navigation/Navigation.ts";
import {Props} from "./props.ts";
import type {usePacks} from "../../hooks/packs/usePacks.ts";

type NavigationProps = Props & {packsManager: ReturnType<typeof usePacks>};
const items = [
    {page: Navigation.Sounds, label: "Sounds", icon: AudioLines},
    {page: Navigation.Setup, label: "Setup", icon: Headphones},
    {page: Navigation.Pack, label: "Packs", icon: Package},
    {page: Navigation.Settings, label: "Settings", icon: SlidersHorizontal},
];

export function ComponentNavigation({navigation, changeNavigation, packsManager}: NavigationProps) {
    return (
        <nav aria-label="Main navigation" className="flex h-full w-18 flex-col gap-2 px-3 py-6 lg:w-48 lg:px-4">
            <p className="mb-4 hidden px-3 text-[10px] font-semibold uppercase tracking-[0.2em] text-primary-300 lg:block">Your quiet corner</p>
            {items.filter(item => item.page === Navigation.Pack || item.page === Navigation.Settings || Boolean(packsManager.selectedPack)).map(({page, label, icon: Icon}) => (
                <button
                    type="button"
                    key={page}
                    aria-label={label}
                    aria-current={navigation === page ? "page" : undefined}
                    title={label}
                    onClick={() => changeNavigation(page)}
                    className={`flex min-h-11 items-center justify-center gap-3 rounded-2xl border px-3 py-3 text-sm transition-colors duration-300 lg:justify-start ${page === Navigation.Settings ? "mt-auto" : ""} ${navigation === page ? "border-primary-600/50 bg-primary-800/80 text-primary-50" : "border-transparent text-primary-300 hover:bg-primary-800/40 hover:text-primary-100"}`}
                >
                    <Icon size={19} strokeWidth={1.6} className="shrink-0" aria-hidden="true" />
                    <span className="hidden lg:inline">{label}</span>
                </button>
            ))}
            <div aria-hidden="true" className="mt-5 hidden items-center gap-2 px-3 text-xs text-primary-300 lg:flex">
                <Moon size={13} /> Take it slow.
            </div>
        </nav>
    );
}
