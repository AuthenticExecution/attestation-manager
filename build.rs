use std::env;

fn main() {
    let platform = env::var("MANAGER_PLATFORM").unwrap_or("native".to_string());
    println!("cargo:rerun-if-env-changed=MANAGER_PLATFORM");
    println!("cargo:rustc-cfg=platform=\"{}\"", platform);
}
