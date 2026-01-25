use anyhow::Result;
use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use indicatif::ProgressBar;
use shared::{NewNoteScheme, NewRecordScheme, come_up_with_solution};
use std::time::{Duration, SystemTime};
use std::{env, fs};
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
    New {
        #[command(subcommand)]
        subcommand: PayloadType,
    },
    Resolve {
        #[command(subcommand)]
        subcommand: RequestType,
    },
}

#[derive(Subcommand)]
enum PayloadType {
    Link { link: String },
    Note { filename: String },
}

#[derive(Subcommand)]
enum RequestType {
    Link { id: u32 },
    Note { id: u32 },
}

async fn get_difficulty(backend_url: &str) -> Result<usize> {
    Ok(reqwest::Client::new()
        .get(format!("{}/api/difficulty", backend_url))
        .send()
        .await?
        .text()
        .await?
        .parse()?)
}

async fn calculate_hash(difficulty: usize) -> Result<String> {
    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();
    println!("Current difficulty: {difficulty}");
    let spinner = ProgressBar::new_spinner().with_message("Calculating hash...");
    spinner.enable_steady_tick(Duration::from_millis(100));
    let (challenge, _) = come_up_with_solution(difficulty, seed);
    spinner.finish();
    println!("challenge: {challenge}");
    Ok(challenge)
}

async fn new_link(payload_str: String, backend_url: &str) -> Result<()> {
    let difficulty = get_difficulty(backend_url).await?;
    let challenge = calculate_hash(difficulty).await?;
    let payload = Url::parse(&payload_str)?;
    let new_record = NewRecordScheme { payload, challenge };
    let post_url = format!("{backend_url}/links");
    let data = reqwest::Client::new()
        .post(&post_url)
        .json(&new_record)
        .send()
        .await?
        .text()
        .await?;
    println!("Short link: {post_url}/{data}");
    Ok(())
}

async fn new_note(filename: String, backend_url: &str) -> Result<()> {
    let difficulty = get_difficulty(backend_url).await?;
    let challenge = calculate_hash(difficulty).await?;
    let payload = fs::read_to_string(filename)?;
    let new_record = NewNoteScheme { payload, challenge };
    let post_url = format!("{backend_url}/notes");
    let data = reqwest::Client::new()
        .post(&post_url)
        .json(&new_record)
        .send()
        .await?
        .text()
        .await?;
    println!("Short link: {post_url}/{data}");
    Ok(())
}

async fn resolve_link(id: u32, backend_url: &str) -> Result<()> {
    let data = reqwest::Client::new()
        .get(format!("{backend_url}/links/{id}"))
        .send()
        .await?
        .text()
        .await?;

    println!("{data}");
    Ok(())
}

async fn resolve_note(id: u32, backend_url: &str) -> Result<()> {
    let data = reqwest::Client::new()
        .get(format!("{backend_url}/notes/{id}"))
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
        Commands::New {
            subcommand: PayloadType::Link { link },
        } => new_link(link, &backend_url).await,
        Commands::New {
            subcommand: PayloadType::Note { filename },
        } => new_note(filename, &backend_url).await,
        Commands::Resolve {
            subcommand: RequestType::Link { id },
        } => resolve_link(id, &backend_url).await,
        Commands::Resolve {
            subcommand: RequestType::Note { id },
        } => resolve_note(id, &backend_url).await,
    }
}
