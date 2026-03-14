use std::collections::HashMap;
use std::path::PathBuf;

use iced::widget::{
    button, center, column, container, horizontal_rule, horizontal_space, image, row, scrollable,
    text, Space,
};
use iced::{Center, Element, Fill, Task};

mod gui;
mod utils;
mod vpk;

use crate::gui::cosmetics::{CosmeticCategory, CosmeticItem};
use crate::gui::images::load_terrain_image;
use crate::gui::style;
use crate::gui::terrain::TERRAINS;
use crate::utils::Dota;

fn main() -> iced::Result {
    iced::application("Dota 2 Mod Tool", update, view)
        .theme(|_state| style::dark_theme())
        .window_size((1060.0, 720.0))
        .run_with(init)
}

// ── State ──────────────────────────────────────────────────────────────────

struct App {
    dota: Option<Dota>,
    init_error: Option<String>,
    active_tab: CosmeticCategory,
    selected_terrain: Option<usize>,
    selected_cosmetics: HashMap<CosmeticCategory, usize>,
    discovered_items: HashMap<CosmeticCategory, Vec<CosmeticItem>>,
    images: HashMap<u32, image::Handle>,
    patch_status: PatchStatus,
    scan_status: ScanStatus,
}

#[derive(Debug, Clone, PartialEq)]
enum PatchStatus {
    Idle,
    Patching,
    Success,
    Error(String),
}

#[derive(Debug, Clone, PartialEq)]
enum ScanStatus {
    Scanning,
    Done,
    Error(String),
}

// ── Messages ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
enum Message {
    TabSelected(CosmeticCategory),
    TerrainSelected(usize),
    CosmeticSelected(CosmeticCategory, usize),
    ImageLoaded(u32, Result<Vec<u8>, String>),
    ApplyMods,
    PatchComplete(Result<(), String>),
    LaunchDota,
    ScanComplete(Result<HashMap<CosmeticCategory, Vec<CosmeticItem>>, String>),
}

// ── Init ───────────────────────────────────────────────────────────────────

fn init() -> (App, Task<Message>) {
    let (dota, init_error) = match Dota::new() {
        Ok(d) => (Some(d), None),
        Err(e) => (None, Some(e.to_string())),
    };

    let mut tasks: Vec<Task<Message>> = Vec::new();

    // Fire off terrain image downloads
    for t in TERRAINS.iter() {
        let id = t.id;
        let url = t.image_url.to_string();
        tasks.push(Task::perform(
            async move { load_terrain_image(id, &url).await },
            move |result| Message::ImageLoaded(id, result),
        ));
    }

    // Scan pak01_dir.vpk for cosmetic items
    if let Some(ref d) = dota {
        let pak01_path = d.dota_path.join("dota").join("pak01_dir.vpk");
        tasks.push(Task::perform(
            scan_pak01_async(pak01_path),
            |result| Message::ScanComplete(result),
        ));
    }

    (
        App {
            dota,
            init_error,
            active_tab: CosmeticCategory::Terrain,
            selected_terrain: None,
            selected_cosmetics: HashMap::new(),
            discovered_items: HashMap::new(),
            images: HashMap::new(),
            patch_status: PatchStatus::Idle,
            scan_status: ScanStatus::Scanning,
        },
        Task::batch(tasks),
    )
}

