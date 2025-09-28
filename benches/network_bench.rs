use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pingtest::network::SpeedTest;
use pingtest::ping::PingAnalyzer;
use pingtest::utils::{format_speed, format_ping, calculate_percentage, get_quality_score};
use tokio::runtime::Runtime;

fn bench_speed_test(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("speed_test_5s", |b| {
        b.to_async(&rt).iter(|| async {
            let speed_test = SpeedTest::new();
            speed_test.run_test(black_box(5), black_box(4), false, false).await
        })
    });
    
    c.bench_function("speed_test_10s", |b| {
        b.to_async(&rt).iter(|| async {
            let speed_test = SpeedTest::new();
            speed_test.run_test(black_box(10), black_box(4), false, false).await
        })
    });
    
    c.bench_function("speed_test_download_only", |b| {
        b.to_async(&rt).iter(|| async {
            let speed_test = SpeedTest::new();
            speed_test.run_test(black_box(5), black_box(4), false, true).await
        })
    });
}

fn bench_ping_test(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("ping_test_10_packets", |b| {
        b.to_async(&rt).iter(|| async {
            let analyzer = PingAnalyzer::new();
            analyzer.run_ping_test_with_params(
                black_box("8.8.8.8"),
                black_box(10),
                black_box(std::time::Duration::from_secs(1))
            ).await
        })
    });
    
    c.bench_function("ping_test_20_packets", |b| {
        b.to_async(&rt).iter(|| async {
            let analyzer = PingAnalyzer::new();
            analyzer.run_ping_test_with_params(
                black_box("8.8.8.8"),
                black_box(20),
                black_box(std::time::Duration::from_secs(1))
            ).await
        })
    });
}

fn bench_utility_functions(c: &mut Criterion) {
    c.bench_function("format_speed", |b| {
        b.iter(|| {
            format_speed(black_box(50.0));
            format_speed(black_box(1500.0));
            format_speed(black_box(0.5));
        })
    });
    
    c.bench_function("format_ping", |b| {
        b.iter(|| {
            format_ping(black_box(25.0));
            format_ping(black_box(0.5));
            format_ping(black_box(150.0));
        })
    });
    
    c.bench_function("calculate_percentage", |b| {
        b.iter(|| {
            calculate_percentage(black_box(50.0), black_box(100.0));
            calculate_percentage(black_box(25.0), black_box(100.0));
            calculate_percentage(black_box(75.0), black_box(100.0));
        })
    });
    
    c.bench_function("get_quality_score", |b| {
        b.iter(|| {
            get_quality_score(black_box(50.0), black_box(20.0), black_box(25.0));
            get_quality_score(black_box(100.0), black_box(50.0), black_box(10.0));
            get_quality_score(black_box(5.0), black_box(2.0), black_box(200.0));
        })
    });
}

fn bench_server_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("get_servers", |b| {
        b.to_async(&rt).iter(|| async {
            let speed_test = SpeedTest::new();
            speed_test.get_servers().await
        })
    });
    
    c.bench_function("get_nearest_servers", |b| {
        b.to_async(&rt).iter(|| async {
            let speed_test = SpeedTest::new();
            speed_test.get_nearest_servers(black_box(5)).await
        })
    });
}

fn bench_ping_statistics(c: &mut Criterion) {
    use pingtest::ping::PingAnalyzer;
    
    let analyzer = PingAnalyzer::new();
    let ping_times: Vec<f64> = (0..100).map(|i| 10.0 + (i as f64 * 0.1)).collect();
    
    c.bench_function("calculate_statistics", |b| {
        b.iter(|| {
            analyzer.calculate_statistics(black_box(&ping_times));
        })
    });
}

fn bench_history_operations(c: &mut Criterion) {
    use pingtest::history::{HistoryManager, HistoryEntry};
    use chrono::Utc;
    
    let rt = Runtime::new().unwrap();
    
    c.bench_function("add_history_entry", |b| {
        b.to_async(&rt).iter(|| async {
            let history_manager = HistoryManager::new().await.unwrap();
            let entry = HistoryEntry {
                id: "bench-test".to_string(),
                timestamp: Utc::now(),
                download_speed: 50.0,
                upload_speed: 20.0,
                ping: 25.0,
                server_id: 12345,
                server_name: "Bench Server".to_string(),
                server_location: "Bench Location".to_string(),
                tag: Some("benchmark".to_string()),
            };
            history_manager.add_entry(entry).await
        })
    });
    
    c.bench_function("get_history_statistics", |b| {
        b.to_async(&rt).iter(|| async {
            let history_manager = HistoryManager::new().await.unwrap();
            history_manager.get_statistics(black_box(30)).await
        })
    });
}

fn bench_config_operations(c: &mut Criterion) {
    use pingtest::config::Config;
    
    let rt = Runtime::new().unwrap();
    
    c.bench_function("load_config", |b| {
        b.to_async(&rt).iter(|| async {
            Config::load().await
        })
    });
    
    c.bench_function("save_config", |b| {
        b.to_async(&rt).iter(|| async {
            let config = Config::default();
            config.save().await
        })
    });
}

fn bench_theme_operations(c: &mut Criterion) {
    use pingtest::ui::ThemeManager;
    
    c.bench_function("create_theme_manager", |b| {
        b.iter(|| {
            ThemeManager::new();
        })
    });
    
    c.bench_function("set_theme", |b| {
        b.iter(|| {
            let mut theme_manager = ThemeManager::new();
            theme_manager.set_theme(black_box("dracula"));
            theme_manager.set_theme(black_box("nord"));
            theme_manager.set_theme(black_box("auto"));
        })
    });
}

criterion_group!(
    benches,
    bench_speed_test,
    bench_ping_test,
    bench_utility_functions,
    bench_server_operations,
    bench_ping_statistics,
    bench_history_operations,
    bench_config_operations,
    bench_theme_operations
);

criterion_main!(benches);