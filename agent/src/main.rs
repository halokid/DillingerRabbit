use clap::Parser;
use reqwest::Client;
use serde::Deserialize;
use std::process;

#[derive(Parser)]
#[command(author, version, about = "CI/CD Agent for running pipeline actions")]
struct Args {
    /// Action to perform: presync, build, unitest, deploy, verify
    #[arg(short, long)]
    action: String,

    /// Optional path for the operation
    #[arg(short, long)]
    path: Option<String>,

    /// Server URL, defaults to http://localhost:3000
    #[arg(short = 'u', long, default_value = "http://localhost:3000")]
    server_url: String,
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    success: bool,
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Validate action
    let valid_actions = ["presync", "build", "unitest", "deploy", "verify"];
    if !valid_actions.contains(&args.action.as_str()) {
        eprintln!("Invalid action '{}'. Must be one of: {}", args.action, valid_actions.join(", "));
        process::exit(1);
    }

    // Prepare request
    let url = format!("{}/{}", args.server_url, args.action);
    let request_body = if let Some(path) = args.path.clone() {
        serde_json::json!({"path": path})
    } else {
        serde_json::json!({})
    };

    // Debug output
    // println!("URL: {}", url);
    // println!("Body: {}", request_body);

    // Create HTTP client and send request
    let client = Client::new();
    let resp = match client.post(&url).json(&request_body).send().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Connection failed: {}", e);
            process::exit(1);
        }
    };

    // Handle response
    if resp.status().is_success() {
        let res: ApiResponse = resp.json().await?;
        if res.success {
            println!("{}", res.message);
        } else {
            eprintln!("{}", res.message);
            process::exit(1);
        }
    } else {
        eprintln!("HTTP error {}: {}", resp.status(), resp.text().await?);
        process::exit(1);
    }

    Ok(())
}
