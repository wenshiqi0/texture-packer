use lazy_static::*;

use std::sync::Mutex;

#[derive(Debug, Clone, Copy)]
struct AppStatus {
    hovering: bool,
}

impl AppStatus {
    pub fn file_drop_start(&mut self) {
        self.hovering = true;
    }

    pub fn file_drop_end(&mut self) {
        self.hovering = false;
    }
}

lazy_static! {
    static ref GLOBAL_STATE: Mutex<AppStatus> = Mutex::new(AppStatus {
        hovering: false,
    });
}

pub fn file_drop_start() {
    GLOBAL_STATE.lock().unwrap().file_drop_start();
}

pub fn file_drop_end() {
    GLOBAL_STATE.lock().unwrap().file_drop_end();
}

pub fn get_hover() -> bool {
    return GLOBAL_STATE.lock().unwrap().hovering;
}