// ── Update ─────────────────────────────────────────────────────────────────

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::TabSelected(category) => {
            app.active_tab = category;
            Task::none()
        }

        Message::TerrainSelected(idx) => {
            app.selected_terrain = Some(idx);
            if app.patch_status == PatchStatus::Success {
                app.patch_status = PatchStatus::Idle;
            }
            Task::none()
        }

        Message::CosmeticSelected(category, idx) => {
            // Toggle: deselect if already selected
            if app.selected_cosmetics.get(&category) == Some(&idx) {
                app.selected_cosmetics.remove(&category);
            } else {
                app.selected_cosmetics.insert(category, idx);
            }
            if app.patch_status == PatchStatus::Success {
                app.patch_status = PatchStatus::Idle;
            }
            Task::none()
        }

        Message::ImageLoaded(id, Ok(bytes)) => {
            app.images.insert(id, image::Handle::from_bytes(bytes));
            Task::none()
        }

        Message::ImageLoaded(_id, Err(_)) => Task::none(),

        Message::ApplyMods => {
            if let Some(dota) = &app.dota {
                let dota_path = dota.dota_path.clone();

                let terrain_vpk = app
                    .selected_terrain
                    .map(|idx| TERRAINS[idx].vpk_file.to_string());

                let cosmetic_selections: Vec<(String, String)> = app
                    .selected_cosmetics
                    .iter()
                    .filter_map(|(cat, &idx)| {
                        app.discovered_items.get(cat).and_then(|items| {
                            items.get(idx).map(|item| {
                                (item.vpk_prefix.clone(), item.output_prefix.clone())
                            })
                        })
                    })
                    .collect();

                if terrain_vpk.is_none() && cosmetic_selections.is_empty() {
                    return Task::none();
                }

                app.patch_status = PatchStatus::Patching;

                Task::perform(
                    apply_all_async(dota_path, terrain_vpk, cosmetic_selections),
                    Message::PatchComplete,
                )
            } else {
                Task::none()
            }
        }

        Message::PatchComplete(result) => {
            app.patch_status = match result {
                Ok(()) => PatchStatus::Success,
                Err(e) => PatchStatus::Error(e),
            };
            Task::none()
        }

        Message::LaunchDota => {
            if let Err(e) = launch_dota() {
                app.patch_status = PatchStatus::Error(e);
            }
            Task::none()
        }

        Message::ScanComplete(Ok(items)) => {
            app.discovered_items = items;
            app.scan_status = ScanStatus::Done;
            Task::none()
        }

        Message::ScanComplete(Err(e)) => {
            app.scan_status = ScanStatus::Error(e);
            Task::none()
        }
    }
}

// ── View ───────────────────────────────────────────────────────────────────

