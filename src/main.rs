/// Rust SNMP Producer
///
///      (None) `V1` TODO: Complete with real logical state
///        | ip: String
///        | port: String
///        | topic: String
///        v
///   (UPD::connect())
///       |\_____________
///       |              | create::session()
///       |              |
///       |          [Produce]
///    panic()           |\___________
///                      |            |
///                   panic()         | {CreateMessage()}
///                                   | {Base-on Topic()}
///                                   v
///                       New-data-produced-to-Kafka
///
/// Logical State               __state
/// ------------                ---------
/// Idle                        _RSP_IDLE
/// Udp-Session-created         _RSP_UDP_SES_CREATED
/// Udp-Request-started         _RSP_UDP_STARTED
/// Gather-Snmp-info            _RSP_DATA_COLLECT
/// Parse-data                  _RSP_DATA_PARSE
/// Produce-2Kafka              _RSP_PRODUCER
use clap::Parser;
use std::io;
use tokio::net::UdpSocket;

mod producer;
use producer::Event;

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();
    println!("Start new rsp instance...");

    let _ = UdpSocket::bind(format!("{}:{}", args.ip, args.port)).await?;

    let mut producer: Event = Event::init(vec!["localhost:9092".to_string()]);
    producer.send_data(&args.topic, String::from(format!("This is message {}", "Test")));

    Ok(())
}

/// open-rsp - SNMP data gather and produce them to Kafka broker.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// IP: Destination Snmp machine Ip address
    #[arg(short, long)]
    ip: String,

    /// Port: Destination Snmp machine port number
    #[arg(short, long)]
    port: u32,

    /// Username: SNMP gateway username
    #[arg(short, long)]
    username: String,

    /// Password: SNMP gateway password
    #[arg(short, long)]
    passwd: String,

    /// Protocol: Connection protocol
    #[arg(short, long, default_value_t = String::from("MD5"))]
    connection_protocol: String,

    /// Topic: Topic name to produce value to Kafka
    #[arg(short, long)]
    topic: String,
}
