use anyhow::Result;
use clap::{Parser, Subcommand};
use indicatif::ProgressBar;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use shared::{NewNoteScheme, NewRecordScheme, RecordPayload, come_up_with_solution};
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use url::Url;

#[derive(Deserialize)]
struct Config {
    backend_url: Url,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            backend_url: Url::parse("http://localhost:3000").unwrap(),
        }
    }
}

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    verbose: bool,
    #[arg(short, long)]
    config: Option<PathBuf>,
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

async fn get_difficulty(backend_url: &Url) -> Result<usize> {
    Ok(Client::new()
        .get(backend_url.join("/api/difficulty")?)
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

async fn create_record<T: RecordPayload + Serialize>(payload: T, backend_url: &Url) -> Result<()> {
    let difficulty = get_difficulty(backend_url).await?;
    let challenge = calculate_hash(difficulty).await?;
    let record = payload.with_challenge(challenge);
    let post_url = backend_url.join(record.record_type())?;
    let data = Client::new()
        .post(post_url.as_str())
        .json(&record)
        .send()
        .await?
        .text()
        .await?;
    println!("Short link: {post_url}/{data}");
    Ok(())
}

async fn resolve_record(id: u32, record_type: &str, backend_url: &Url) -> Result<()> {
    let data = Client::new()
        .get(backend_url.join(&format!("{record_type}/{id}"))?)
        .send()
        .await?
        .text()
        .await?;

    println!("{data}");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let conf = match args.config {
        Some(path) => toml::from_str(&std::fs::read_to_string(path)?)?,
        None => Config::default(),
    };

    match args.command {
        Commands::New {
            subcommand: PayloadType::Link { link },
        } => {
            let payload = Url::parse(&link)?;
            let record = NewRecordScheme {
                payload,
                challenge: String::new(),
            };
            create_record(record, &conf.backend_url).await
        }
        Commands::New {
            subcommand: PayloadType::Note { filename },
        } => {
            let payload = fs::read_to_string(filename)?;
            let record = NewNoteScheme {
                payload,
                challenge: String::new(),
            };
            create_record(record, &conf.backend_url).await
        }
        Commands::Resolve {
            subcommand: RequestType::Link { id },
        } => resolve_record(id, "links", &conf.backend_url).await,
        Commands::Resolve {
            subcommand: RequestType::Note { id },
        } => resolve_record(id, "notes", &conf.backend_url).await,
    }
}