fn view(app: &App) -> Element<'_, Message> {
    // Error screen if Dota/Steam not found
    if let Some(err) = &app.init_error {
        return center(
            column![
                text("Could not locate Dota 2").size(28),
                text(err.as_str()).size(16),
                Space::with_height(10),
                text("Please ensure Steam and Dota 2 are installed.").size(14),
            ]
            .spacing(8)
            .align_x(Center),
        )
        .into();
    }

    // ── Header ─────────────────────────────────────────────────────
    let header = container(
        text("DOTA 2 MOD TOOL")
            .size(22)
            .color(iced::Color::from_rgb(0.92, 0.92, 0.92)),
    )
    .padding([15, 20])
    .width(Fill)
    .style(style::header_style);

    // ── Sidebar ────────────────────────────────────────────────────
    let mut sidebar_col = column![].spacing(4).padding([10, 8]);

    for &category in CosmeticCategory::ALL {
        let is_active = app.active_tab == category;
        let label = text(category.display_name()).size(13);

        let mut tab_btn = button(label).padding([10, 16]).width(Fill);

        tab_btn = tab_btn.on_press(Message::TabSelected(category));

        tab_btn = if is_active {
            tab_btn.style(style::active_tab_button_style)
        } else {
            tab_btn.style(style::tab_button_style)
        };

        // Show selection indicator
        let has_selection = match category {
            CosmeticCategory::Terrain => app.selected_terrain.is_some(),
            _ => app.selected_cosmetics.contains_key(&category),
        };

        if has_selection {
            let dot = text("*")
                .size(14)
                .color(iced::Color::from_rgb(0.2, 0.8, 0.35));
            sidebar_col = sidebar_col.push(row![tab_btn, dot].align_y(Center).spacing(4));
        } else {
            sidebar_col = sidebar_col.push(tab_btn);
        }
    }

    let sidebar = container(scrollable(sidebar_col).height(Fill))
        .width(140)
        .height(Fill)
        .style(style::sidebar_style);

    // ── Content area ───────────────────────────────────────────────
    let content: Element<'_, Message> = match app.active_tab {
        CosmeticCategory::Terrain => {
            let mut grid = column![].spacing(12);
            for chunk_start in (0..TERRAINS.len()).step_by(4) {
                let chunk_end = (chunk_start + 4).min(TERRAINS.len());
                let mut r = row![].spacing(12);
                for idx in chunk_start..chunk_end {
                    r = r.push(terrain_card(app, idx, &TERRAINS[idx]));
                }
                grid = grid.push(r);
            }
            container(
                scrollable(container(grid).padding([15, 20]).width(Fill)).height(Fill),
            )
            .into()
        }
        category => {
            match &app.scan_status {
                ScanStatus::Scanning => {
                    center(
                        column![
                            text("Scanning game files...").size(16),
                            Space::with_height(8),
                            text("Parsing pak01_dir.vpk index")
                                .size(12)
                                .color(iced::Color::from_rgb(0.5, 0.5, 0.55)),
                        ]
                        .align_x(Center),
                    )
                    .into()
                }
                ScanStatus::Error(e) => {
                    center(
                        column![
                            text("Failed to scan game files").size(16),
                            text(e.as_str())
                                .size(12)
                                .color(iced::Color::from_rgb(0.9, 0.25, 0.25)),
                        ]
                        .spacing(8)
                        .align_x(Center),
                    )
                    .into()
                }
                ScanStatus::Done => {
                    if let Some(items) = app.discovered_items.get(&category) {
                        if items.is_empty() {
                            center(
                                text(format!("No {} found in game files", category.display_name().to_lowercase()))
                                    .size(14)
                                    .color(iced::Color::from_rgb(0.5, 0.5, 0.55)),
                            )
                            .into()
                        } else {
                            let selected_idx = app.selected_cosmetics.get(&category).copied();
                            let mut grid = column![].spacing(12);
                            for chunk_start in (0..items.len()).step_by(4) {
                                let chunk_end = (chunk_start + 4).min(items.len());
                                let mut r = row![].spacing(12);
                                for idx in chunk_start..chunk_end {
                                    r = r.push(cosmetic_card(
                                        &items[idx],
                                        idx,
                                        selected_idx == Some(idx),
                                        category,
                                    ));
                                }
                                grid = grid.push(r);
                            }
                            container(
                                scrollable(container(grid).padding([15, 20]).width(Fill))
                                    .height(Fill),
                            )
                            .into()
                        }
                    } else {
                        center(
                            text(format!("No {} found in game files", category.display_name().to_lowercase()))
                                .size(14)
                                .color(iced::Color::from_rgb(0.5, 0.5, 0.55)),
                        )
                        .into()
                    }
                }
            }
        }
    };

    // ── Bottom bar ─────────────────────────────────────────────────
    let status_text: Element<'_, Message> = match &app.patch_status {
        PatchStatus::Idle => {
            let has_any = app.selected_terrain.is_some()
                || !app.selected_cosmetics.is_empty();
            if has_any {
                text("Ready to apply")
                    .size(13)
                    .color(iced::Color::from_rgb(0.6, 0.6, 0.65))
                    .into()
            } else {
                text("Select items to mod")
                    .size(13)
                    .color(iced::Color::from_rgb(0.5, 0.5, 0.55))
                    .into()
            }
        }
        PatchStatus::Patching => text("Applying mods... please wait")
            .size(13)
            .color(iced::Color::from_rgb(0.9, 0.75, 0.2))
            .into(),
        PatchStatus::Success => text("Mods applied! Launch Dota 2 to play.")
            .size(13)
            .color(iced::Color::from_rgb(0.2, 0.8, 0.35))
            .into(),
        PatchStatus::Error(e) => text(format!("Error: {e}"))
            .size(13)
            .color(iced::Color::from_rgb(0.9, 0.25, 0.25))
            .into(),
    };

    let has_any_selection =
        app.selected_terrain.is_some() || !app.selected_cosmetics.is_empty();
    let can_apply = has_any_selection && app.patch_status != PatchStatus::Patching;

    let apply_btn = button(text("Apply Mods").size(14))
        .padding([10, 24])
        .on_press_maybe(if can_apply {
            Some(Message::ApplyMods)
        } else {
            None
        })
        .style(style::apply_button_style);

    let launch_btn = button(text("Launch Dota 2").size(14))
        .padding([10, 24])
        .on_press(Message::LaunchDota)
        .style(style::launch_button_style);

    // Build selection summary
    let selection_summary = build_selection_summary(app);
    let selection_label: Element<'_, Message> = if !selection_summary.is_empty() {
        text(selection_summary)
            .size(12)
            .color(iced::Color::from_rgb(0.85, 0.85, 0.88))
            .into()
    } else {
        text("").size(12).into()
    };

    let bottom_bar = container(
        column![
            row![selection_label, horizontal_space(), status_text]
                .align_y(Center)
                .spacing(10),
            Space::with_height(8),
            row![apply_btn, horizontal_space(), launch_btn]
                .align_y(Center)
                .spacing(10),
        ]
        .padding([12, 20]),
    )
    .width(Fill)
    .style(style::bottom_bar_style);

    // ── Compose ────────────────────────────────────────────────────
    column![
        header,
        horizontal_rule(1),
        row![sidebar, container(content).width(Fill).height(Fill)],
        horizontal_rule(1),
        bottom_bar
    ]
    .into()
}

