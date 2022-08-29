use tauri::{self, Manager};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            
            match app.get_window("main-window") {
                Some(window) => {

                },
                _ => (),
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
