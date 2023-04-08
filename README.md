# Rust SNMP Producer - Client

A Rust microservice, connects to SNMP server, gets all SNMP data, parsing them
and produces data to kafka broker.

At `0.1.0` version, it get data from SNMP server through `snmpwalk` networking
shell tool.

## RSP App Description

```sh
open-rsp -i 127.0.0.1 -p 1234 -u username -k passwd --oid "1.3.6.1.6.3.15.1.13.44.0" -t "my-topic"
```
