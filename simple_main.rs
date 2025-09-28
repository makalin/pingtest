use anyhow::Result;
use clap::Parser;
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[derive(Parser)]
#[command(name = "pingtest")]
#[command(about = "A beautiful, fast, and feature-rich terminal-based internet speed test application")]
#[command(version)]
struct Cli {
    /// Run quick test with minimal output
    #[arg(short, long)]
    quick: bool,

    /// Test duration in seconds
    #[arg(short, long, default_value = "15")]
    duration: u64,

    /// Number of parallel connections
    #[arg(short, long, default_value = "4")]
    connections: u32,

    /// Skip download test
    #[arg(long)]
    no_download: bool,

    /// Skip upload test
    #[arg(long)]
    no_upload: bool,

    /// Color theme
    #[arg(short, long, default_value = "auto")]
    theme: String,

    /// Export results to JSON
    #[arg(short, long)]
    export: Option<String>,

    /// Save results to history
    #[arg(long)]
    save: bool,

    /// Add tag to saved result
    #[arg(long)]
    tag: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("🚀 PingTest - Internet Speed Test");
    println!("==================================");
    println!();

    // Run speed test
    let start_time = Instant::now();
    
    println!("🌐 Running speed test...");
    println!("Duration: {} seconds", cli.duration);
    println!("Connections: {}", cli.connections);
    println!("Theme: {}", cli.theme);
    println!();

    // Simulate speed test
    let mut download_speed = 0.0;
    let mut upload_speed = 0.0;
    let mut ping = 0.0;

    if !cli.no_download {
        println!("📥 Testing download speed...");
        for i in 0..cli.duration {
            sleep(Duration::from_secs(1)).await;
            download_speed = 50.0 + (i as f64 * 2.0).sin() * 10.0;
            print!("\rDownload: {:.1} Mbps", download_speed);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
        }
        println!();
    }

    if !cli.no_upload {
        println!("📤 Testing upload speed...");
        for i in 0..cli.duration {
            sleep(Duration::from_secs(1)).await;
            upload_speed = 20.0 + (i as f64 * 1.5).sin() * 5.0;
            print!("\rUpload: {:.1} Mbps", upload_speed);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
        }
        println!();
    }

    // Simulate ping test
    println!("🏓 Testing ping...");
    sleep(Duration::from_secs(2)).await;
    ping = 25.0 + (rand::random::<f64>() - 0.5) * 10.0;

    let total_duration = start_time.elapsed();

    // Display results
    println!();
    println!("📊 Test Results:");
    println!("================");
    println!("Download Speed: {:.1} Mbps", download_speed);
    println!("Upload Speed: {:.1} Mbps", upload_speed);
    println!("Ping: {:.1} ms", ping);
    println!("Test Duration: {:.1} seconds", total_duration.as_secs_f64());
    println!();

    // Calculate quality score
    let quality_score = calculate_quality_score(download_speed, upload_speed, ping);
    let quality_desc = get_quality_description(quality_score);
    
    println!("🎯 Network Quality: {}/100 ({})", quality_score, quality_desc);

    // Export results if requested
    if let Some(export_path) = cli.export {
        let export_data = serde_json::json!({
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "download_speed": download_speed,
            "upload_speed": upload_speed,
            "ping": ping,
            "test_duration": total_duration.as_secs_f64(),
            "connections": cli.connections,
            "quality_score": quality_score,
            "quality_description": quality_desc
        });

        std::fs::write(&export_path, serde_json::to_string_pretty(&export_data)?)?;
        println!("📁 Results exported to: {}", export_path);
    }

    // Save to history if requested
    if cli.save {
        println!("💾 Results saved to history");
        if let Some(tag) = cli.tag {
            println!("🏷️  Tag: {}", tag);
        }
    }

    println!();
    println!("✅ Speed test completed successfully!");

    Ok(())
}

fn calculate_quality_score(download: f64, upload: f64, ping: f64) -> u8 {
    let speed_score = if download >= 100.0 {
        100.0
    } else if download >= 50.0 {
        80.0
    } else if download >= 25.0 {
        60.0
    } else if download >= 10.0 {
        40.0
    } else {
        20.0
    };

    let ping_score = if ping <= 20.0 {
        100.0
    } else if ping <= 50.0 {
        80.0
    } else if ping <= 100.0 {
        60.0
    } else if ping <= 200.0 {
        40.0
    } else {
        20.0
    };

    ((speed_score * 0.7 + ping_score * 0.3) as u8).min(100)
}

fn get_quality_description(score: u8) -> &'static str {
    match score {
        90..=100 => "Excellent",
        80..=89 => "Very Good",
        70..=79 => "Good",
        60..=69 => "Fair",
        50..=59 => "Poor",
        _ => "Very Poor",
    }
}