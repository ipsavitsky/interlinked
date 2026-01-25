use anyhow::Result;
use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use indicatif::ProgressBar;
use serde::Serialize;
use shared::{NewNoteScheme, NewRecordScheme, RecordPayload, come_up_with_solution};
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

async fn create_record<T: RecordPayload + Serialize>(payload: T, backend_url: &str) -> Result<()> {
    let difficulty = get_difficulty(backend_url).await?;
    let challenge = calculate_hash(difficulty).await?;
    let record = payload.with_challenge(challenge);
    let post_url = format!("{}/{}", backend_url, record.record_type());
    let data = reqwest::Client::new()
        .post(&post_url)
        .json(&record)
        .send()
        .await?
        .text()
        .await?;
    println!("Short link: {post_url}/{data}");
    Ok(())
}

async fn resolve_record(id: u32, record_type: &str, backend_url: &str) -> Result<()> {
    let data = reqwest::Client::new()
        .get(format!("{}/{}/{}", backend_url, record_type, id))
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
        } => {
            let payload = Url::parse(&link)?;
            let record = NewRecordScheme {
                payload,
                challenge: String::new(),
            };
            create_record(record, &backend_url).await
        }
        Commands::New {
            subcommand: PayloadType::Note { filename },
        } => {
            let payload = fs::read_to_string(filename)?;
            let record = NewNoteScheme {
                payload,
                challenge: String::new(),
            };
            create_record(record, &backend_url).await
        }
        Commands::Resolve {
            subcommand: RequestType::Link { id },
        } => resolve_record(id, "links", &backend_url).await,
        Commands::Resolve {
            subcommand: RequestType::Note { id },
        } => resolve_record(id, "notes", &backend_url).await,
    }
}
