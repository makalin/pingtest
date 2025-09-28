# PingTest Advanced Network Tools

PingTest now includes a comprehensive suite of advanced network analysis and diagnostic tools. These tools provide deep insights into network performance, security, and topology.

## 🛠️ Available Tools

### 1. Network Diagnostics (`diagnose`)
Comprehensive network health check that tests connectivity, DNS resolution, latency, bandwidth, and more.

```bash
# Run full diagnostics
pingtest --tools diagnose

# Export results to JSON
pingtest --tools diagnose --format json --output diagnostics.json

# Export results to CSV
pingtest --tools diagnose --format csv --output diagnostics.csv
```

**Tests Performed:**
- Internet connectivity
- DNS resolution
- Local network discovery
- Latency analysis
- Bandwidth testing
- Packet loss detection
- MTU discovery
- Firewall detection
- Proxy detection
- IP configuration analysis
- Routing table inspection
- Network interface enumeration

### 2. Bandwidth Monitor (`monitor`)
Real-time bandwidth usage monitoring with detailed statistics and export capabilities.

```bash
# Monitor for 60 seconds with 2-second intervals
pingtest --tools monitor --interface eth0 --duration 60 --interval 2

# Export to CSV
pingtest --tools monitor --export-csv bandwidth.csv

# Export to JSON
pingtest --tools monitor --export-json bandwidth.json
```

**Features:**
- Real-time bandwidth tracking
- Download/upload speed monitoring
- Packet statistics
- Error rate tracking
- Usage pattern analysis
- Historical data export

### 3. Traceroute (`trace`)
Advanced route tracing with detailed hop analysis and path optimization recommendations.

```bash
# Basic traceroute
pingtest --tools trace google.com

# Advanced traceroute with custom settings
pingtest --tools trace google.com --max-hops 20 --timeout 5

# Export results
pingtest --tools trace google.com --format json --output trace.json
```

**Features:**
- Multi-probe per hop
- Jitter calculation
- Packet loss detection
- Hostname resolution
- Path analysis
- Bottleneck identification
- Export to multiple formats

### 4. DNS Resolver (`dns`)
Comprehensive DNS testing and benchmarking with support for multiple record types.

```bash
# Resolve A record
pingtest --tools dns google.com --record-type A

# Resolve MX record
pingtest --tools dns example.com --record-type MX

# Benchmark DNS servers
pingtest --tools dns google.com --benchmark

# Use specific DNS server
pingtest --tools dns google.com --server 1.1.1.1
```

**Supported Record Types:**
- A (IPv4 addresses)
- AAAA (IPv6 addresses)
- CNAME (Canonical names)
- MX (Mail exchange)
- NS (Name servers)
- TXT (Text records)
- PTR (Reverse DNS)

### 5. Port Scanner (`scan-ports`)
Advanced port scanning with multiple scan types and comprehensive analysis.

```bash
# Scan common ports
pingtest --tools scan-ports 192.168.1.1 --ports common

# Scan specific range
pingtest --tools scan-ports 192.168.1.1 --ports 1-1000

# Service detection scan
pingtest --tools scan-ports 192.168.1.1 --scan-type service

# Banner grabbing
pingtest --tools scan-ports 192.168.1.1 --scan-type banner
```

**Scan Types:**
- TCP Connect
- TCP SYN
- UDP
- Service Detection
- Banner Grabbing

**Port Ranges:**
- `common` - Common services (21, 22, 23, 25, 53, 80, 443, etc.)
- `well-known` - Well-known ports (1-1023)
- `all` - All ports (1-65535)
- Custom range (e.g., `1-1000`)

### 6. Network Scanner (`scan-network`)
Comprehensive network host discovery with multiple scanning techniques.

```bash
# Ping scan
pingtest --tools scan-network 192.168.1.0/24 --scan-type ping

# ARP scan
pingtest --tools scan-network 192.168.1.0/24 --scan-type arp

# Comprehensive scan
pingtest --tools scan-network 192.168.1.0/24 --scan-type comprehensive

# Custom range
pingtest --tools scan-network 192.168.1.1-192.168.1.254
```

**Scan Types:**
- Ping - ICMP echo requests
- ARP - Address Resolution Protocol
- TCP - TCP connection attempts
- UDP - UDP packet probes
- Comprehensive - Combined approach

**Network Formats:**
- CIDR notation: `192.168.1.0/24`
- Range notation: `192.168.1.1-192.168.1.254`
- Single IP: `192.168.1.1`

### 7. WiFi Analyzer (`wifi`)
WiFi network discovery and analysis with security assessment.

```bash
# Comprehensive WiFi scan
pingtest --tools wifi --interface wlan0 --scan-type comprehensive

# Active scan
pingtest --tools wifi --scan-type active

# Passive scan with custom duration
pingtest --tools wifi --scan-type passive --duration 60
```

**Scan Types:**
- Active - Sends probe requests
- Passive - Listens for beacons
- Comprehensive - Combined approach

**Analysis Features:**
- Signal strength analysis
- Channel congestion detection
- Security assessment
- Vendor identification
- Encryption analysis

