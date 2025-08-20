//! Cross-platform helpers for OS, paths, shell, and window tweaks.
//! Linux-friendly (tested with Fedora deps used by Tauri).
//!
//! Add these crates in `Cargo.toml` if you don’t have them yet:
//!   open = "5"
//!   serde = { version = "1", features = ["derive"] }
//!   thiserror = "1"
//!   dirs = "5"

use std::process::Command;
use std::path::PathBuf;
use thiserror::Error;

use tauri::WebviewWindow;

#[derive(Debug, Error)]
pub enum PlatformError {
    #[error("{0}")]
    Msg(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone, Copy, serde::Serialize)]
pub enum Platform {
    Windows,
    Linux,
    MacOs,
    Unknown,
}

pub fn current_platform() -> Platform {
    #[cfg(target_os = "windows")]
    { Platform::Windows }
    #[cfg(target_os = "linux")]
    { Platform::Linux }
    #[cfg(target_os = "macos")]
    { Platform::MacOs }
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    { Platform::Unknown }
}

/// Open a URL or file using the system default app (cross-platform).
pub fn open_system(url_or_path: &str) -> Result<(), PlatformError> {
    open::that(url_or_path)
        .map_err(|e| PlatformError::Msg(format!("Failed to open '{}': {e}", url_or_path)))
}

/// Return a per-app config directory.
pub fn app_config_dir(identifier: &str) -> Result<PathBuf, PlatformError> {
    let base = dirs::config_dir()
        .ok_or_else(|| PlatformError::Msg("Could not resolve config directory".into()))?;
    Ok(base.join(identifier))
}

/// Return a per-app data directory.
pub fn app_data_dir(identifier: &str) -> Result<PathBuf, PlatformError> {
    let base = dirs::data_dir()
        .ok_or_else(|| PlatformError::Msg("Could not resolve data directory".into()))?;
    Ok(base.join(identifier))
}

/// Spawn a shell command non-blocking.
pub fn spawn_shell(command: &str) -> Result<(), PlatformError> {
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd").args(["/C", command]).spawn()?;
        Ok(())
    }

    #[cfg(any(target_os = "linux", target_os = "macos", not(target_os = "windows")))]
    {
        Command::new("sh").arg("-c").arg(command).spawn()?;
        Ok(())
    }
}

/// Run a command and capture stdout/stderr (blocking).
pub fn run_shell_capture(command: &str) -> Result<(i32, String, String), PlatformError> {
    #[cfg(target_os = "windows")]
    let output = Command::new("cmd").args(["/C", command]).output()?;

    #[cfg(any(target_os = "linux", target_os = "macos", not(target_os = "windows")))]
    let output = Command::new("sh").arg("-c").arg(command).output()?;

    let code = output.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    Ok((code, stdout, stderr))
}

/// Window helpers (all via Tauri v2’s WebviewWindow API)

pub fn set_always_on_top(window: &WebviewWindow, enabled: bool) -> Result<(), PlatformError> {
    window.set_always_on_top(enabled)
        .map_err(|e| PlatformError::Msg(format!("set_always_on_top failed: {e}")))
}

pub fn set_decorations(window: &WebviewWindow, enabled: bool) -> Result<(), PlatformError> {
    window.set_decorations(enabled)
        .map_err(|e| PlatformError::Msg(format!("set_decorations failed: {e}")))
}

pub fn set_shadow(window: &WebviewWindow, enabled: bool) -> Result<(), PlatformError> {
    window.set_shadow(enabled)
        .map_err(|e| PlatformError::Msg(format!("set_shadow failed: {e}")))
}

pub fn resize(window: &WebviewWindow, width: f64, height: f64) -> Result<(), PlatformError> {
    use tauri::PhysicalSize;
    window
        .set_size(PhysicalSize::new(width, height))
        .map_err(|e| PlatformError::Msg(format!("resize failed: {e}")))
}

pub fn move_window(window: &WebviewWindow, x: f64, y: f64) -> Result<(), PlatformError> {
    use tauri::PhysicalPosition;
    window
        .set_position(PhysicalPosition::new(x, y))
        .map_err(|e| PlatformError::Msg(format!("move failed: {e}")))
}
