use std::path::PathBuf;

fn cache_dir() -> PathBuf {
    dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("dota-terrain-mod")
        .join("images")
}

pub async fn load_terrain_image(terrain_id: u32, url: &str) -> Result<Vec<u8>, String> {
    let cache_path = cache_dir().join(format!("terrain_{terrain_id}.png"));

    // Try cache first
    if cache_path.exists() {
        return tokio::fs::read(&cache_path)
            .await
            .map_err(|e| e.to_string());
    }

    // Download
    let response = reqwest::get(url).await.map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("HTTP {}", response.status()));
    }

    let bytes = response.bytes().await.map_err(|e| e.to_string())?;

    // Cache to disk
    if let Some(parent) = cache_path.parent() {
        let _ = tokio::fs::create_dir_all(parent).await;
    }
    let _ = tokio::fs::write(&cache_path, &bytes).await;

    Ok(bytes.to_vec())
}
