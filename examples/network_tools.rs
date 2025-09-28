use anyhow::Result;
use pingtest::tools::{
    diagnostics::NetworkDiagnostics,
    bandwidth_monitor::BandwidthMonitor,
    traceroute::Traceroute,
    dns_resolver::DnsResolver,
    port_scanner::PortScanner,
    network_scanner::NetworkScanner,
    wifi_analyzer::WifiAnalyzer,
    quality_analyzer::QualityAnalyzer,
    statistics_collector::StatisticsCollector,
};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔧 PingTest Network Tools Demo");
    println!("==============================\n");

    // Network Diagnostics
    println!("🔍 Running Network Diagnostics...");
    let diagnostics = NetworkDiagnostics::new();
    let diagnostic_results = diagnostics.run_full_diagnostics().await?;
    
    println!("Diagnostic Results:");
    for result in diagnostic_results {
        let status_icon = match result.status {
            pingtest::tools::diagnostics::DiagnosticStatus::Pass => "✅",
            pingtest::tools::diagnostics::DiagnosticStatus::Warning => "⚠️",
            pingtest::tools::diagnostics::DiagnosticStatus::Fail => "❌",
            pingtest::tools::diagnostics::DiagnosticStatus::Info => "ℹ️",
        };
        
        println!("  {} {}: {}", status_icon, result.test_name, result.details);
        
        if !result.recommendations.is_empty() {
            for recommendation in &result.recommendations {
                println!("    💡 {}", recommendation);
            }
        }
    }
    println!();

    // Bandwidth Monitoring
    println!("📊 Starting Bandwidth Monitoring...");
    let mut bandwidth_monitor = BandwidthMonitor::new("eth0")
        .with_interval(Duration::from_secs(2))
        .with_history_size(10);
    
    let bandwidth_history = bandwidth_monitor.monitor_for_duration(Duration::from_secs(10)).await?;
    
    println!("Bandwidth Statistics:");
    println!("  Average Download: {:.2} Mbps", bandwidth_history.avg_download_mbps);
    println!("  Average Upload: {:.2} Mbps", bandwidth_history.avg_upload_mbps);
    println!("  Peak Download: {:.2} Mbps", bandwidth_history.peak_download_mbps);
    println!("  Peak Upload: {:.2} Mbps", bandwidth_history.peak_upload_mbps);
    println!("  Total Data Transferred: {}", pingtest::utils::format_bytes(bandwidth_history.total_bytes_transferred));
    println!();

    // Traceroute
    println!("🛤️  Running Traceroute...");
    let traceroute = Traceroute::new()
        .with_max_hops(15)
        .with_timeout(Duration::from_secs(3));
    
    let trace_result = traceroute.trace("8.8.8.8").await?;
    
    println!("Traceroute to {}:", trace_result.target);
    println!("  Total hops: {}", trace_result.total_hops);
    println!("  Successful hops: {}", trace_result.successful_hops);
    println!("  Average RTT: {:.1}ms", trace_result.average_rtt_ms);
    println!("  Path complete: {}", trace_result.path_complete);
    
    println!("  Route:");
    for hop in &trace_result.hops[..5] { // Show first 5 hops
        if hop.is_timeout {
            println!("    {} * * * timeout", hop.hop_number);
        } else {
            let hostname = hop.hostname.as_ref().map(|h| h.as_str()).unwrap_or("");
            println!("    {} {} ({}) {:.1}ms", 
                     hop.hop_number, 
                     hop.ip_address.map(|ip| ip.to_string()).unwrap_or("*".to_string()),
                     hostname,
                     hop.avg_rtt_ms);
        }
    }
    println!();

    // DNS Resolution
    println!("🌐 Testing DNS Resolution...");
    let dns_resolver = DnsResolver::new();
    
    let domains = vec!["google.com", "github.com", "cloudflare.com"];
    let dns_results = dns_resolver.bulk_resolve(domains, pingtest::tools::dns_resolver::DnsRecordType::A).await?;
    
    println!("DNS Resolution Results:");
    for result in dns_results {
        if result.error.is_none() {
            println!("  {}: {} ({:.1}ms)", 
                     result.query, 
                     result.records.first().map(|r| &r.value).unwrap_or("N/A"),
                     result.response_time_ms);
        } else {
            println!("  {}: Error - {}", result.query, result.error.as_ref().unwrap());
        }
    }
    println!();

    // Port Scanning
    println!("🔍 Scanning Common Ports...");
    let port_scanner = PortScanner::new()
        .with_timeout(Duration::from_secs(2))
        .with_max_threads(10);
    
    let port_scan_result = port_scanner.scan_common_ports("127.0.0.1".parse()?).await?;
    
    println!("Port Scan Results for {}:", port_scan_result.target);
    println!("  Ports scanned: {}", port_scan_result.ports_scanned);
    println!("  Open ports: {}", port_scan_result.open_ports.len());
    println!("  Closed ports: {}", port_scan_result.closed_ports.len());
    println!("  Filtered ports: {}", port_scan_result.filtered_ports.len());
    
    if !port_scan_result.open_ports.is_empty() {
        println!("  Open Ports:");
        for port_result in &port_scan_result.open_ports {
            let service = port_result.service.as_ref().map(|s| s.as_str()).unwrap_or("Unknown");
            println!("    {}: {} ({:.1}ms)", port_result.port, service, port_result.response_time_ms);
        }
    }
    println!();

    // Network Scanning
    println!("🌐 Scanning Local Network...");
    let network_scanner = NetworkScanner::new()
        .with_timeout(Duration::from_secs(1))
        .with_max_threads(20);
    
    let network_scan_result = network_scanner.scan_network("192.168.1.1-192.168.1.10", pingtest::tools::network_scanner::ScanType::Ping).await?;
    
    println!("Network Scan Results:");
    println!("  Network range: {}", network_scan_result.network_range);
    println!("  Hosts scanned: {}", network_scan_result.hosts_scanned);
    println!("  Alive hosts: {}", network_scan_result.alive_hosts.len());
    
    if !network_scan_result.alive_hosts.is_empty() {
        println!("  Alive Hosts:");
        for host in &network_scan_result.alive_hosts[..3] { // Show first 3 hosts
            let hostname = host.hostname.as_ref().map(|h| h.as_str()).unwrap_or("");
            let vendor = host.vendor.as_ref().map(|v| v.as_str()).unwrap_or("");
            println!("    {} ({}) - {} - {:.1}ms", 
                     host.target_ip, hostname, vendor, host.response_time_ms);
        }
    }
    println!();

    // WiFi Analysis
    println!("📶 Analyzing WiFi Networks...");
    let wifi_analyzer = WifiAnalyzer::new("wlan0");
    
    let wifi_scan_result = wifi_analyzer.scan_networks(pingtest::tools::wifi_analyzer::WifiScanType::Comprehensive).await?;
    
    println!("WiFi Scan Results:");
    println!("  Interface: {}", wifi_scan_result.interface_name);
    println!("  Networks found: {}", wifi_scan_result.networks_found);
    println!("  Scan duration: {:.2}s", wifi_scan_result.total_duration.as_secs_f64());
    
    if !wifi_scan_result.networks.is_empty() {
        println!("  Networks:");
        for network in &wifi_scan_result.networks[..3] { // Show first 3 networks
            let vendor = network.vendor.as_ref().map(|v| v.as_str()).unwrap_or("");
            println!("    {} ({}) - Ch{} - {}% - {:?}", 
                     network.ssid, network.bssid, network.channel,
                     network.signal_percentage, network.security);
        }
    }
    println!();

    // Network Quality Analysis
    println!("📈 Analyzing Network Quality...");
    let quality_analyzer = QualityAnalyzer::new()
        .with_test_duration(Duration::from_secs(15))
        .with_test_interval(Duration::from_secs(3));
    
    let quality_analysis = quality_analyzer.analyze_network_quality().await?;
    
    println!("Network Quality Analysis:");
    println!("  Tests performed: {}", quality_analysis.test_count);
    println!("  Overall assessment: {:?}", quality_analysis.overall_assessment);
    println!("  Quality trend: {:?}", quality_analysis.quality_trend);
    println!("  Average quality score: {}/100", quality_analysis.average_metrics.overall_quality_score);
    println!("  Average latency: {:.1}ms", quality_analysis.average_metrics.latency_ms);
    println!("  Average bandwidth: {:.1} Mbps", quality_analysis.average_metrics.bandwidth_download_mbps);
    
    if !quality_analysis.network_issues.is_empty() {
        println!("  Issues detected: {}", quality_analysis.network_issues.len());
        for issue in &quality_analysis.network_issues[..2] { // Show first 2 issues
            println!("    {:?} ({:?}): {}", issue.issue_type, issue.severity, issue.description);
        }
    }
    
    if !quality_analysis.recommendations.is_empty() {
        println!("  Recommendations:");
        for (i, recommendation) in quality_analysis.recommendations.iter().take(3).enumerate() {
            println!("    {}. {}", i + 1, recommendation);
        }
    }
    println!();

    // Network Statistics Collection
    println!("📊 Collecting Network Statistics...");
    let mut stats_collector = StatisticsCollector::new()
        .with_interval(Duration::from_secs(2))
        .with_max_history(5);
    
    let comprehensive_stats = stats_collector.collect_comprehensive_stats().await?;
    
    println!("Network Statistics:");
    println!("  Interface: {}", comprehensive_stats.network.interface_name);
    println!("  Download: {:.2} Mbps", comprehensive_stats.network.bandwidth_download_mbps);
    println!("  Upload: {:.2} Mbps", comprehensive_stats.network.bandwidth_received_mbps);
    println!("  Packets sent: {}", comprehensive_stats.network.packets_sent);
    println!("  Packets received: {}", comprehensive_stats.network.packets_received);
    println!("  Errors: {}", comprehensive_stats.network.errors_sent + comprehensive_stats.network.errors_received);
    
    println!("System Statistics:");
    println!("  CPU usage: {:.1}%", comprehensive_stats.system.cpu_usage_percent);
    println!("  Memory usage: {:.1}%", comprehensive_stats.system.memory_usage_percent);
    println!("  Load average: {:.2}", comprehensive_stats.system.load_average);
    println!("  Processes: {}", comprehensive_stats.system.processes_count);
    
    println!("Connection Statistics:");
    println!("  TCP connections: {}", comprehensive_stats.connections.tcp_connections);
    println!("  UDP connections: {}", comprehensive_stats.connections.udp_connections);
    println!("  Established: {}", comprehensive_stats.connections.established_connections);
    println!("  Listening ports: {}", comprehensive_stats.connections.listening_ports);
    println!();

    println!("✅ Network tools demonstration completed successfully!");

    Ok(())
}