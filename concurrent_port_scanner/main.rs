use std::env;
use std::process;
use std::thread;
use std::net::{TcpStream, Ipv4Addr, SocketAddr, SocketAddrV4};
use std::time::Duration;

fn check_args(args: Vec<String>) {
    if args.len() == 1 || (args.len() == 2 && &args[1] == "h") {
        println!("How to use:\n");
        println!("cargo run [Ip address] <X>-<Y>");
        println!("                        X being the starting port and Y the ending port");
        process::exit(1);
    } else if args.len() != 3 {
        println!("Need only IP address and ports range.");
        process::exit(1);
    }
}

fn check_ip(input: String) -> Ipv4Addr {
    let ip: Ipv4Addr = match input.parse() {
        Ok(addr) => addr,
        Err(e) => {
            println!("IP format incorrect causing {:?}.\nEnding program.", e);
            process::exit(1);
        },
    };
    ip
}

fn check_port(input: String) -> Vec<u16> {
    let range: Vec<u16> = match input
        .split("-")
        .map(|p| p.parse::<u16>())
        .collect::<Result<Vec<u16>, _>>()
    {
        Ok(vec) => vec,
        Err(e) => {
            println!("Port range format incorrect causing {:?}.\nEnding program.", e);
            process::exit(1);
        }
    };

    if range.len() != 2 {
        println!("Port range format incorrect, please provide only the first and last ports you want.\nEnding program.");
        process::exit(1);
    }
    range
}

fn scan(ip: Ipv4Addr, port: u16) -> u8 {
    let addr = SocketAddr::V4(SocketAddrV4::new(ip, port));
    let timeout = Duration::from_secs(1); // reduced timeout for faster scanning

    match TcpStream::connect_timeout(&addr, timeout) {
        Ok(_) => 1, // port is open
        Err(_) => 0, // port is closed
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    check_args(args.clone());

    let ip = check_ip(args[1].clone());
    let port_range = check_port(args[2].clone());

    let start = port_range[0];
    let finish = port_range[1];

    let mut handles = Vec::new();

    for port in start..=finish {
        let ip_copy = ip;
        let handle = thread::spawn(move || {
            let result = scan(ip_copy, port);
            (port, result)
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        if let Ok(res) = handle.join() {
            results.push(res);
        }
    }

    let mut open_ports = Vec::new();
    let mut closed_count = 0;

    for (port, status) in results {
        if status == 1 {
            open_ports.push(port);
        } else {
            closed_count += 1;
        }
    }

    println!{"Number of closed ports: {}", closed_count};
    println!("Open ports: {:?}", open_ports);
}

