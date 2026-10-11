import { useEffect, useState } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";

// Payload
interface WidgetData {
    test: string
}

// Default Payload
const initialData: WidgetData = { test: "Test value." };

function useWistData<T>(initialValue: T): T {
    // Fetch Payload
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

export default function Template() {
    // To use the payload
    const data = useWistData<WidgetData>(initialData);


    return (
        // To have dynamic value to display on your widget
        <p>{data.test}</p>
    );
}

// To create your page
createRoot(document.getElementById("root")!).render(<Template />);
