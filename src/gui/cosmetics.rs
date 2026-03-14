use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CosmeticCategory {
    Terrain,
    Announcer,
    Music,
    Cursor,
    Hud,
    Weather,
}

impl CosmeticCategory {
    pub const ALL: &'static [CosmeticCategory] = &[
        CosmeticCategory::Terrain,
        CosmeticCategory::Announcer,
        CosmeticCategory::Music,
        CosmeticCategory::Cursor,
        CosmeticCategory::Hud,
        CosmeticCategory::Weather,
    ];

    pub fn display_name(&self) -> &'static str {
        match self {
            CosmeticCategory::Terrain => "Terrains",
            CosmeticCategory::Announcer => "Announcers",
            CosmeticCategory::Music => "Music",
            CosmeticCategory::Cursor => "Cursors",
            CosmeticCategory::Hud => "HUD Skins",
            CosmeticCategory::Weather => "Weather",
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CosmeticItem {
    pub id: u32,
    pub name: String,
    pub category: CosmeticCategory,
    pub vpk_prefix: String,
    pub output_prefix: String,
}

/// Discover available cosmetic items by scanning file paths from a VPK index.
/// Groups items by category based on known path patterns.
pub fn discover_cosmetics(vpk_index: &[String]) -> HashMap<CosmeticCategory, Vec<CosmeticItem>> {
    let mut items: HashMap<CosmeticCategory, Vec<CosmeticItem>> = HashMap::new();
    let mut id_counter: u32 = 100;

    // Track unique prefixes we've already seen to avoid duplicates
    let mut seen_announcers: Vec<String> = Vec::new();
    let mut seen_music: Vec<String> = Vec::new();
    let mut seen_cursors: Vec<String> = Vec::new();
    let mut seen_huds: Vec<String> = Vec::new();
    let mut seen_weather: Vec<String> = Vec::new();

    for path in vpk_index {
        // Announcers: sounds/vo/announcer* directories
        if path.starts_with("sounds/vo/") {
            if let Some(dir) = extract_dir_at_depth(path, 2) {
                if dir.starts_with("announcer") && !seen_announcers.contains(&dir) {
                    seen_announcers.push(dir.clone());
                    let prefix = format!("sounds/vo/{dir}/");
                    items
                        .entry(CosmeticCategory::Announcer)
                        .or_default()
                        .push(CosmeticItem {
                            id: id_counter,
                            name: prettify_name(&dir, &["announcer_dlc_", "announcer_"]),
                            category: CosmeticCategory::Announcer,
                            vpk_prefix: prefix,
                            output_prefix: "sounds/vo/announcer/".to_string(),
                        });
                    id_counter += 1;
                }
            }
        }

        // Music packs: sounds/music/* directories
        if path.starts_with("sounds/music/") {
            if let Some(dir) = extract_dir_at_depth(path, 2) {
                if dir != "default" && !seen_music.contains(&dir) {
                    seen_music.push(dir.clone());
                    let prefix = format!("sounds/music/{dir}/");
                    items
                        .entry(CosmeticCategory::Music)
                        .or_default()
                        .push(CosmeticItem {
                            id: id_counter,
                            name: prettify_name(&dir, &["valve_dota_", "valve_"]),
                            category: CosmeticCategory::Music,
                            vpk_prefix: prefix,
                            output_prefix: "sounds/music/default/".to_string(),
                        });
                    id_counter += 1;
                }
            }
        }

        // Cursor packs: resource/cursor/* or resource/cursor_* patterns
        if path.starts_with("resource/cursor") {
            if let Some(dir) = extract_dir_at_depth(path, 1) {
                if dir.starts_with("cursor") && dir != "cursor" && !seen_cursors.contains(&dir) {
                    seen_cursors.push(dir.clone());
                    let prefix = format!("resource/{dir}/");
                    items
                        .entry(CosmeticCategory::Cursor)
                        .or_default()
                        .push(CosmeticItem {
                            id: id_counter,
                            name: prettify_name(&dir, &["cursor_pack_", "cursor_"]),
                            category: CosmeticCategory::Cursor,
                            vpk_prefix: prefix,
                            output_prefix: "resource/cursor/".to_string(),
                        });
                    id_counter += 1;
                }
            }
        }

        // HUD skins: resource/flash3/images/hud_skins/*
        if path.starts_with("resource/flash3/images/hud_skins/") {
            if let Some(dir) = extract_dir_at_depth(path, 4) {
                if !seen_huds.contains(&dir) {
                    seen_huds.push(dir.clone());
                    let prefix = format!("resource/flash3/images/hud_skins/{dir}/");
                    items
                        .entry(CosmeticCategory::Hud)
                        .or_default()
                        .push(CosmeticItem {
                            id: id_counter,
                            name: prettify_name(&dir, &["hud_skin_", "hud_"]),
                            category: CosmeticCategory::Hud,
                            vpk_prefix: prefix,
                            output_prefix: "resource/flash3/images/hud_skins/default/".to_string(),
                        });
                    id_counter += 1;
                }
            }
        }

        // Weather effects: particles/weather/*
        if path.starts_with("particles/weather/") {
            if let Some(dir) = extract_dir_at_depth(path, 2) {
                if !seen_weather.contains(&dir) {
                    seen_weather.push(dir.clone());
                    let prefix = format!("particles/weather/{dir}/");
                    items
                        .entry(CosmeticCategory::Weather)
                        .or_default()
                        .push(CosmeticItem {
                            id: id_counter,
                            name: prettify_name(&dir, &["weather_"]),
                            category: CosmeticCategory::Weather,
                            vpk_prefix: prefix,
                            output_prefix: format!("particles/weather/{dir}/"),
                        });
                    id_counter += 1;
                }
            }
        }
    }

    // Sort items within each category alphabetically
    for list in items.values_mut() {
        list.sort_by(|a, b| a.name.cmp(&b.name));
    }

    items
}

/// Extract the directory name at a specific depth from a file path.
/// e.g., "sounds/vo/announcer_dlc_bastion/file.txt" at depth 2 -> "announcer_dlc_bastion"
fn extract_dir_at_depth(path: &str, depth: usize) -> Option<String> {
    let parts: Vec<&str> = path.split('/').collect();
    if parts.len() > depth + 1 {
        Some(parts[depth].to_string())
    } else {
        None
    }
}

/// Prettify a directory name into a display name.
/// Strips known prefixes, replaces underscores with spaces, and title-cases.
fn prettify_name(raw: &str, strip_prefixes: &[&str]) -> String {
    let mut name = raw.to_string();
    for prefix in strip_prefixes {
        if let Some(stripped) = name.strip_prefix(prefix) {
            name = stripped.to_string();
            break;
        }
    }
    name.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => {
                    let upper: String = c.to_uppercase().collect();
                    upper + &chars.collect::<String>()
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
