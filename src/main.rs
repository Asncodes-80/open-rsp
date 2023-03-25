/// Rust SNMP Producer
///
/// WIP: Up and running produce and write new data from here.
use kafka::producer::{Producer, Record};

fn main() {
    println!("Start new rsp instance...");

    let hosts: Vec<String> = vec!["localhost:9092".to_string()];
    let mut producer = Event::init(hosts);

    producer.send_data(
        "my-topic",
        String::from("This is only test from a super simple Rust app"),
    );
}

struct Event {
    producer: Producer,
}

impl Event {
    /// # Initiate new Kafka connection
    pub fn init(hosts: Vec<String>) -> Self {
        let producer = Producer::from_hosts(hosts).create().unwrap();

        Self { producer }
    }

    /// # Send raw data
    ///
    /// Gets string data, and converts it to bytes and sends it by a proper Kafka:Producer.
    pub fn send_data(&mut self, topic: &str, data: String) {
        let record = Record::from_value(topic, data.as_bytes());
        self.producer.send(&record).unwrap();
    }
}
