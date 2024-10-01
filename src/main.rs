use clap::Parser;
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::{Mutex, Semaphore};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(name = "C99 Subdomain Finder")]
struct Args {
    #[arg(short, long, help = "API key for C99 Subdomain Finder")]
    apikey: String,

    #[arg(short, long, help = "Output file to save subdomains")]
    output: PathBuf,

    #[arg(
        short,
        long,
        default_value_t = 10,
        help = "Number of concurrent requests"
    )]
    concurrency: usize,

    #[arg(short, long, help = "File containing list of domains")]
    input: PathBuf,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct C99SubdomainResponse {
    success: bool,
    count: i32,
    requests_left: i32,
    expires: String,
    #[serde(default)]
    data: Vec<String>,
}

struct C99SubdomainFinder {
    client: Client,
    apikey: String,
    output: Arc<Mutex<File>>,
}

impl Clone for C99SubdomainFinder {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            apikey: self.apikey.clone(),
            output: self.output.clone(),
        }
    }
}

impl C99SubdomainFinder {
    async fn new(apikey: String, output: PathBuf) -> Result<Self, Box<dyn Error>> {
        let client = Client::builder()
            .user_agent("C99SubdomainFinder/1.0")
            .build()?;

        let output = File::create(output).await?;

        Ok(Self {
            client,
            apikey,
            output: Arc::new(Mutex::new(output)),
        })
    }

    async fn scan(&self, domain: &str) -> Result<(), Box<dyn Error>> {
        let url = format!(
            "https://worker.vktools.com/api/subdomainfinder.php?key={}&domain={}",
            self.apikey, domain
        );

        let response = self.client.get(&url).send().await?;
        if !response.status().is_success() {
            return Err(format!("API request failed with status: {}", response.status()).into());
        }

        let response_data: C99SubdomainResponse = serde_json::from_str(&response.text().await?)?;

        if !response_data.success {
            return Err("API request was not successful".into());
        }

        self.save_subdomains(&response_data.data).await?;
        println!(
            "Scan results for {}: {} subdomains found",
            domain, response_data.count
        );

        Ok(())
    }

    async fn save_subdomains(&self, subdomains: &[String]) -> Result<(), std::io::Error> {
        let mut output = self.output.lock().await;
        for subdomain in subdomains {
            output.write_all(subdomain.as_bytes()).await?;
            output.write_all(b"\n").await?;
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let finder = Arc::new(C99SubdomainFinder::new(args.apikey, args.output).await?);

    let semaphore = Arc::new(Semaphore::new(args.concurrency));
    let domains = tokio::fs::read_to_string(args.input).await?;

    let mut tasks = vec![];

    for domain in domains.lines() {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let finder = finder.clone();
        let domain = domain.to_string();

        let task = tokio::spawn(async move {
            let _permit = permit;
            if let Err(e) = finder.scan(&domain).await {
                eprintln!("Error scanning {}: {}", domain, e);
            }
        });

        tasks.push(task);
    }

    for task in tasks {
        task.await.map_err(|e| e.to_string())?;
    }

    Ok(())
}