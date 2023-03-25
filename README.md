# Rust SNMP Producer - Client

A Rust client app, connects to SNMP server, gets all SNMP data, parsing them,
produces data to kafka borker.

## RSP App Description

## Kafka running

This is only demo to run a kafka broker server after downloaded it bineries.

```sh
# Running zookeeper server
bin/zookeeper-server-start.sh config/zookeeper.properties

# Running Kafka broker
bin/kafka-server-start.sh config/server.properties
```

## Create new Topic

After create a topic, it will open new consumer about your topic: 

```sh
bin/kafka-topics.sh --create --topic my-topic --botstrap-server localhost:9092 # Created topic my-topic
```

## Read::Consumer

Listening all data over the localhost:9092 base on kafka consumer console:

```sh
bin/kafka-console-consumer.sh --topic my-topic --from-beginning --bootstrap-server localhost:9092
```

## Write::Producer

Prepares a console environment to write new message to send Kafka consumer:

```sh
bin/kafka-console-producer.sh --topic my-topic --botstrap-server localhost:9092
```
