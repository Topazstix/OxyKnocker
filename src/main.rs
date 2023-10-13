use clap::{arg, Command};
use std::net::{SocketAddr, TcpStream, UdpSocket};
use std::thread::sleep;
use std::time::Duration;

// TODO: timeout and delay currently have an issue unwrapping. 
struct Knocker {
    // timeout: u64,
    // delay: u64,
    default_udp: bool,
    ports: Vec<String>,
    verbose: bool,
    ip_addr: String,
}

impl Knocker {
    fn new(args: Vec<String>) -> Self {
        let matches = Command::new(clap::crate_name!())
            
            .args(&[
                // arg!(-t --timeout [timeout_ms] "Timeout in milliseconds")
                //     .default_value("200"),
                // arg!(-d --delay [delay_ms] "Delay in milliseconds")
                //     .default_value("100"),
                arg!(-u --udp "Use UDP instead of TCP")
                    .default_value("false"),
                arg!(-v --verbose "Verbose output")
                    .default_value("false"),
                arg!(<ip> "IP to connect to")
                    .required(true),
                arg!(<ports> "Ports to knock")
                    .required(true),
            ])
            .get_matches_from(args);

        // let timeout = matches.get_one("timeout").unwrap();
        // let delay = matches.get_one("delay").unwrap();
        let default_udp = matches.get_one::<bool>("udp").unwrap();
        let verbose = matches.get_one::<bool>("verbose").unwrap();
        let ip_addr = matches.get_one::<String>("ip").unwrap();
        let ports = matches.get_many::<String>("ports").unwrap().map(|s| s.to_string()).collect::<Vec<_>>();


        Knocker {
            // timeout: *timeout,
            // delay: *delay,
            default_udp: *default_udp,
            ports: (*ports).to_vec(),
            verbose: *verbose,
            ip_addr: (*ip_addr).to_string(),
        }
    }

    fn knock(&self) {

        let last_index = self.ports.len() - 1;
        for (i, port) in self.ports.iter().enumerate() {
            let use_udp = self.default_udp;
            let port_num = port.clone().parse::<u16>().unwrap();

            if self.verbose {
                println!(
                    "Knocking on port {} {}:{}", 
                    if use_udp { "UDP" } else { "TCP" },
                    self.ip_addr,
                    port_num
                );
            }

            let socket_addr = SocketAddr::new(self.ip_addr.parse().unwrap(), port_num);

            if use_udp {
                let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
                socket.send_to(&[0u8; 0], &socket_addr).unwrap();
            } else {
                // let _stream = TcpStream::connect_timeout(&socket_addr, Duration::from_millis(self.timeout)).unwrap();
                let _stream = TcpStream::connect_timeout(&socket_addr, Duration::from_millis(200)).unwrap();
            }

            // TODO: Once self.delay is fixed, add self.delay > 0 &&
            if i != last_index {
                sleep(Duration::from_millis(200));
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let knocker = Knocker::new(args);
    knocker.knock();
}
