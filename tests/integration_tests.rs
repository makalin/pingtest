use pingtest::network::{SpeedTest, TestResult};
use pingtest::ping::{PingAnalyzer, PingResult};
use pingtest::utils::{format_speed, format_ping, calculate_percentage, get_quality_score};
use tokio;

#[tokio::test]
async fn test_speed_test_basic() {
    let speed_test = SpeedTest::new();
    
    // Test with minimal parameters
    let result = speed_test.run_test(5, 2, false, false).await;
    assert!(result.is_ok());
    
    let test_result = result.unwrap();
    assert!(test_result.download_speed >= 0.0);
    assert!(test_result.upload_speed >= 0.0);
    assert!(test_result.ping > 0.0);
    assert!(!test_result.server_name.is_empty());
}

#[tokio::test]
async fn test_speed_test_download_only() {
    let speed_test = SpeedTest::new();
    
    let result = speed_test.run_test(5, 2, false, true).await;
    assert!(result.is_ok());
    
    let test_result = result.unwrap();
    assert!(test_result.download_speed > 0.0);
    assert_eq!(test_result.upload_speed, 0.0);
}

#[tokio::test]
async fn test_speed_test_upload_only() {
    let speed_test = SpeedTest::new();
    
    let result = speed_test.run_test(5, 2, true, false).await;
    assert!(result.is_ok());
    
    let test_result = result.unwrap();
    assert_eq!(test_result.download_speed, 0.0);
    assert!(test_result.upload_speed > 0.0);
}

#[tokio::test]
async fn test_ping_analyzer_basic() {
    let analyzer = PingAnalyzer::new();
    
    let result = analyzer.run_ping_test().await;
    assert!(result.is_ok());
    
    let ping_result = result.unwrap();
    assert!(ping_result.avg_ping > 0.0);
    assert!(ping_result.min_ping > 0.0);
    assert!(ping_result.max_ping > 0.0);
    assert!(ping_result.jitter >= 0.0);
    assert!(ping_result.packet_loss >= 0.0);
    assert!(ping_result.packet_loss <= 100.0);
}

#[tokio::test]
async fn test_ping_analyzer_custom_params() {
    let analyzer = PingAnalyzer::new();
    
    let result = analyzer.run_ping_test_with_params("8.8.8.8", 5, std::time::Duration::from_secs(2)).await;
    assert!(result.is_ok());
    
    let ping_result = result.unwrap();
    assert_eq!(ping_result.packet_count, 5);
    assert!(ping_result.avg_ping > 0.0);
}

#[tokio::test]
async fn test_server_selection() {
    let speed_test = SpeedTest::new();
    
    let servers = speed_test.get_servers().await;
    assert!(servers.is_ok());
    
    let server_list = servers.unwrap();
    assert!(!server_list.is_empty());
    
    // Test getting nearest servers
    let nearest = speed_test.get_nearest_servers(3).await;
    assert!(nearest.is_ok());
    
    let nearest_servers = nearest.unwrap();
    assert!(nearest_servers.len() <= 3);
}

#[test]
fn test_format_speed() {
    assert_eq!(format_speed(0.5), "500.00 Kbps");
    assert_eq!(format_speed(50.0), "50.00 Mbps");
    assert_eq!(format_speed(1500.0), "1.5 Gbps");
}

#[test]
fn test_format_ping() {
    assert_eq!(format_ping(0.5), "500.0 μs");
    assert_eq!(format_ping(25.0), "25.0 ms");
}

#[test]
fn test_calculate_percentage() {
    assert_eq!(calculate_percentage(50.0, 100.0), 50.0);
    assert_eq!(calculate_percentage(25.0, 100.0), 25.0);
    assert_eq!(calculate_percentage(0.0, 100.0), 0.0);
    assert_eq!(calculate_percentage(100.0, 0.0), 0.0);
}

#[test]
fn test_quality_score() {
    let score = get_quality_score(50.0, 20.0, 25.0);
    assert!(score >= 0 && score <= 100);
    
    // Test with excellent conditions
    let excellent_score = get_quality_score(100.0, 50.0, 10.0);
    assert!(excellent_score > 80);
    
    // Test with poor conditions
    let poor_score = get_quality_score(5.0, 2.0, 200.0);
    assert!(poor_score < 50);
}

