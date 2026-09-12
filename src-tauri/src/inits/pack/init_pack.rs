use std::{fs, io};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use crate::commands::packs::commands_packs::{command_select_pack};
use crate::types::packs::type_packs::Pack;
use zip::ZipArchive;
use crate::global::global::{PACK, PATHS};
use crate::types::manifest::type_manifest::{Manifest, ManifestSounds};
use crate::types::sounds::type_sounds::{Effect, Sound};


pub fn init() -> Vec<Pack> {


    let paths = PATHS.get().unwrap();
    let packs = scan_packs(&paths.packs, &paths.packs_cache);
    let selected = crate::global::global::MANIFEST.get().unwrap().lock().unwrap().pack.clone();
    if !selected.is_empty() {
        if let Err(error) = command_select_pack(selected) {
            eprintln!("Unable to restore selected pack: {error}");
        }
    }
    packs
}

fn scan_packs(path: &Path, cache: &Path) -> Vec<Pack> {
    let Ok(entries) = fs::read_dir(path) else { return Vec::new(); };
    let mut packs = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()).is_some_and(|ext| ext.eq_ignore_ascii_case("zip")) {
            match read_pack(path, cache) {
                Ok(pack) => packs.push(pack),
                Err(error) => eprintln!("Unable to load pack: {error}"),
            }
        }
    }
    packs.sort_by(|a, b| a.id.cmp(&b.id));
    packs.dedup_by(|a, b| a.id == b.id);
    packs
}

pub fn watch_packs(app: tauri::AppHandle) {
    use std::collections::BTreeMap;
    use std::time::{Duration, SystemTime};
    use tauri::Emitter;
    std::thread::spawn(move || {
        let paths = PATHS.get().unwrap();
        let mut previous = BTreeMap::<PathBuf, (u64, Option<SystemTime>)>::new();
        let mut loaded = None;
        loop {
            std::thread::sleep(Duration::from_secs(1));
            let Ok(entries) = fs::read_dir(&paths.packs) else { continue; };
            let snapshot: BTreeMap<_, _> = entries.flatten().filter_map(|entry| {
                let path = entry.path();
                if !path.extension().and_then(|ext| ext.to_str()).is_some_and(|ext| ext.eq_ignore_ascii_case("zip")) { return None; }
                let metadata = entry.metadata().ok()?;
                Some((path, (metadata.len(), metadata.modified().ok())))
            }).collect();
            if snapshot == previous && loaded.as_ref() != Some(&snapshot) {
                let packs = scan_packs(&paths.packs, &paths.packs_cache);
                let complete = packs.len() == snapshot.len();
                let removed_active_pack = complete && PACK.get().is_some_and(|pack| {
                    let pack = pack.lock().unwrap();
                    !pack.id.is_empty()
                        && pack.root == paths.packs_cache.join(&pack.id)
                        && !packs.iter().any(|available| available.id == pack.id)
                });
                if removed_active_pack {
                    if let Err(error) = crate::commands::packs::commands_packs::command_deselect_pack() {
                        eprintln!("Unable to deselect removed pack: {error}");
                        // Retry on the next scan if persisting the selection failed.
                        continue;
                    }
                }
                if complete { loaded = Some(snapshot.clone()); }
                let mut current = crate::global::global::PACKS.get().unwrap().lock().unwrap();
                if removed_active_pack || serde_json::to_value(&*current).ok() != serde_json::to_value(&packs).ok() {
                    *current = packs;
                    let _ = app.emit("packs-changed", ());
                }
            }
            previous = snapshot;
        }
    });
}

