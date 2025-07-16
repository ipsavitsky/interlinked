use shared::NewRecord;

#[tokio::main]
async fn main() {
    let new_record = NewRecord {
        payload: "hello world".to_string(),
        challange: "asdf".to_string()
    };

    let resp = reqwest::Client::new().post("http://localhost:3000").json(&new_record).send().await.unwrap();

    println!("{:?}", resp.text().await.unwrap());
}