fn terrain_card<'a>(
    app: &'a App,
    idx: usize,
    terrain: &'a gui::terrain::TerrainInfo,
) -> Element<'a, Message> {
    let is_selected = app.selected_terrain == Some(idx);

    let image_content: Element<'_, Message> = match app.images.get(&terrain.id) {
        Some(handle) => container(image(handle.clone()).width(Fill).height(120)).into(),
        None => container(center(text("No Preview").size(12)))
            .width(Fill)
            .height(120)
            .style(style::placeholder_style)
            .into(),
    };

    let label = text(terrain.name).size(13);
    let desc = text(terrain.description)
        .size(11)
        .color(iced::Color::from_rgb(0.55, 0.55, 0.60));

    let card_content = column![image_content, label, desc].spacing(6).width(185);

    let styled_card = container(card_content).padding(10).style(if is_selected {
        style::selected_card_style
    } else {
        style::card_style
    });

    button(styled_card)
        .on_press(Message::TerrainSelected(idx))
        .style(style::card_button_style)
        .into()
}

fn cosmetic_card(
    item: &CosmeticItem,
    idx: usize,
    is_selected: bool,
    category: CosmeticCategory,
) -> Element<'_, Message> {
    let placeholder: Element<'_, Message> = container(center(text("No Preview").size(12)))
        .width(Fill)
        .height(120)
        .style(style::placeholder_style)
        .into();

    let label = text(item.name.as_str()).size(13);
    let desc = text(category.display_name())
        .size(11)
        .color(iced::Color::from_rgb(0.55, 0.55, 0.60));

    let card_content = column![placeholder, label, desc].spacing(6).width(185);

    let styled_card = container(card_content).padding(10).style(if is_selected {
        style::selected_card_style
    } else {
        style::card_style
    });

    button(styled_card)
        .on_press(Message::CosmeticSelected(category, idx))
        .style(style::card_button_style)
        .into()
}

fn build_selection_summary(app: &App) -> String {
    let mut parts: Vec<String> = Vec::new();

    if let Some(idx) = app.selected_terrain {
        parts.push(format!("Terrain: {}", TERRAINS[idx].name));
    }

    for &category in CosmeticCategory::ALL {
        if category == CosmeticCategory::Terrain {
            continue;
        }
        if let Some(&idx) = app.selected_cosmetics.get(&category) {
            if let Some(items) = app.discovered_items.get(&category) {
                if let Some(item) = items.get(idx) {
                    parts.push(format!("{}: {}", category.display_name(), item.name));
                }
            }
        }
    }

    parts.join(" | ")
}

// ── Async scanning ────────────────────────────────────────────────────────

