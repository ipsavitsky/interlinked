use rand::{Rng, distr::Alphanumeric};
use shared::{NewRecord, get_hash};

async fn get_difficulty() -> usize {
    reqwest::Client::new()
        .get("http://localhost:3000/difficulty")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
        .parse()
        .unwrap()
}

fn make_random_string() -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(70)
        .map(char::from)
        .collect()
}

async fn come_up_with_solution() -> String {
    let difficulty = get_difficulty().await;
    let prefix_string = "0".repeat(difficulty);
    loop {
        let attempt = make_random_string();
        if get_hash(&attempt).starts_with(&prefix_string) {
            return attempt;
        }
    }
}

#[tokio::main]
async fn main() {
    let challange = come_up_with_solution().await;

    println!("solved challange! {challange}: {}", get_hash(&challange));

    let new_record = NewRecord {
        payload: "hello world".to_string(),
        challange,
    };

    let resp = reqwest::Client::new()
        .post("http://localhost:3000")
        .json(&new_record)
        .send()
        .await
        .unwrap();

    println!("{:?}", resp.text().await.unwrap());
}
