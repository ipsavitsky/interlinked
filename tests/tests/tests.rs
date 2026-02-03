use reqwest::Client;
use serial_test::serial;
use testutil::{run_cli, start_server};
mod testutil;

#[tokio::test]
#[serial]
async fn test_write_read_link() {
    let server = start_server().await;
    run_cli(&["new", "link", "http://github.com"]).await;
    let output = run_cli(&["resolve", "link", "1"]).await;
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "http://github.com/\n"
    );
    server
        .lock()
        .unwrap()
        .kill()
        .await
        .expect("failed to kill interlinked server");
}

#[tokio::test]
#[serial]
async fn test_write_link() {
    let server = start_server().await;
    run_cli(&["new", "link", "http://gitlab.com"]).await;
    let data = Client::new()
        .get("http://localhost:3000/link/1")
        .send()
        .await
        .expect("Could not query backend")
        .text()
        .await
        .expect("Could not extract text from response");
    assert_eq!(data, "http://gitlab.com/");
    server
        .lock()
        .unwrap()
        .kill()
        .await
        .expect("failed to kill interlinked server");
}

#[tokio::test]
#[serial]
async fn test_write_note() {
    let server = start_server().await;
    run_cli(&["new", "note", "tests/test/test_note"]).await;
    let data = Client::new()
        .get("http://localhost:3000/note/1")
        .send()
        .await
        .expect("Could not query backend")
        .text()
        .await
        .expect("Could not extract text from response");
    assert_eq!(data, "hello!\n");
    server
        .lock()
        .unwrap()
        .kill()
        .await
        .expect("failed to kill interlinked server");
}
