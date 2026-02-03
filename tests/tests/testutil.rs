use std::{
    process::Output,
    sync::{Arc, Mutex},
};

use tokio::process::{Child, Command};

pub async fn start_server() -> Arc<Mutex<Child>> {
    let server_binary_path = std::env::var("ITLKD_SERVER_PATH").expect("ITLKD_SERVER_PATH not set");
    if std::fs::exists("interlinked.db").expect("could not check for database") {
        std::fs::remove_file("interlinked.db").expect("could not remove test database");
    }

    if std::fs::exists("objects").expect("could not check for object dir") {
        std::fs::remove_dir_all("objects").expect("could not remove test object dir");
    }

    let mut handle = Command::new(server_binary_path)
        .env("INTERLINKED_DB_URL", "interlinked.db")
        .spawn()
        .expect("failed to start interlinked server");

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("checking...");
    match handle.try_wait() {
        Ok(Some(e)) => panic!("The server process has exited with status: {e}"),
        Ok(None) => return Arc::new(Mutex::new(handle)),
        Err(e) => panic!("Could not query the status of the server: {e}"),
    };
}

pub fn check_code(output: &Output) {
    if !output.status.success() {
        println!("CLI stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("CLI stderr: {}", String::from_utf8_lossy(&output.stderr));
        panic!("interlinked cli exited with error code {}", output.status);
    }
}

pub async fn run_cli(command: &[&str]) -> Output {
    let project_root = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
    let cli_binary_path = std::env::var("ITLKD_CLI_PATH").expect("ITLKD_CLI_PATH not set");
    let config_path = std::path::PathBuf::from(project_root).join("tests/cli_config.toml");
    let cli_query = Command::new(cli_binary_path)
        .arg("-c")
        .arg(&config_path)
        .current_dir(project_root)
        .args(command)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("failed to start interlinked cli");

    let output_query = cli_query
        .wait_with_output()
        .await
        .expect("failed to wait for interlinked cli");

    check_code(&output_query);
    output_query
}
