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

use crate::gui::images::load_terrain_image;
use crate::gui::style;
use crate::gui::terrain::TERRAINS;
use crate::utils::Dota;

fn main() -> iced::Result {
    iced::application("Dota Terrain Mod", update, view)
        .theme(|_state| style::dark_theme())
        .window_size((960.0, 720.0))
        .run_with(init)
}

// ── State ──────────────────────────────────────────────────────────────────

struct App {
    dota: Option<Dota>,
    init_error: Option<String>,
    selected: Option<usize>,
    images: HashMap<u32, image::Handle>,
    patch_status: PatchStatus,
}

#[derive(Debug, Clone, PartialEq)]
enum PatchStatus {
    Idle,
    Patching,
    Success,
    Error(String),
}

// ── Messages ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
enum Message {
    TerrainSelected(usize),
    ImageLoaded(u32, Result<Vec<u8>, String>),
    ApplyTerrain,
    PatchComplete(Result<(), String>),
    LaunchDota,
}

// ── Init ───────────────────────────────────────────────────────────────────

fn init() -> (App, Task<Message>) {
    let (dota, init_error) = match Dota::new() {
        Ok(d) => (Some(d), None),
        Err(e) => (None, Some(e.to_string())),
    };

    // Fire off image downloads for all terrains concurrently
    let image_tasks: Vec<Task<Message>> = TERRAINS
        .iter()
        .map(|t| {
            let id = t.id;
            let url = t.image_url.to_string();
            Task::perform(
                async move { load_terrain_image(id, &url).await },
                move |result| Message::ImageLoaded(id, result),
            )
        })
        .collect();

    (
        App {
            dota,
            init_error,
            selected: None,
            images: HashMap::new(),
            patch_status: PatchStatus::Idle,
        },
        Task::batch(image_tasks),
    )
}

// ── Update ─────────────────────────────────────────────────────────────────

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::TerrainSelected(idx) => {
            app.selected = Some(idx);
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

        Message::ApplyTerrain => {
            if let (Some(idx), Some(dota)) = (app.selected, &app.dota) {
                let terrain = &TERRAINS[idx];
                let dota_path = dota.dota_path.clone();
                let vpk_file = terrain.vpk_file.to_string();

                app.patch_status = PatchStatus::Patching;

                Task::perform(
                    patch_terrain_async(dota_path, vpk_file),
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
        text("DOTA TERRAIN MOD")
            .size(22)
            .color(iced::Color::from_rgb(0.92, 0.92, 0.92)),
    )
    .padding([15, 20])
    .width(Fill)
    .style(style::header_style);

    // ── Terrain grid ───────────────────────────────────────────────
    let mut grid = column![].spacing(12);

    for chunk_start in (0..TERRAINS.len()).step_by(4) {
        let chunk_end = (chunk_start + 4).min(TERRAINS.len());
        let mut r = row![].spacing(12);
        for idx in chunk_start..chunk_end {
            r = r.push(terrain_card(app, idx, &TERRAINS[idx]));
        }
        grid = grid.push(r);
    }

    let grid_container = container(
        scrollable(container(grid).padding([15, 20]).width(Fill)).height(Fill),
    );

    // ── Bottom bar ─────────────────────────────────────────────────
    let status_text: Element<'_, Message> = match &app.patch_status {
        PatchStatus::Idle => {
            if app.selected.is_some() {
                text("Ready to apply")
                    .size(13)
                    .color(iced::Color::from_rgb(0.6, 0.6, 0.65))
                    .into()
            } else {
                text("Select a terrain")
                    .size(13)
                    .color(iced::Color::from_rgb(0.5, 0.5, 0.55))
                    .into()
            }
        }
        PatchStatus::Patching => text("Patching VPK... please wait")
            .size(13)
            .color(iced::Color::from_rgb(0.9, 0.75, 0.2))
            .into(),
        PatchStatus::Success => text("Terrain applied! Launch Dota 2 to play.")
            .size(13)
            .color(iced::Color::from_rgb(0.2, 0.8, 0.35))
            .into(),
        PatchStatus::Error(e) => text(format!("Error: {e}"))
            .size(13)
            .color(iced::Color::from_rgb(0.9, 0.25, 0.25))
            .into(),
    };

    let can_apply = app.selected.is_some() && app.patch_status != PatchStatus::Patching;

    let apply_btn = button(text("Apply Terrain").size(14))
        .padding([10, 24])
        .on_press_maybe(if can_apply {
            Some(Message::ApplyTerrain)
        } else {
            None
        })
        .style(style::apply_button_style);

    let launch_btn = button(text("Launch Dota 2").size(14))
        .padding([10, 24])
        .on_press(Message::LaunchDota)
        .style(style::launch_button_style);

    let selected_name = app.selected.map(|i| TERRAINS[i].name).unwrap_or("");

    let selection_label: Element<'_, Message> = if !selected_name.is_empty() {
        text(format!("Selected: {selected_name}"))
            .size(14)
            .color(iced::Color::from_rgb(0.85, 0.85, 0.88))
            .into()
    } else {
        text("")
            .size(14)
            .into()
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
        grid_container,
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
    let is_selected = app.selected == Some(idx);

    // Image or placeholder
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

    let card_content = column![image_content, label, desc].spacing(6).width(200);

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

// ── Async patching ─────────────────────────────────────────────────────────

async fn patch_terrain_async(dota_path: PathBuf, vpk_file: String) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        let base_path = dota_path.join("dota").join("maps").join("dota.vpk");
        let target_path = dota_path.join("dota").join("maps").join(&vpk_file);
        let out_path = dota_path
            .join("dota_tempcontent")
            .join("maps")
            .join("dota.vpk");

        // Catch panics from vpk.rs (it uses unwrap/panic for invalid VPKs)
        let result = std::panic::catch_unwind(|| vpk::create_terrain(base_path, target_path));

        match result {
            Ok(out_file) => {
                std::fs::create_dir_all(out_path.parent().unwrap())
                    .map_err(|e| format!("Failed to create output directory: {e}"))?;
                std::fs::write(&out_path, &out_file)
                    .map_err(|e| format!("Failed to write VPK: {e}"))?;
                Ok(())
            }
            Err(_) => Err("Failed to parse VPK file. It may be corrupted.".to_string()),
        }
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
