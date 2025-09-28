use anyhow::Result;
use pingtest::network::SpeedTest;
use pingtest::ping::PingAnalyzer;
use pingtest::history::{HistoryManager, HistoryEntry};
use pingtest::config::Config;
use pingtest::ui::ThemeManager;
use pingtest::utils::{format_speed, format_ping, get_quality_score};
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 PingTest Advanced Usage Example");
    println!("===================================\n");

    // Load configuration
    println!("⚙️  Loading configuration...");
    let config = Config::load().await?;
    println!("Default theme: {}", config.general.theme);
    println!("Default duration: {} seconds", config.general.default_duration);
    println!("Default connections: {}", config.general.default_connections);
    println!();

    // Initialize history manager
    println!("📚 Initializing history manager...");
    let history_manager = HistoryManager::new().await?;
    println!();

    // Test different themes
    println!("🎨 Testing themes...");
    let mut theme_manager = ThemeManager::new();
    let themes = theme_manager.get_available_themes();
    
    for theme in &themes {
        theme_manager.set_theme(theme)?;
        println!("  ✓ Theme '{}' loaded", theme);
    }
    println!();

    // Run multiple ping tests with different targets
    println!("📡 Running ping tests to different targets...");
    let ping_analyzer = PingAnalyzer::new();
    
    let targets = vec!["8.8.8.8", "1.1.1.1", "208.67.222.222"];
    for target in targets {
        println!("  Testing {}...", target);
        let result = ping_analyzer.run_ping_test_with_params(target, 5, std::time::Duration::from_secs(2)).await?;
        println!("    Average ping: {}", format_ping(result.avg_ping));
        println!("    Jitter: {}", format_ping(result.jitter));
        println!("    Packet loss: {:.1}%", result.packet_loss);
    }
    println!();

    // Run speed test with different parameters
    println!("🌐 Running speed tests with different parameters...");
    let speed_test = SpeedTest::new();
    
    // Test with different durations
    let durations = vec![5, 10, 15];
    for duration in durations {
        println!("  Testing with {} second duration...", duration);
        let result = speed_test.run_test(duration, 4, false, false).await?;
        println!("    Download: {}", format_speed(result.download_speed));
        println!("    Upload: {}", format_speed(result.upload_speed));
        println!("    Quality Score: {}/100", get_quality_score(result.download_speed, result.upload_speed, result.ping));
    }
    println!();

    // Test with different connection counts
    println!("🔗 Testing with different connection counts...");
    let connections = vec![2, 4, 8];
    for conn_count in connections {
        println!("  Testing with {} connections...", conn_count);
        let result = speed_test.run_test(10, conn_count, false, false).await?;
        println!("    Download: {}", format_speed(result.download_speed));
        println!("    Upload: {}", format_speed(result.upload_speed));
    }
    println!();

    // Test download-only and upload-only tests
    println!("⬇️  Testing download-only...");
    let download_result = speed_test.run_test(10, 4, false, true).await?;
    println!("  Download: {}", format_speed(download_result.download_speed));
    println!("  Upload: {} (skipped)", format_speed(download_result.upload_speed));
    
    println!("⬆️  Testing upload-only...");
    let upload_result = speed_test.run_test(10, 4, true, false).await?;
    println!("  Download: {} (skipped)", format_speed(upload_result.download_speed));
    println!("  Upload: {}", format_speed(upload_result.upload_speed));
    println!();

    // Get server information
    println!("🖥️  Getting server information...");
    let servers = speed_test.get_servers().await?;
    println!("Available servers:");
    for server in &servers {
        println!("  {} - {} ({}) - {:.1} km", 
                 server.id, server.name, server.location, server.distance);
    }
    println!();

    // Get nearest servers
    println!("📍 Getting nearest servers...");
    let nearest = speed_test.get_nearest_servers(3).await?;
    println!("Nearest servers:");
    for server in &nearest {
        println!("  {} - {} ({}) - {:.1} km", 
                 server.id, server.name, server.location, server.distance);
    }
    println!();

    // Save test results to history
    println!("💾 Saving test results to history...");
    let test_result = speed_test.run_test(10, 4, false, false).await?;
    
    let history_entry = HistoryEntry {
        id: uuid::Uuid::new_v4().to_string(),
        timestamp: Utc::now(),
        download_speed: test_result.download_speed,
        upload_speed: test_result.upload_speed,
        ping: test_result.ping,
        server_id: test_result.server_id,
        server_name: test_result.server_name.clone(),
        server_location: test_result.server_location.clone(),
        tag: Some("advanced_example".to_string()),
    };
    
    history_manager.add_entry(history_entry).await?;
    println!("  ✓ Test result saved to history");
    println!();

    // Get history statistics
    println!("📊 Getting history statistics...");
    let stats = history_manager.get_statistics(30).await?;
    println!("Statistics for last 30 days:");
    println!("  Total tests: {}", stats.total_tests);
    println!("  Average download: {}", format_speed(stats.avg_download));
    println!("  Average upload: {}", format_speed(stats.avg_upload));
    println!("  Average ping: {}", format_ping(stats.avg_ping));
    println!("  Best download: {}", format_speed(stats.best_download));
    println!("  Best upload: {}", format_speed(stats.best_upload));
    println!("  Best ping: {}", format_ping(stats.best_ping));
    println!();

    // Export history
    println!("📤 Exporting history...");
    history_manager.export_history("pingtest_history.json").await?;
    println!("  ✓ History exported to pingtest_history.json");
    println!();

    // Get trends
    println!("📈 Getting trends...");
    let download_trends = history_manager.get_trends("download").await?;
    println!("Download trends (last {} days):", download_trends.len());
    for trend in download_trends.iter().take(5) {
        println!("  {}: {}", trend.date.format("%Y-%m-%d"), format_speed(trend.value));
    }
    println!();

    println!("✅ Advanced usage example completed successfully!");

    Ok(())
}