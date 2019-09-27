#![windows_subsystem = "windows"]

use binpack::*;

#[derive(Bundle)]
struct Python;

fn main() {
    Python::run_no_window();
}
