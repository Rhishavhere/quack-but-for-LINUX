#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod platform;
mod commands;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            commands::get_platform,
            commands::open_path_or_url,
            commands::spawn,
            commands::run,
            commands::get_paths,
            commands::window_set_always_on_top,
            commands::window_set_decorations,
            commands::window_set_shadow,
            commands::window_resize,
            commands::window_move,
            commands::quit_app,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
