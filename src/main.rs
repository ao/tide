mod banner;
mod requests;

use clap::Parser;
use colored::*;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::signal;
use tokio::sync::Mutex;
use tokio::time::interval;
use url::Url;
use thiserror::Error;

use banner::banner;
use requests::{RequestMetrics, make_request_with_retry};

#[derive(Parser)]
#[command(name = "tide")]
#[command(about = "A concurrent HTTP load testing tool")]
#[command(version = "1.0")]
struct Args {
    /// Target URL (required)
    #[arg(long, value_name = "URL")]
    url: String,

    /// Number of concurrent requests per interval (must be > 0)
    #[arg(short = 'n', long, default_value = "5")]
    concurrency: u32,

    /// Duration for which the program should run (in seconds)
    #[arg(short = 't', long, default_value = "10")]
    duration: u64,

    /// Timeout for each HTTP request (in seconds)
    #[arg(long, default_value = "10")]
    timeout: u64,

    /// Number of retries for failed requests (>= 0)
    #[arg(long, default_value = "2")]
    retries: u32,
}

use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
struct Config {
    url: String,
    concurrency: u32,
    duration: u64,
    timeout: u64,
    retries: u32,
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    // Check for custom config path from environment variable
    let config_path_str = std::env::var("TIDE_CONFIG").unwrap_or_else(|_| "config.toml".to_string());
    let config_path = Path::new(&config_path_str);
    
    if !config_path.exists() {
        return Err(format!("Config file not found: {}", config_path_str).into());
    }
    
    let config_content = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_content)?;
    
    Ok(config)
}

#[derive(Debug, Error)]
enum AppError {
    #[error("Argument error: {0}")]
    ArgumentError(String),
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Signal error: {0}")]
    SignalError(#[from] tokio::io::Error),
}

fn validate_args(args: &Args) -> Result<(), AppError> {
    if args.url.trim().is_empty() {
        return Err(AppError::ArgumentError("Target URL is required".to_string()));
    }

    if Url::parse(&args.url).is_err() {
        return Err(AppError::ArgumentError("Invalid target URL".to_string()));
    }

    if args.concurrency == 0 {
        return Err(AppError::ArgumentError("Concurrency must be > 0".to_string()));
    }

    if args.duration == 0 {
        return Err(AppError::ArgumentError("Duration must be > 0".to_string()));
    }

    if args.timeout == 0 {
        return Err(AppError::ArgumentError("Timeout must be > 0".to_string()));
    }

    Ok(())
}

fn create_separator(label_width: usize, value_width: usize) -> String {
    format!(
        "+{}+{}+",
        "-".repeat(label_width + 2),
        "-".repeat(value_width + 2)
    )
}

async fn print_summary_report(
    target_url: &str,
    concurrency: u32,
    elapsed: Duration,
    total_requests: u32,
    metrics: &RequestMetrics,
) {
    let successful_requests = *metrics.successful_requests.lock().await;
    let failed_requests = *metrics.failed_requests.lock().await;
    let request_times = metrics.request_times.lock().await;

    if request_times.is_empty() {
        println!(
            "\n{}No requests were completed. Please check your network or target URL.{}",
            "".red(),
            "".clear()
        );
        return;
    }

    let mut times = request_times.clone();
    times.sort();

    let min_time = times[0];
    let max_time = times[times.len() - 1];
    let median_time = times[times.len() / 2];
    let total_nanos: u128 = times.iter().map(|d| d.as_nanos()).sum();
    let avg_nanos = total_nanos / times.len() as u128;
    let avg_time = Duration::from_nanos(avg_nanos.min(u64::MAX as u128) as u64);

    let titles_width = 25;
    let mut max_width = 40;
    if target_url.len() > max_width {
        max_width = target_url.len();
    }
    let separator = create_separator(titles_width, max_width);

    println!("\n*** Summary Report ***");
    println!("{}", separator);
    println!(
        "| {:<width$} | {:<max_width$} |",
        "Target URL",
        target_url,
        width = titles_width,
        max_width = max_width
    );
    println!("{}", separator);
    println!(
        "| {:<width$} | {:<max_width$} |",
        "Concurrency",
        concurrency,
        width = titles_width,
        max_width = max_width
    );
    println!("{}", separator);
    println!(
        "| {:<width$} | {:<max_width$} |",
        "Duration",
        format!("{:.3}s", elapsed.as_secs_f64()),
        width = titles_width,
        max_width = max_width
    );
    println!("{}", separator);
    println!(
        "| {:<width$} | {:<max_width$} |",
        "Total Requests",
        total_requests,
        width = titles_width,
        max_width = max_width
    );
    println!("{}", separator);
    println!(
        "| {:<width$} | {:<max_width$} |",
        "Successful Requests",
        successful_requests,
        width = titles_width,
        max_width = max_width
    );
    println!("{}", separator);
    println!(
        "| {:<width$} | {:<max_width$} |",
        "Failed Requests",
        failed_requests,
        width = titles_width,
        max_width = max_width
    );
    println!("{}", separator);
    println!(
        "| {:<width$} | {:<max_width$} |",
        "Min Request Time",
        format!("{:.3}ms", min_time.as_secs_f64() * 1000.0),
        width = titles_width,
        max_width = max_width
    );
    println!("{}", separator);
    println!(
        "| {:<width$} | {:<max_width$} |",
        "Median Request Time",
        format!("{:.3}ms", median_time.as_secs_f64() * 1000.0),
        width = titles_width,
        max_width = max_width
    );
    println!("{}", separator);
    println!(
        "| {:<width$} | {:<max_width$} |",
        "Max Request Time",
        format!("{:.3}ms", max_time.as_secs_f64() * 1000.0),
        width = titles_width,
        max_width = max_width
    );
    println!("{}", separator);
    println!(
        "| {:<width$} | {:<max_width$} |",
        "Avg Request Time",
        format!("{:.3}ms", avg_time.as_secs_f64() * 1000.0),
        width = titles_width,
        max_width = max_width
    );
    println!("{}", separator);
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    println!("{}", banner());

    // Parse command-line arguments
    let args = Args::parse();
    validate_args(&args)?;

    // Try to load config file, use command-line args as fallback
    let config = match load_config() {
        Ok(c) => c,
        Err(e) => {
            println!("{}Warning: {}, using command-line arguments{}", "".yellow(), e, "".clear());
            // Convert args to config
            Config {
                url: args.url.clone(),
                concurrency: args.concurrency,
                duration: args.duration,
                timeout: args.timeout,
                retries: args.retries,
            }
        }
    };

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(config.timeout))
        .build()
        .map_err(AppError::RequestError)?;

