import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { SoundData } from "./interface/sound-data.ts";
import "./App.css";
import {SoundCard} from "./component/sound-card.tsx";

type SoundMap = Record<string, SoundData>;

function App() {
  const [sounds, setSounds] = useState<SoundMap>({});

  useEffect(() => {
    async function fetchSounds() {
      try {
        const fetchedSounds = await invoke<SoundMap>("get_sounds");
        setSounds(fetchedSounds);
      } catch (error) {
        console.error("Failed loading songs :", error);
      }
    }
    fetchSounds();
  }, []);

  const handleTogglePlay = async (id: string) => {
    try {
      const updatedSounds = await invoke<SoundMap>("toggle_play", { id });
      setSounds(   updatedSounds);
    } catch (error) {
      console.error("Failed to toggle play:", error);
    }
  };

  const handleVolumeChange = async (id: string, volume: number) => {
    try {
      const updatedSounds = await invoke<SoundMap>("change_volume", { id, volume });
      setSounds(updatedSounds);
    } catch (error) {
      console.error("Failed to change volume:", error);
    }
  }

  return (
    <main className="container">
      <div className="row">
        {Object.entries(sounds).map(([id, data]) => (
            <SoundCard
                id={id}
                data={data}
                onClick={() => handleTogglePlay(id)}
                onChanged={(volume) => handleVolumeChange(id, volume)}
            />
        ))}
      </div>
    </main>
  );
}

export default App;
