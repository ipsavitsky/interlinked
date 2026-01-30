use std::env;
use std::path::PathBuf;

fn main() {
    let profile = env::var("PROFILE").unwrap();
    let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    path.push("../target");
    path.push(&profile);
    path.push(if cfg!(windows) {
        "itlkd-server.exe"
    } else {
        "itlkd-server"
    });
    println!("cargo:rustc-env=ITLKD_SERVER_PATH={}", path.display());

    let mut cli_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    cli_path.push("../target");
    cli_path.push(&profile);
    cli_path.push(if cfg!(windows) { "itlkd.exe" } else { "itlkd" });
    println!("cargo:rustc-env=ITLKD_CLI_PATH={}", cli_path.display());
}
