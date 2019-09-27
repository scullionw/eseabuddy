#![windows_subsystem = "windows"]

use binpack::*;

#[derive(Bundle)]
#[folder = "dist/"]
struct Python;

fn main() {
    Python::run();
}
