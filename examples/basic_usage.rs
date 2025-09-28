use anyhow::Result;
use pingtest::network::SpeedTest;
use pingtest::ping::PingAnalyzer;
use pingtest::utils::{format_speed, format_ping, get_quality_score};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 PingTest Basic Usage Example");
    println!("===============================\n");

    // Run a basic ping test
    println!("📡 Running ping test...");
    let ping_analyzer = PingAnalyzer::new();
    let ping_result = ping_analyzer.run_ping_test().await?;
    
    println!("Ping Results:");
    println!("  Average: {}", format_ping(ping_result.avg_ping));
    println!("  Min: {}", format_ping(ping_result.min_ping));
    println!("  Max: {}", format_ping(ping_result.max_ping));
    println!("  Jitter: {}", format_ping(ping_result.jitter));
    println!("  Packet Loss: {:.1}%", ping_result.packet_loss);
    println!();

    // Run a basic speed test
    println!("🌐 Running speed test...");
    let speed_test = SpeedTest::new();
    let test_result = speed_test.run_test(10, 4, false, false).await?;
    
    println!("Speed Test Results:");
    println!("  Download: {}", format_speed(test_result.download_speed));
    println!("  Upload: {}", format_speed(test_result.upload_speed));
    println!("  Ping: {}", format_ping(test_result.ping));
    println!("  Server: {} ({})", test_result.server_name, test_result.server_location);
    println!();

    // Calculate quality score
    let quality_score = get_quality_score(
        test_result.download_speed,
        test_result.upload_speed,
        test_result.ping,
    );
    
    println!("📊 Network Quality Score: {}/100", quality_score);
    
    // Determine quality description
    let quality_desc = match quality_score {
        90..=100 => "Excellent",
        80..=89 => "Very Good",
        70..=79 => "Good",
        60..=69 => "Fair",
        50..=59 => "Poor",
        _ => "Very Poor",
    };
    
    println!("Quality: {}", quality_desc);

    Ok(())
}