use std::{
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

pub struct World {
    pub name: String,
    pub path: PathBuf,
    pub last_modified: SystemTime,
}

pub fn minecraft_saves_dir() -> Option<PathBuf> {
    if cfg!(target_os = "windows") {
        std::env::var("APPDATA")
            .ok()
            .map(|appdata| PathBuf::from(appdata).join(".minecraft").join("saves"))
    } else {
        dirs::home_dir().map(|home| home.join(".minecraft").join("saves"))
    }
}

pub fn discover_worlds() -> Vec<World> {
    let Some(saves_dir) = minecraft_saves_dir() else {
        return Vec::new();
    };

    let Ok(entries) = fs::read_dir(&saves_dir) else {
        return Vec::new();
    };

    let mut worlds: Vec<World> = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if !path.is_dir() {
                return None;
            }

            let level_dat = path.join("level.dat");
            if !level_dat.exists() {
                return None;
            }

            let last_modified = level_dat.metadata().ok()?.modified().ok()?;
            let name = path.file_name()?.to_string_lossy().into_owned();

            Some(World {
                name,
                path,
                last_modified,
            })
        })
        .collect();

    worlds.sort_by(|a, b| b.last_modified.cmp(&a.last_modified));
    worlds
}

pub fn find_stats_file(world_path: &Path) -> Option<PathBuf> {
    let stats_dir = world_path.join("stats");
    let entries = fs::read_dir(&stats_dir).ok()?;

    entries
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .find(|p| p.extension().is_some_and(|ext| ext == "json"))
}

pub fn latest_world(worlds: &[World]) -> Option<&World> {
    worlds.iter().max_by_key(|w| w.last_modified)
}
