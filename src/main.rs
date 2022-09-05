mod file;
mod state;
mod server;

use tauri::{self, Manager};

fn main() {
    // setup udp server for files
    server::setup();

    // build application
    let app = tauri::Builder::default()
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    // appliaction run handler;
    app.run(|app, _event| {
        match app.get_window("main-window") {
            Some(window) => {
                // 由于 window 是监听侧，而函数内部也需要使用 window 实例，
                // 所以这个直接 clone 一个。
                let handler = window.clone();
                window.on_window_event(move |event| match event {
                    tauri::WindowEvent::Resized(_) => (),
                    tauri::WindowEvent::Moved(_) => (),
                    tauri::WindowEvent::CloseRequested { api, .. } => (),
                    tauri::WindowEvent::Destroyed => (),
                    tauri::WindowEvent::Focused(_) => (),
                    tauri::WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size, .. } => (),
                    tauri::WindowEvent::FileDrop(drop) => {
                        file::handle_drop_file_event(&handler, drop);
                    },
                    tauri::WindowEvent::ThemeChanged(_) => (),
                    _ => (),
                })
            },
            _ => (),
        }
    })
}