    let metrics = RequestMetrics::new();
    let total_requests = Arc::new(Mutex::new(0u32));

    println!(
        "Running for {}s with concurrency={}, timeout={}s, retries={}\n",
        config.duration, config.concurrency, config.timeout, config.retries
    );

    let start_time = Instant::now();
    let duration = Duration::from_secs(config.duration);
    let timeout_duration = Duration::from_secs(config.timeout);

    // Set up graceful shutdown
    let shutdown_signal = async {
        match signal::ctrl_c().await {
            Ok(_) => {},
            Err(e) => eprintln!("{}Shutdown signal error: {}{}", "".red(), e, "".clear()),
        }
    };

    // Main execution loop
    let execution = async {
        let mut ticker = interval(Duration::from_secs(1));

        loop {
            let elapsed = start_time.elapsed();
            if elapsed >= duration {
                break;
            }

            let remaining = duration - elapsed;
            println!(
                "\nTime elapsed: {}s - Time remaining: {}s",
                elapsed.as_secs(),
                remaining.as_secs()
            );

            // Launch concurrent requests for this interval
            let mut handles = Vec::new();

            for _ in 0..config.concurrency {
                let client = client.clone();
                let url = config.url.clone();
                let metrics = RequestMetrics {
                    successful_requests: metrics.successful_requests.clone(),
                    failed_requests: metrics.failed_requests.clone(),
                    request_times: metrics.request_times.clone(),
                };
                let total_requests = total_requests.clone();

                let handle = tokio::spawn(async move {
                    {
                        let mut total = total_requests.lock().await;
                        *total += 1;
                    }
                    let result = make_request_with_retry(
                        &client,
                        &url,
                        timeout_duration,
                        config.retries,
                        &metrics,
                    )
                    .await;

                    if let Err(e) = result {
                        eprintln!("{}Request failed: {}{}", "".red(), e, "".clear());
                    }
                });

                handles.push(handle);
            }

            // Wait for all requests in this interval to complete
            for handle in handles {
                let _ = handle.await;
            }

            ticker.tick().await;
        }
    };

