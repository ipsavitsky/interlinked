use anyhow::Result;
use rand::{Rng, distr::Alphanumeric};
use shared::{NewRecordScheme, get_hash};

async fn get_difficulty() -> Result<usize> {
    Ok(reqwest::Client::new()
        .get("http://localhost:3000/difficulty")
        .send()
        .await?
        .text()
        .await?
        .parse()?)
}

fn make_random_string() -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(70)
        .map(char::from)
        .collect()
}

async fn come_up_with_solution() -> Result<String> {
    let difficulty = get_difficulty().await?;
    println!("Solving challenge with difficulty {difficulty}");
    let prefix_string = "0".repeat(difficulty);
    loop {
        let attempt = make_random_string();
        if get_hash(&attempt).starts_with(&prefix_string) {
            return Ok(attempt);
        }
    }
}

#[tokio::main]
async fn main() {
    let challenge = come_up_with_solution()
        .await
        .expect("Could not get challenge difficulty from server");

    println!("solved challenge! {challenge}: {}", get_hash(&challenge));

    let new_record = NewRecordScheme {
        payload: "hello world".to_string(),
        challenge,
    };

    let resp = reqwest::Client::new()
        .post("http://localhost:3000")
        .json(&new_record)
        .send()
        .await
        .unwrap();

    println!("{:?}", resp.text().await.unwrap());
}
