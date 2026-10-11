import { useEffect, useState } from "react";
import { createRoot } from "react-dom/client";
import { Layers, Waves, X, Headphones } from "lucide-react";
import "./index.css";

interface SaesthWidgetData {
    pack: string | null;
    sounds: string[];
}
const initialData: SaesthWidgetData = { pack: null, sounds: [] };

function useWistData<T>(initialValue: T): T {
    const [data, setData] = useState<T>(initialValue);
    useEffect(() => {
        function receive(event: MessageEvent) {
            if (event.source !== window.parent || event.data?.type !== "wist:data") return;
            if (event.data.data != null) setData(event.data.data as T);
        }
        window.addEventListener("message", receive);
        window.parent.postMessage({ type: "wist:ready" }, "*");
        return () => window.removeEventListener("message", receive);
    }, []);
    return data;
}

export default function TestWidget() {
    const data = useWistData<SaesthWidgetData>(initialData);
    const sounds = data.sounds ?? [];
    const hasPack = Boolean(data.pack);
    const [limit, setLimit] = useState(3);
    useEffect(() => {
        const observer = new ResizeObserver(([entry]) => {
            setLimit(entry.contentRect.width < 220 ? 1 : entry.contentRect.width < 380 ? 2 : 4);
        });
        observer.observe(document.documentElement);
        return () => observer.disconnect();
    }, []);

    return (
        <main className="saesth-widget">
            <header className="widget-heading">
                <span className="wordmark"><Waves aria-hidden="true" /> SAESTH</span>
                <span className="playback"><i className={sounds.length ? "active" : ""} />{sounds.length ? "Playing" : "Quiet"}</span>
            </header>

            <section className="pack-panel" aria-label="Current pack">
                <span className="pack-icon"><Layers aria-hidden="true" /></span>
                <div className="pack-copy">
                    <h2 className="eyebrow">Current pack</h2>
                    <p className="pack-name" title={data.pack || "No pack selected"}>{data.pack || "No pack selected"}</p>
                </div>
            </section>

            <section className="sounds-section" aria-label="Active sounds">
                <div className="section-heading"><h2 className="eyebrow">Active sounds</h2><span className="sound-count">{sounds.length}</span></div>
                {sounds.length ? (
                    <ul className="sounds" aria-label={sounds.join(", ")}>
                        {sounds.slice(0, limit).map((sound, index) => (
                            <li className="sound-chip" key={`${sound}-${index}`} title={sound}><Waves aria-hidden="true" /><span>{sound}</span></li>
                        ))}
                        {sounds.length > limit && <li className="more" title={sounds.slice(limit).join(", ")} aria-label={`More sounds: ${sounds.slice(limit).join(", ")}`}>+{sounds.length - limit}</li>}
                    </ul>
                ) : <p className="empty-sounds"><Headphones aria-hidden="true" />No sounds playing</p>}
            </section>

            <footer className="widget-footer">
                <span className="footer-note">Your space, your sound.</span>
                <button type="button" className="remove-pack" disabled={!hasPack}
                    onClick={() => window.parent.postMessage({ type: "wist:action", action: { type: "deselect-pack" } }, "*")}>
                    <X aria-hidden="true" /><span>Remove pack</span>
                </button>
            </footer>
        </main>
    );
}
createRoot(document.getElementById("root")!).render(<TestWidget />);
