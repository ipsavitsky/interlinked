use anyhow::Result;
use clap::{Parser, Subcommand};
use shared::{NewRecordScheme, come_up_with_solution};
use url::Url;
use std::time::SystemTime;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    verbose: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New { link: String },
    Resolve { id: String },
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

async fn new(payload_str: String) -> Result<()> {
    let difficulty = get_difficulty().await.expect("Could not query difficulty");
    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Could not seed rng")
        .as_secs();
    let (challenge, _) = come_up_with_solution(difficulty, seed);

    let payload = Url::parse(&payload_str)?;

    let new_record = NewRecordScheme { payload, challenge };

    let data = reqwest::Client::new()
        .post("http://localhost:3000")
        .json(&new_record)
        .send()
        .await?
        .text()
        .await?;

    println!("http://localhost:3000/{data}");
    Ok(())
}

async fn resolve(id: String) -> Result<()> {
    let data = reqwest::Client::new()
        .get(format!("http://localhost:3000/{id}"))
        .send()
        .await?
        .text()
        .await?;

    println!("{data}");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()>{
    let args = Args::parse();

    match args.command {
        Commands::New { link } => new(link).await,
        Commands::Resolve { id } => resolve(id).await,
    }
}
