//! Tauri commands exposed to the frontend.

use tauri::{AppHandle, Manager, WebviewWindow};
use serde::{Deserialize, Serialize};

use crate::platform;

#[derive(Debug, Serialize)]
pub struct PlatformInfo {
    pub platform: platform::Platform,
    pub arch: String,
    pub os: String,
}

#[tauri::command]
pub fn get_platform() -> PlatformInfo {
    PlatformInfo {
        platform: platform::current_platform(),
        arch: std::env::consts::ARCH.to_string(),
        os: std::env::consts::OS.to_string(),
    }
}

#[derive(Debug, Deserialize)]
pub struct SpawnInput {
    /// Shell command to execute (e.g., "echo hello" or "ls -la")
    pub command: String,
}

#[derive(Debug, Serialize)]
pub struct RunOutput {
    pub status: i32,
    pub stdout: String,
    pub stderr: String,
}

#[tauri::command]
pub fn spawn(input: SpawnInput) -> Result<(), String> {
    platform::spawn_shell(&input.command).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn run(input: SpawnInput) -> Result<RunOutput, String> {
    platform::run_shell_capture(&input.command)
        .map(|(status, stdout, stderr)| RunOutput { status, stdout, stderr })
        .map_err(|e| e.to_string())
}

#[derive(Debug, Deserialize)]
pub struct OpenInput {
    /// URL (https://…) or file path
    pub target: String,
}

#[tauri::command]
pub fn open_path_or_url(input: OpenInput) -> Result<(), String> {
    platform::open_system(&input.target).map_err(|e| e.to_string())
}

#[derive(Debug, Serialize)]
pub struct Paths {
    /// e.g., "com.quack.app"
    pub identifier: String,
    pub config_dir: String,
    pub data_dir: String,
}

#[tauri::command]
pub fn get_paths(app: AppHandle) -> Result<Paths, String> {
    let identifier = app.config().identifier.clone();
    let config_dir = platform::app_config_dir(&identifier)
        .map_err(|e| e.to_string())?;
    let data_dir = platform::app_data_dir(&identifier)
        .map_err(|e| e.to_string())?;
    Ok(Paths {
        identifier,
        config_dir: config_dir.display().to_string(),
        data_dir: data_dir.display().to_string(),
    })
}

/// ----- Window controls -----

#[derive(Debug, Deserialize)]
pub struct WindowFlag {
    pub label: Option<String>, // default to "main" if None
    pub value: bool,
}

fn get_window<'a>(app: &'a AppHandle, label: Option<String>) -> Result<WebviewWindow, String> {
    let lbl = label.unwrap_or_else(|| "main".to_string());
    app.get_webview_window(&lbl)
        .ok_or_else(|| format!("window '{lbl}' not found"))
}

#[tauri::command]
pub fn window_set_always_on_top(app: AppHandle, payload: WindowFlag) -> Result<(), String> {
    let w = get_window(&app, payload.label)?;
    platform::set_always_on_top(&w, payload.value).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn window_set_decorations(app: AppHandle, payload: WindowFlag) -> Result<(), String> {
    let w = get_window(&app, payload.label)?;
    platform::set_decorations(&w, payload.value).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn window_set_shadow(app: AppHandle, payload: WindowFlag) -> Result<(), String> {
    let w = get_window(&app, payload.label)?;
    platform::set_shadow(&w, payload.value).map_err(|e| e.to_string())
}

#[derive(Debug, Deserialize)]
pub struct WindowSize {
    pub label: Option<String>,
    pub width: f64,
    pub height: f64,
}

#[tauri::command]
pub fn window_resize(app: AppHandle, payload: WindowSize) -> Result<(), String> {
    let w = get_window(&app, payload.label)?;
    platform::resize(&w, payload.width, payload.height).map_err(|e| e.to_string())
}

#[derive(Debug, Deserialize)]
pub struct WindowPosition {
    pub label: Option<String>,
    pub x: f64,
    pub y: f64,
}

#[tauri::command]
pub fn window_move(app: AppHandle, payload: WindowPosition) -> Result<(), String> {
    let w = get_window(&app, payload.label)?;
    platform::move_window(&w, payload.x, payload.y).map_err(|e| e.to_string())
}

/// Quit the application
#[tauri::command]
pub fn quit_app(app: AppHandle) {
    app.exit(0);
}
