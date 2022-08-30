use tauri::{self, Manager};

fn main() {
    // build application
    let app = tauri::Builder::default()
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    // appliaction run handler;
    app.run(|app, _event| {
        match app.get_window("main-window") {
            Some(_window) => {
            },
            _ => (),
        }
    })
}
