#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use catalyst_app::app;

pub fn main() {
    app::launch();
}