#[tokio::test]
async fn test_history_management() {
    use pingtest::history::{HistoryManager, HistoryEntry};
    use chrono::Utc;
    
    let history_manager = HistoryManager::new().await.unwrap();
    
    // Create a test entry
    let entry = HistoryEntry {
        id: "test-123".to_string(),
        timestamp: Utc::now(),
        download_speed: 50.0,
        upload_speed: 20.0,
        ping: 25.0,
        server_id: 12345,
        server_name: "Test Server".to_string(),
        server_location: "Test Location".to_string(),
        tag: Some("test".to_string()),
    };
    
    // Add entry
    let result = history_manager.add_entry(entry.clone()).await;
    assert!(result.is_ok());
    
    // Get history
    let history = history_manager.get_history().await.unwrap();
    assert!(!history.is_empty());
    
    // Get statistics
    let stats = history_manager.get_statistics(30).await.unwrap();
    assert!(stats.total_tests > 0);
    assert!(stats.avg_download > 0.0);
    
    // Clear history
    let clear_result = history_manager.clear_history().await;
    assert!(clear_result.is_ok());
}

#[tokio::test]
async fn test_config_management() {
    use pingtest::config::Config;
    
    // Test loading default config
    let config = Config::load().await;
    assert!(config.is_ok());
    
    let config = config.unwrap();
    assert_eq!(config.general.theme, "auto");
    assert_eq!(config.general.default_duration, 15);
    assert_eq!(config.general.default_connections, 4);
    
    // Test saving config
    let save_result = config.save().await;
    assert!(save_result.is_ok());
}

#[test]
fn test_theme_management() {
    use pingtest::ui::{ThemeManager, Theme};
    
    let mut theme_manager = ThemeManager::new();
    
    // Test available themes
    let themes = theme_manager.get_available_themes();
    assert!(themes.contains(&"dracula".to_string()));
    assert!(themes.contains(&"nord".to_string()));
    assert!(themes.contains(&"auto".to_string()));
    
    // Test setting theme
    let result = theme_manager.set_theme("dracula");
    assert!(result.is_ok());
    assert_eq!(theme_manager.get_current_theme(), "dracula");
    
    // Test invalid theme
    let invalid_result = theme_manager.set_theme("invalid_theme");
    assert!(invalid_result.is_err());
}

#[test]
fn test_ui_components() {
    use pingtest::ui::UiManager;
    
    let ui_manager = UiManager::new();
    
    // Test that UI manager can be created
    assert!(true); // Basic test that it doesn't panic
}

#[tokio::test]
async fn test_ping_statistics() {
    use pingtest::ping::PingAnalyzer;
    
    let analyzer = PingAnalyzer::new();
    let ping_times = vec![10.0, 15.0, 12.0, 18.0, 11.0];
    
    let stats = analyzer.calculate_statistics(&ping_times);
    
    assert_eq!(stats.count, 5);
    assert!(stats.mean > 0.0);
    assert!(stats.median > 0.0);
    assert_eq!(stats.min, 10.0);
    assert_eq!(stats.max, 18.0);
    assert!(stats.std_dev >= 0.0);
    assert!(stats.jitter >= 0.0);
}

#[test]
fn test_utility_functions() {
    use pingtest::utils::{validate_server_id, validate_duration, validate_connections, sanitize_filename};
    
    // Test server ID validation
    assert!(validate_server_id(12345));
    assert!(!validate_server_id(0));
    assert!(!validate_server_id(1000000));
    
    // Test duration validation
    assert!(validate_duration(15));
    assert!(validate_duration(5));
    assert!(validate_duration(300));
    assert!(!validate_duration(4));
    assert!(!validate_duration(301));
    
    // Test connections validation
    assert!(validate_connections(4));
    assert!(validate_connections(1));
    assert!(validate_connections(16));
    assert!(!validate_connections(0));
    assert!(!validate_connections(17));
    
    // Test filename sanitization
    assert_eq!(sanitize_filename("test file.txt"), "test_file.txt");
    assert_eq!(sanitize_filename("test@#$file"), "test___file");
}