    // Run with graceful shutdown
    tokio::select! {
        _ = execution => {},
        _ = shutdown_signal => {},
    }

    let elapsed = start_time.elapsed();
    let total_requests_count = *total_requests.lock().await;

    print_summary_report(
        &config.url,
        config.concurrency,
        elapsed,
        total_requests_count,
        &metrics,
    )
    .await;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_validate_args_valid() {
        let args = Args {
            url: "https://example.com".to_string(),
            concurrency: 5,
            duration: 10,
            timeout: 5,
            retries: 2,
        };
        
        let result = validate_args(&args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_args_empty_url() {
        let args = Args {
            url: "".to_string(),
            concurrency: 5,
            duration: 10,
            timeout: 5,
            retries: 2,
        };
        
        let result = validate_args(&args);
        assert!(result.is_err());
        match result {
            Err(AppError::ArgumentError(msg)) => {
                assert_eq!(msg, "Target URL is required");
            }
            _ => panic!("Expected ArgumentError"),
        }
    }

    #[test]
    fn test_validate_args_invalid_url() {
        let args = Args {
            url: "not-a-valid-url".to_string(),
            concurrency: 5,
            duration: 10,
            timeout: 5,
            retries: 2,
        };
        
        let result = validate_args(&args);
        assert!(result.is_err());
        match result {
            Err(AppError::ArgumentError(msg)) => {
                assert_eq!(msg, "Invalid target URL");
            }
            _ => panic!("Expected ArgumentError"),
        }
    }

    #[test]
    fn test_validate_args_zero_concurrency() {
        let args = Args {
            url: "https://example.com".to_string(),
            concurrency: 0,
            duration: 10,
            timeout: 5,
            retries: 2,
        };
        
        let result = validate_args(&args);
        assert!(result.is_err());
        match result {
            Err(AppError::ArgumentError(msg)) => {
                assert_eq!(msg, "Concurrency must be > 0");
            }
            _ => panic!("Expected ArgumentError"),
        }
    }

    #[test]
    fn test_validate_args_zero_duration() {
        let args = Args {
            url: "https://example.com".to_string(),
            concurrency: 5,
            duration: 0,
            timeout: 5,
            retries: 2,
        };
        
        let result = validate_args(&args);
        assert!(result.is_err());
        match result {
            Err(AppError::ArgumentError(msg)) => {
                assert_eq!(msg, "Duration must be > 0");
            }
            _ => panic!("Expected ArgumentError"),
        }
    }

    #[test]
    fn test_validate_args_zero_timeout() {
        let args = Args {
            url: "https://example.com".to_string(),
            concurrency: 5,
            duration: 10,
            timeout: 0,
            retries: 2,
        };
        
        let result = validate_args(&args);
        assert!(result.is_err());
        match result {
            Err(AppError::ArgumentError(msg)) => {
                assert_eq!(msg, "Timeout must be > 0");
            }
            _ => panic!("Expected ArgumentError"),
        }
    }

    #[test]
    fn test_create_separator() {
        let separator = create_separator(10, 20);
        assert_eq!(separator, "+------------+----------------------+");
    }

    #[test]
    fn test_load_config_with_temp_file() {
        // Create a temporary directory
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.toml");
        
        // Create a config file
        let mut file = File::create(&config_path).unwrap();
        writeln!(file, "url = \"https://example.com\"").unwrap();
        writeln!(file, "concurrency = 5").unwrap();
        writeln!(file, "duration = 10").unwrap();
        writeln!(file, "timeout = 5").unwrap();
        writeln!(file, "retries = 2").unwrap();
        
        // For this example, we'll just verify the config format is correct
        let config_content = std::fs::read_to_string(&config_path).unwrap();
        let config: Config = toml::from_str(&config_content).unwrap();
        
        assert_eq!(config.url, "https://example.com");
        assert_eq!(config.concurrency, 5);
        assert_eq!(config.duration, 10);
        assert_eq!(config.timeout, 5);
        assert_eq!(config.retries, 2);
    }
}
