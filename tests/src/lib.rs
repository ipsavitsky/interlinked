use std::sync::{Arc, Mutex};

use tokio::process::{Child, Command};

fn start_server(binary_path: String) -> Arc<Mutex<Child>> {
    Arc::new(Mutex::new(
        Command::new(binary_path)
            .spawn()
            .expect("failed to start interlinked server"),
    ))
}

#[tokio::test]
async fn test() {
    let server_binary_path = std::env::var("ITLKD_SERVER_PATH").expect("ITLKD_SERVER_PATH not set");
    let server = start_server(server_binary_path);
    let cli_binary_path = std::env::var("ITLKD_CLI_PATH").expect("ITLKD_CLI_PATH not set");

    let mut cli = Command::new(cli_binary_path)
        .arg("-c cli_config.toml")
        .args(["new", "link", "http://github.com"])
        .spawn()
        .expect("failed to start interlinked cli");

    let exit_code = cli
        .wait()
        .await
        .expect("failed to wait for interlinked cli");

    if !exit_code.success() {
        panic!("interlinked cli exited with error code {}", exit_code);
    }

    server
        .lock()
        .unwrap()
        .kill()
        .await
        .expect("failed to kill interlinked server");
}
