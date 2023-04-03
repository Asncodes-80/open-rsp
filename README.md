# Rust SNMP Producer - Client

A Rust microservice, connects to SNMP server, gets all SNMP data, parsing them
and produces data to kafka broker.

## RSP App Description

```sh
./open-rsp --ip 5.201.128.78 --port 5051 --username publicMD5 --password publicMD5 --oid 1.3.6.1.6.3.15.1.13.44.0
```

## References 

Check this sites:

- [Rust Snmp Simulator](https://sonalake.com/latest/an-open-source-rust-snmp-simulator/)
- [Rust Snmp data collector](https://github.com/kporika/snmp-collection/blob/main/src/main.rs)
- [Sexy Rust TUI](https://docs.rs/ratatui/latest/ratatui/)