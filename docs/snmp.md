# Simple Network Management Protocol

At This markdown file, I wrote some attentions about What SNMP is, and how you
can use it natively and What is difference between SNMP versions.

The most widely used version is SNMPv1, but it is man ways is insecure. SNMPv3
provides more advance security features.

## SNMP Versions Difference

SNMPV1 and SNMPV2 is insecure, but in SNMPV3 you can use user Authentication
before data gathering.

In SNMP V3, connection level is like following:

- Authentications (Pass user auth in `username`, `authKey`, `authProtocol`)
- Udp transport and transmission

## SNMP agent:

An SNMP agent is a program that can gather information about a piece of
**hardware**, organize it into predefined entries and respond to queries using
the SNMP protocol.

## Management Information Based or MIB

A database that SNMP agents can gathering information about the local system and
storing them in a format that can be queried in a database.

## Object Identifier or OID

The base path to this branch is: `1.3.6.1.2.1`

This can be strings like: `iso.org.dod.internet.mgmt.mid-2`

The section `1.3.6.1` or `iso.org.dod.internet` is the OID that defines
internets resources. The `2` or `mgmt` that follows in our base path is for a
management subcategory. The `1` or `mid-2` under that defines the MIB-2
specification.

## Trap

A trap message is generally sent by an agent to a manager. Traps are async
notifications in that they are unsolicited by the manage receiving them.

## snmpget cmd

```sh
snmpwalk -v3 -l authnoPriv -u <username> -a MD5 -A <password> <ip:port> <oid>
```

```txt
out:
SNMP-USER-BASED-SM-MIB::usmMIBObjects.13.44.0 = STRING: "26.38"
```