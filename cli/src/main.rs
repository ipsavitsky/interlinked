use anyhow::Result;
use clap::{Parser, Subcommand};
use shared::{NewRecordScheme, come_up_with_solution};

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    New {
        link: String
    },
    Resolve {
        id: String
    }
}

async fn get_difficulty() -> Result<usize> {
    Ok(reqwest::Client::new()
        .get("http://localhost:3000/difficulty")
        .send()
        .await?
        .text()
        .await?
        .parse()?)
}

async fn new(payload: String) {
    let difficulty = get_difficulty().await.expect("Could not query difficulty");
    let (challenge, _) = come_up_with_solution(difficulty);

    let new_record = NewRecordScheme {
        payload,
        challenge,
    };

    let data = reqwest::Client::new()
        .post("http://localhost:3000")
        .json(&new_record)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{data}");
}

async fn resolve(id: String) {
    let data = reqwest::Client::new()
        .get(format!("http://localhost:3000/{id}"))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{data}");
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.command {
        Commands::New { link } => new(link).await,
        Commands::Resolve { id } => resolve(id).await,
    }
}
