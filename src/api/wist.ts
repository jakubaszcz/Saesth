import  { WistClient } from "wist-sdk";
import type { WidgetAction } from "wist-sdk";


let wist: WistClient | null = null;
let operations: Promise<void> = Promise.resolve();
type ActionListener = (action: WidgetAction) => void | Promise<void>;
const actionListeners = new Map<ActionListener, (() => void) | null>();

export function onSaesthWidgetAction(listener: ActionListener): () => void {
    actionListeners.set(listener, wist?.onWidgetAction(WIST_WIDGET_ID, listener) ?? null);
    return () => {
        actionListeners.get(listener)?.();
        actionListeners.delete(listener);
    };
}

function enqueue(operation: () => Promise<void>): Promise<void> {
    const result = operations.then(operation);
    operations = result.catch(() => {});
    return result;
}
export const WIST_WIDGET_ID = "saesth";

interface SaesthWidgetData {
    pack: string | null;
    sounds: string[];
}

let widgetData: SaesthWidgetData = {
    pack: null,
    sounds: [],
};

export function updateSaesthWidget(
    changes: Partial<SaesthWidgetData>,
): Promise<void> {
    widgetData = {
        ...widgetData,
        ...changes,
    };

    return enqueue(async () => {
        if (wist) await wist.updateWidget(WIST_WIDGET_ID, widgetData);
    });
}

export function setSaesthWidgetEnabled(enabled: boolean): Promise<void> {
    return enqueue(async () => {
        if (!enabled) {
            const client = wist;
            try {
                if (client) await client.unregisterWidget(WIST_WIDGET_ID);
            } finally {
                for (const [listener, unsubscribe] of actionListeners) {
                    unsubscribe?.();
                    actionListeners.set(listener, null);
                }
                wist = null;
            }
            return;
        }

        if (wist) return;
        const client = new WistClient();
        if (!await client.isAvailable()) throw new Error("Wist is unavailable");
        await client.registerWidget({
            id: WIST_WIDGET_ID,
            title: "Saesth",
            width: 320,
            height: 160,
            path: "C:\\Users\\bangl\\Documents\\dist-widget",
        });
        wist = client;
        for (const listener of actionListeners.keys()) {
            actionListeners.set(listener, client.onWidgetAction(WIST_WIDGET_ID, listener));
        }
        await client.updateWidget(WIST_WIDGET_ID, widgetData);
    });
}
