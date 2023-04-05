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
            &args.oid,
            "-O",
            "q",
        ])
        .output()?;

    Ok(output)
}

/// Returned SNMPWalk value.
#[derive(Debug, PartialEq)]
enum SnmpValue<'a> {
    Number(f32),
    Str(&'a str),
}

impl<'a> SnmpValue<'a> {
    pub fn new(value: &'a str) -> Self {
        let evaled = value.trim_end_matches('\n').trim_matches('"');
        let parsed = evaled.parse::<f32>();
        match parsed {
            Ok(v) => Self::Number(v),
            Err(_) => Self::Str(evaled),
        }
    }
}

fn main() {
    let args = Args::parse();
    simple_logger::init().unwrap();
    // let mut prev_value: f32 = 0.1;

    let binding = rs_shell(&args).unwrap();
    let output = String::from_utf8_lossy(&binding.stdout).into_owned();
    let last_value = {
        let mut output_bindings = output.split(" ");
        output_bindings.next();
        let Some(output) = output_bindings
            .next() else {
                panic!("Invalid value received from the `snmpwalk` command.");
            };
        SnmpValue::new(output)
    };

    log::info!("{:?}", last_value);

    // let mut producer: producer::Event = producer::Event::init(vec!["localhost:9092".to_string()]);
    // producer.send_data(&args.topic, String::from(format!("value: {}", last_value)));
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

    /// OID: Object id of one sensor
    #[arg(short, long)]
    oid: String,

    /// Topic: Topic name to produce value to Kafka
    #[arg(short, long, default_value_t = String::from("my-topic"))]
    topic: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_snmp_value_new() {
        assert_eq!(SnmpValue::new("salam"), SnmpValue::Str("salam"));
        assert_eq!(SnmpValue::new("24"), SnmpValue::Number(24.0));
        assert_eq!(SnmpValue::new("24.0384"), SnmpValue::Number(24.0384));
    }
}
