import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface SoundData {
  play: boolean;
  path: string;
}

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

  return (
    <main className="container">
      <div className="row">
        <h3>Available songs :</h3>
        <ul style={{ textAlign: 'left' }}>
          {Object.entries(sounds).map(([id, data]) => (
            <li key={id}>
              <strong>{id}</strong>: {data.path} {data.play ? " (Playing)" : " (Stop)"}
            </li>
          ))}
        </ul>
      </div>
    </main>
  );
}

export default App;