### 8. Network Quality Analyzer (`quality`)
Comprehensive network quality assessment with trend analysis.

```bash
# Basic quality analysis
pingtest --tools quality

# Extended analysis
pingtest --tools quality --duration 120 --interval 10

# Export results
pingtest --tools quality --format json --output quality.json
```

**Metrics Analyzed:**
- Latency and jitter
- Packet loss
- Bandwidth performance
- DNS resolution speed
- Connection stability
- Overall quality score

**Analysis Features:**
- Trend detection
- Issue identification
- Performance recommendations
- Quality grading

### 9. Statistics Collector (`stats`)
System and network statistics collection with comprehensive monitoring.

```bash
# Basic statistics collection
pingtest --tools stats --duration 60

# Export network stats
pingtest --tools stats --export-network network.csv

# Export system stats
pingtest --tools stats --export-system system.csv

# Export all stats
pingtest --tools stats --export-all stats.json
```

**Collected Statistics:**
- Network interface statistics
- System resource usage
- Connection statistics
- Performance metrics
- Historical trends

## 📊 Output Formats

All tools support multiple output formats:

### Text Format (Default)
Human-readable output with formatted tables and summaries.

### JSON Format
Structured data export for programmatic processing.

```bash
pingtest --tools diagnose --format json --output results.json
```

### CSV Format
Tabular data export for spreadsheet analysis.

```bash
pingtest --tools scan-ports 192.168.1.1 --format csv --output ports.csv
```

## 🔧 Advanced Usage

### Combining Tools
Tools can be combined for comprehensive network analysis:

```bash
# Full network assessment
pingtest --tools diagnose --format json --output diagnostics.json
pingtest --tools scan-network 192.168.1.0/24 --format json --output hosts.json
pingtest --tools quality --format json --output quality.json
```

### Automation
Tools can be integrated into scripts and automation workflows:

```bash
#!/bin/bash
# Network health check script

echo "Running network diagnostics..."
pingtest --tools diagnose --format json --output /var/log/network-diagnostics.json

echo "Scanning local network..."
pingtest --tools scan-network 192.168.1.0/24 --format csv --output /var/log/network-hosts.csv

echo "Analyzing network quality..."
pingtest --tools quality --duration 300 --format json --output /var/log/network-quality.json
```

### Performance Tuning
Tools can be tuned for different environments:

```bash
# Fast scan for large networks
pingtest --tools scan-network 10.0.0.0/16 --threads 200 --timeout 1

# Thorough scan for security assessment
pingtest --tools scan-ports target.com --ports all --threads 50 --timeout 5
```

## 🛡️ Security Considerations

### Network Scanning
- Always obtain proper authorization before scanning networks
- Use appropriate scan types to avoid overwhelming targets
- Respect rate limits and timeouts
- Consider legal implications of network scanning

### Data Export
- Be careful with sensitive network information in exports
- Use secure file permissions for output files
- Consider data retention policies

### Firewall Detection
- Tools may trigger security alerts
- Use appropriate scan techniques for different environments
- Consider stealth scanning options

## 📈 Performance Tips

### Optimization
- Use appropriate thread counts for your system
- Adjust timeouts based on network conditions
- Use targeted scans instead of broad sweeps when possible

### Resource Usage
- Monitor system resources during intensive scans
- Use appropriate intervals for continuous monitoring
- Consider network impact of active scanning

### Storage
- Export formats affect file sizes (JSON > CSV > Text)
- Consider data retention for long-term monitoring
- Use compression for large exports

## 🔍 Troubleshooting

### Common Issues
- **Permission denied**: Ensure proper network interface access
- **Timeout errors**: Increase timeout values for slow networks
- **No results**: Check network connectivity and target availability
- **Resource limits**: Reduce thread counts or scan ranges

### Debug Mode
Enable debug logging for detailed troubleshooting:

```bash
RUST_LOG=debug pingtest --tools diagnose
```

### Performance Issues
- Reduce thread counts for resource-constrained systems
- Use shorter timeouts for faster scans
- Limit scan ranges to reduce network load

## 📚 Examples

### Network Health Check
```bash
# Complete network assessment
pingtest --tools diagnose --format json --output health.json
pingtest --tools quality --duration 60 --format json --output quality.json
```

### Security Assessment
```bash
# Port scan with service detection
pingtest --tools scan-ports target.com --scan-type service --format json --output ports.json

# Network discovery
pingtest --tools scan-network 192.168.1.0/24 --scan-type comprehensive --format csv --output hosts.csv
```

### Performance Monitoring
```bash
# Bandwidth monitoring
pingtest --tools monitor --duration 300 --export-csv bandwidth.csv

# Statistics collection
pingtest --tools stats --duration 600 --export-all stats.json
```

### WiFi Analysis
```bash
# WiFi network analysis
pingtest --tools wifi --scan-type comprehensive --format json --output wifi.json
```

These advanced tools make PingTest a comprehensive network analysis platform suitable for network administrators, security professionals, and developers who need detailed network insights.