use crate::state;
use tauri::{FileDropEvent, Window};

pub fn handle_drop_file_event(window: &Window, drop: &FileDropEvent) {
    match drop {
        FileDropEvent::Hovered(_) => {
            if !state::get_hover() {
                // 首次触发 drop file 的时候，发送一个事件给前台
                window.emit("file-drop", 123).expect("event failed");
                state::file_drop_start();
            }
        }
        FileDropEvent::Dropped(paths) => {
            state::file_drop_end();
        }
        FileDropEvent::Cancelled => {
            state::file_drop_end();
        }
        _ => (),
    }
}
