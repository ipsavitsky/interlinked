use anyhow::Result;
use clap::{Parser, Subcommand};
use indicatif::ProgressBar;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use shared::{
    new_object_schemes::{NewLinkScheme, NewNoteScheme, RecordPayload},
    proof_of_work::solve_pow_challenge,
    requests::{create_record, fetch_difficulty},
};
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
    /// Path to a configuration file
    #[arg(short, long)]
    config: Option<PathBuf>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new resource in an interlinked instance
    New {
        #[command(subcommand)]
        subcommand: PayloadType,
    },
    /// Resolve an id into the resource content
    Resolve {
        #[command(subcommand)]
        subcommand: RequestType,
    },
}

#[derive(Subcommand)]
enum PayloadType {
    /// Create a new link
    Link { link: String },
    /// Create a new note
    Note { filename: String },
}

#[derive(Subcommand)]
enum RequestType {
    /// Request a link
    Link { id: u32 },
    /// Request a note
    Note { id: u32 },
}

async fn calculate_hash(difficulty: usize) -> Result<String> {
    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();
    println!("Current difficulty: {difficulty}");
    let spinner = ProgressBar::new_spinner().with_message("Calculating hash...");
    spinner.enable_steady_tick(Duration::from_millis(100));
    let (challenge, _) = solve_pow_challenge(difficulty, seed);
    spinner.finish();
    println!("challenge: {challenge}");
    Ok(challenge)
}

async fn write_record<T: RecordPayload + Serialize>(payload: T, backend_url: &Url) -> Result<()> {
    let index = create_record::<T>(backend_url, &payload).await?;
    println!(
        "Short link: {}{}/{}",
        backend_url,
        payload.record_type(),
        index
    );
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
        None => {
            let home = std::env::var("HOME")?;
            let conf_path = format!("{}/.config/interlinked/config.toml", home);
            if std::fs::exists(&conf_path)? {
                toml::from_str(&std::fs::read_to_string(conf_path)?)?
            } else {
                Config::default()
            }
        }
    };

    let difficulty = fetch_difficulty(&conf.backend_url).await?;

    match args.command {
        Commands::New {
            subcommand: PayloadType::Link { link },
        } => {
            let payload = Url::parse(&link)?;
            let record = NewLinkScheme {
                payload,
                challenge: calculate_hash(difficulty).await?,
            };
            write_record(record, &conf.backend_url).await
        }
        Commands::New {
            subcommand: PayloadType::Note { filename },
        } => {
            let payload = std::fs::read_to_string(filename)?;
            let record = NewNoteScheme {
                payload,
                challenge: calculate_hash(difficulty).await?,
            };
            write_record(record, &conf.backend_url).await
        }
        Commands::Resolve {
            subcommand: RequestType::Link { id },
        } => resolve_record(id, "link", &conf.backend_url).await,
        Commands::Resolve {
            subcommand: RequestType::Note { id },
        } => resolve_record(id, "note", &conf.backend_url).await,
    }
}
