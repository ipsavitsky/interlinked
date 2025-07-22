use anyhow::Result;
use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use shared::{NewRecordScheme, come_up_with_solution};
use std::env;
use std::time::SystemTime;
use url::Url;

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

async fn get_difficulty(backend_url: &str) -> Result<usize> {
    Ok(reqwest::Client::new()
        .get(format!("{}/difficulty", backend_url))
        .send()
        .await?
        .text()
        .await?
        .parse()?)
}

async fn new(payload_str: String, backend_url: &str) -> Result<()> {
    let difficulty = get_difficulty(backend_url)
        .await
        .expect("Could not query difficulty");
    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Could not seed rng")
        .as_secs();
    let (challenge, _) = come_up_with_solution(difficulty, seed);

    let payload = Url::parse(&payload_str)?;

    let new_record = NewRecordScheme { payload, challenge };

    let data = reqwest::Client::new()
        .post(backend_url)
        .json(&new_record)
        .send()
        .await?
        .text()
        .await?;

    println!("{backend_url}/{data}");
    Ok(())
}

async fn resolve(id: String, backend_url: &str) -> Result<()> {
    let data = reqwest::Client::new()
        .get(format!("{backend_url}/{id}"))
        .send()
        .await?
        .text()
        .await?;

    println!("{data}");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let backend_url = env::var("BACKEND_URL").unwrap_or("http://127.0.0.1:3000".to_string());
    let args = Args::parse();

    match args.command {
        Commands::New { link } => new(link, &backend_url).await,
        Commands::Resolve { id } => resolve(id, &backend_url).await,
    }
}
