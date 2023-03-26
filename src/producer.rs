use kafka::producer::{Producer, Record};

pub struct Event {
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