fn read_pack(path: PathBuf, cache: &Path) -> Result<Pack, Box<dyn std::error::Error>> {
    let file = File::open(&path)?;
    let mut archive = ZipArchive::new(file)?;

    let mut config: Pack = {
        let manifest = archive.by_name("manifest.json")?;
        serde_json::from_reader(manifest)?
    };

    if config.id.is_empty() || Path::new(&config.id).components().count() != 1
        || !matches!(Path::new(&config.id).components().next(), Some(std::path::Component::Normal(_))) {
        return Err("Invalid pack ID".into());
    }
    let pack_cache = cache.join(&config.id);
    let pack_icon_cache = pack_cache.join("icon.png");

    fs::create_dir_all(&pack_cache)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;

        let Some(enclosed_path) = file.enclosed_name() else {
            continue;
        };

        let output_path = pack_cache.join(enclosed_path);

        if file.is_dir() {
            fs::create_dir_all(&output_path)?;
            continue;
        }

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        if output_path.is_file() {
            continue;
        }

        let mut output = tempfile::NamedTempFile::new_in(output_path.parent().ok_or("Missing parent")?)?;
        io::copy(&mut file, &mut output)?;
        output.persist(&output_path)?;
    }

    config.icon = pack_icon_cache.to_string_lossy().to_string();

    Ok(config)
}

fn make_stream(id: &str, effects: Vec<Effect>, config: &ManifestSounds) -> Sound {

    let sound_id = id.to_string();

    Sound {
        sound_id: sound_id.clone(),
        handle: None,
        player: None,
        play: Arc::new(AtomicBool::new(false)),
        volume: Arc::new(Mutex::new(config.volume)),
        fade_volume: Arc::new(Mutex::new(0.0)),
        drift_volume: Arc::new(Mutex::new(1.0)),
        effects
    }

}

fn make_effect(id: &str) -> Effect {

    let effect_id = id.to_string();

    Effect {
        effect_id: effect_id.clone(),
        active: Arc::new(AtomicBool::new(false)),
    }
}

pub fn init_pack_sound() -> Vec<Sound> {
    let Some(pack) = PACK.get() else {
        return Vec::new();
    };

    let selected_pack = pack.lock().unwrap();
    if selected_pack.id.is_empty() {
        return Vec::new();
    }
    let manifest_path = selected_pack.root.join("manifest.json");

    drop(selected_pack);

    let file = match File::open(manifest_path) {
        Ok(file) => file,
        Err(_) => return Vec::new(),
    };

    let manifest: Manifest = match serde_json::from_reader(file) {
        Ok(m) => m,
        Err(_) => return Vec::new(),
    };

    manifest.sounds
        .iter()
        .map(|sound| {
            let effects = sound.effects
                .iter()
                .map(|effect| make_effect(effect))
                .collect();

            make_stream(&sound.id, effects, &sound)
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn library_tracks_added_and_removed_archives_and_skips_incomplete_copies() {
        let dir = tempfile::tempdir().unwrap();
        let packs = dir.path().join("packs");
        let cache = dir.path().join("cache");
        fs::create_dir_all(&packs).unwrap();
        fs::create_dir_all(&cache).unwrap();
        assert!(scan_packs(&packs, &cache).is_empty());
        let path = packs.join("test.zip");
        fs::write(&path, "incomplete zip").unwrap();
        assert!(scan_packs(&packs, &cache).is_empty());
        let mut archive = zip::ZipWriter::new(File::create(&path).unwrap());
        archive.start_file("manifest.json", zip::write::SimpleFileOptions::default()).unwrap();
        archive.write_all(br#"{"id":"test","name":"Test","description":"A pack","icon":"icon.png"}"#).unwrap();
        archive.finish().unwrap();
        assert_eq!(scan_packs(&packs, &cache)[0].id, "test");
        fs::write(cache.join("test/manifest.json"), "user preferences").unwrap();
        assert_eq!(scan_packs(&packs, &cache).len(), 1);
        assert_eq!(fs::read_to_string(cache.join("test/manifest.json")).unwrap(), "user preferences");
        fs::remove_file(path).unwrap();
        assert!(scan_packs(&packs, &cache).is_empty());
    }
}
