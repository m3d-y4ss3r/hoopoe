use ipnetwork::{IpNetwork, Ipv4Network, Ipv6Network};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::time:Duration;
use std::str::FromStr;
use std::env;

fn parse_args() -> Result<(Vec<IpAddr>, Vec<u16>, Duration), &'static str> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        return Err("Usage: ./hoopoe <ip> -p <port> [-t <timeout>]");
    }
    // IPs single, multiple, CIDR
    let ips: Vec<IpAddr> = args[1]
    .split(',')
    .flat_map(|ip_arg| {
        if let Ok(ip) = IpAddr::from_str(ip_arg) {
            return Ok(vec![ip]);
        }

        if let Ok(cidr) = IpNetwork::from_str(ip_arg) {
            return Ok(cidr.iter().collect());
        }

        Err("Invalid IP address or CIDR range")
    })
    .collect::<Result<Vec<IpAddr>, &str>>()?;

    // Ports sinlge,multiple,ranges
    if args[2] != "-p" {
        return Err("Invalid argument");
    }
    let ports: Vec<u16> = args[3]
    .split(',')
    .flat_map(|port| {
        if port.contains('-') {
            let range: Vec<&str> = port.split('-').collect();
            if range.len() != 2 {
                return Err("Invalid port range");
            }
            let start = range[0].parse::<u16>().map_err(|_| "Invalid port number")?;
            let end = range[1].parse::<u16>().map_err(|_| "Invalid port number")?;
            Ok(start..=end)
        } else {
            port.parse::<u16>().map_err(|_| "Invalid port number").map(|p| vec![p])
        }
    })
    .flatten()
    .collect::<Vec<u16>>()?;
    // Timeout ms/s
    let timeout = if args.len() > 4 && args[4] == "-t" {
        if args.len() < 6 { //arg[0]..arg[5]
            return Err("Timeout value is missing");
        }
        if timeout_str.ends_with("ms") {
            // Timeout is in milliseconds
            let numeric_part = &timeout_str[..timeout_str.len() - 2];
            let timeout_value = numeric_part.parse::<u16>().map_err(|_| "Invalid timeout value")?;
            let timeout_duration = Duration::from_millis(timeout_value);
        } else if timeout_str.ends_with("s") {
            // Timeout is in seconds
            let numeric_part = &timeout_str[..timeout_str.len() - 1];
            let timeout_value = numeric_part.parse::<u16>().map_err(|_| "Invalid timeout value")?;
            let timeout_duration = Duration::from_secs(timeout_value);
        } else {
            return Err("Invalid timeout unit");
        }
    } else {
        let timeout_duration = Duration::from_secs(2); //Default = 2secs
    };

    Ok((ips, ports, timeout))
}

fn tcp_connect(ips: &[IpAddr], ports: &[u16], timeout: Duration) {
    for &ip in ips {
        for &port in ports {
            let target = format!("{}:{}", ip, port);
            match TcpStream::connect_timeout(&(target), timeout) {
                Ok(_) => println!("[+] {} is OPEN", port),
                Err(_) => println!("[-] {} is CLOSED", port)
            }
        }
    }
}

fn main() {
    tcp_connect(ips, ports ,duration)
    
}