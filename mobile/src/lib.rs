use bevy::prelude::*;

#[bevy_main]
fn main() {
    #[cfg(target_os = "android")]
    rustcraft::gen_app(rustcraft::OSType::Android);
    #[cfg(target_os = "ios")]
    rustcraft::gen_app(rustcraft::OSType::Ios);
}
