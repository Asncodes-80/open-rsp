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
use std::io;
use std::process::{Command, Output};

use clap::Parser;

mod producer;

/// Rust Shell
///
/// Runs `snmpwalk` with shell use in std Command.
fn rs_shell(args: &Args) -> Result<Output, io::Error> {
    let output = Command::new("/usr/bin/snmpwalk")
        .args(&[
            "-v3",
            "-l",
            "authnoPriv",
            "-u",
            &args.username,
            "-A",
            &args.key,
            "-a",
            &args.encryption,
            &format!("{}:{}", &args.ip, &args.port),
            "1.3.6.1.6.3.15.1.13.44.0",
            "-O",
            "q",
        ])
        .output()?;

    Ok(output)
}

fn main() {
    let args = Args::parse();
    simple_logger::init().unwrap();

    let mut prev_value: f32 = 0.1;

    let binding = rs_shell(&args).unwrap();
    let output = String::from_utf8_lossy(&binding.stdout).into_owned();
    let snmp_output = output.split(" ").collect::<Vec<&str>>()[1].to_string();
    log::info!("{}", snmp_output);

    let mut producer: producer::Event = producer::Event::init(vec!["localhost:9092".to_string()]);
    producer.send_data(&args.topic, String::from(format!("value: {}", snmp_output)));

    // let str_output = output.split("=").collect::<Vec<&str>>()[1];
    // println!("{}", str_output.to_owned());

    // loop {
    //     // ...
    //     // Get snmp last data about an oid
    //     // let socket = UdpSocket::bind(format!("{}:{}", args.ip, args.port)).await?;
    //     // i.e. `next_value` parsed to a float value
    //     let next_value: f32 = 0.0;

    //     if prev_value != next_value {
    //         prev_value = next_value;

    // let mut producer: producer::Event = producer::Event::init(vec!["localhost:9092".to_string()]);
    // producer.send_data(&args.topic, String::from(format!("value: {}", prev_value)));

    //         log::info!("Value {} as new value, sent to to Kafka broker", prev_value);
    //     }
    // }
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
    key: String,

    /// Protocol: Connection protocol
    #[arg(short, long, default_value_t = String::from("MD5"))]
    encryption: String,

    /// Topic: Topic name to produce value to Kafka
    #[arg(short, long, default_value_t = String::from("my-topic"))]
    topic: String,
}
