/// Rust SNMP Producer
///
/// WIP: Up and running produce and write new data from here.
use std::{thread, time};

use clap::Parser;

mod producer;
use producer::Event;

fn main() {
    let args = Args::parse();
    println!("Start new rsp instance...");

    let hosts: Vec<String> = vec!["localhost:9092".to_string()];
    let mut producer = Event::init(hosts);

    for i in 1..=args.times {
        producer.send_data("my-topic", String::from(format!("This is message {}", i)));
        thread::sleep(time::Duration::from_secs(args.duration));
    }
}

/// open-rsp - Produce Snmp data to Kafka broker.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// duration: Speed of produce new value to Kafka broker
    #[arg(short, long, default_value_t = 1)]
    duration: u64,

    /// times: count of produce new value
    #[arg(short, long, default_value_t = 5)]
    times: i32,

    /// ip: Destination Snmp machine Ip address
    #[arg(short, long, default_value_t=String::from("127.0.0.1"))]
    ip: String,

    /// port: Destination Snmp machine port number
    #[arg(short, long, default_value_t = 1234)]
    port: u32,
}
