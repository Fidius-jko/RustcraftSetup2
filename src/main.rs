// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rustcraft::gen_app;
use rustcraft::OSType;

fn main() {
    #[cfg(target_os = "linux")]
    gen_app(OSType::Linux).run();
    #[cfg(target_os = "windows")]
    gen_app(OSType::Windows).run();
    #[cfg(target_os = "macos")]
    gen_app(OSType::Macos).run();
}