async fn scan_pak01_async(
    pak01_path: PathBuf,
) -> Result<HashMap<CosmeticCategory, Vec<CosmeticItem>>, String> {
    tokio::task::spawn_blocking(move || {
        if !pak01_path.exists() {
            return Err(format!(
                "pak01_dir.vpk not found at {}",
                pak01_path.display()
            ));
        }

        let result = std::panic::catch_unwind(|| {
            let index = vpk::scan_vpk_index(pak01_path);
            gui::cosmetics::discover_cosmetics(&index)
        });

        match result {
            Ok(items) => Ok(items),
            Err(_) => Err("Failed to parse pak01_dir.vpk. It may be corrupted.".to_string()),
        }
    })
    .await
    .map_err(|e| format!("Scan task failed: {e}"))?
}

// ── Async patching ────────────────────────────────────────────────────────

async fn apply_all_async(
    dota_path: PathBuf,
    terrain_vpk: Option<String>,
    cosmetic_selections: Vec<(String, String)>,
) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        let tempcontent = dota_path.join("dota_tempcontent");

        // Apply terrain if selected
        if let Some(vpk_file) = terrain_vpk {
            let base_path = dota_path.join("dota").join("maps").join("dota.vpk");
            let target_path = dota_path.join("dota").join("maps").join(&vpk_file);
            let out_path = tempcontent.join("maps").join("dota.vpk");

            let result =
                std::panic::catch_unwind(|| vpk::create_terrain(base_path, target_path));

            match result {
                Ok(out_file) => {
                    std::fs::create_dir_all(out_path.parent().unwrap())
                        .map_err(|e| format!("Failed to create output directory: {e}"))?;
                    std::fs::write(&out_path, &out_file)
                        .map_err(|e| format!("Failed to write VPK: {e}"))?;
                }
                Err(_) => {
                    return Err("Failed to parse terrain VPK file. It may be corrupted.".into())
                }
            }
        }

        // Apply non-terrain cosmetics by extracting from pak01_dir.vpk
        if !cosmetic_selections.is_empty() {
            let pak01_path = dota_path.join("dota").join("pak01_dir.vpk");

            if !pak01_path.exists() {
                return Err(format!(
                    "pak01_dir.vpk not found at {}",
                    pak01_path.display()
                ));
            }

            for (vpk_prefix, output_prefix) in &cosmetic_selections {
                let prefixes = [vpk_prefix.as_str()];

                let result = std::panic::catch_unwind(|| {
                    vpk::extract_files_by_prefix(pak01_path.clone(), &prefixes)
                });

                match result {
                    Ok(files) => {
                        for (path, data) in files {
                            let output_path =
                                vpk::remap_path(&path, &prefixes, output_prefix);
                            let full_path = tempcontent.join(&output_path);
                            if let Some(parent) = full_path.parent() {
                                std::fs::create_dir_all(parent).map_err(|e| {
                                    format!("Failed to create directory: {e}")
                                })?;
                            }
                            std::fs::write(&full_path, &data)
                                .map_err(|e| format!("Failed to write file: {e}"))?;
                        }
                    }
                    Err(_) => {
                        return Err(format!(
                            "Failed to extract files for prefix: {vpk_prefix}"
                        ))
                    }
                }
            }
        }

        Ok(())
    })
    .await
    .map_err(|e| format!("Patching task failed: {e}"))?
}

// ── Dota 2 launch ──────────────────────────────────────────────────────────

fn launch_dota() -> Result<(), String> {
    // Try Steam protocol URL first
    if open::that("steam://rungameid/570//+language tempcontent").is_ok() {
        return Ok(());
    }

    // Fallback: direct command
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("steam.exe")
            .args(["-applaunch", "570", "-language", "tempcontent"])
            .spawn()
            .map_err(|e| format!("Failed to launch Dota 2: {e}"))?;
        return Ok(());
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("steam")
            .args(["-applaunch", "570", "-language", "tempcontent"])
            .spawn()
            .map_err(|e| format!("Failed to launch Dota 2: {e}"))?;
        return Ok(());
    }

    #[allow(unreachable_code)]
    Err("Could not launch Dota 2".to_string())
}
