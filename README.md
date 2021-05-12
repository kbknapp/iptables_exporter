# `iptables_exporter`


An asynchronous Prometheus exporter for `iptables`

`iptables_exporter` runs `iptables-save --counter` and scrapes the output to
build Prometheus metrics. Because `iptables-save` requires `root` privileges,
this tool must be run as `root` (or via `sudo`) or with the following
capabilities in both the ambient and bounding set:

- CAP_DAC_READ_SEARCH
- CAP_NET_ADMIN
- CAP_NET_RAW

# Metrics Provided

- Total number of bytes per chain/table/policy
- Total number of bytes matched per rule/chain/table
- Total number of packets per chain/table/policy
- Total number of packets matched per rule/chain/table
- Total number of rules per chain/table
- Total number of chains per table
- Scrape duration in milliseconds
- Scrape success

# Installation

`iptables_exporter` is a single binary that must be placed somewhere in your
`$PATH`. One can either download 64-bit Linux binaries from [the Release Page](https://github.com/kbknapp/iptables_exporter/releases)
or one can also compile from source.

## Compile from Source

Ensure you have a [Rust toolchain installed](https://rustup.rs). Some of the
dependencies also require `gcc` to be installed.

```
$ git clone https://github.com/kbknapp/iptables_exporter
$ cd iptables_exporter
$ cargo build --release
$ sudo cp target/release/iptables_exporter /usr/local/bin/
```

# Usage

## Command Line Interface

```
USAGE:
    iptables_exporter [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -q, --quiet      Supress output at a level or lower. -q: INFO, -qq: WARN, -qqq: ERROR (i.e.
                     everything)
    -v, --verbose    Show verbose output at a level or higher. -v:  DEBUG, -vv: TRACE
    -V, --version    Prints version information

OPTIONS:
        --collect-interval <SECS>    How often metrics are gathered [default: 5]
    -l, --listen-address <ADDR>      The listen address scraping metrics [default: 0.0.0.0]
    -p, --listen-port <PORT>         The listen port for scraping metrics [default: 9455]
```

To run with the default options, and the binary is installed somewhere in your
`$PATH`:

```
$ sudo iptables_exporter
```

# Prometheus Configuration

You can add the following scrape configs to Prometheus:

```yaml
scrape_configs:
  - job_name: 'iptables'
    static_configs:
    - targets:
      - 'localhost:9455'
      - 'other_host:9455'

    relabel_configs:
    - source_labels: [ '__address__' ]
      regex: '(.*):\d+'
      target_label: instance
```

# Example Metrics

```
# HELP iptables_chain_bytes_total Total bytes flowing through a given chain
# TYPE iptables_chain_bytes_total counter
iptables_chain_bytes_total{chain="DOCKER",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="DOCKER",policy="ACCEPT",table="nat"} 0
iptables_chain_bytes_total{chain="DOCKER-ISOLATION-STAGE-1",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="DOCKER-ISOLATION-STAGE-2",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="FORWARD",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="FORWARD",policy="ACCEPT",table="mangle"} 0
iptables_chain_bytes_total{chain="FORWARD",policy="ACCEPT",table="security"} 0
iptables_chain_bytes_total{chain="INPUT",policy="ACCEPT",table="filter"} 1243840979
iptables_chain_bytes_total{chain="INPUT",policy="ACCEPT",table="mangle"} 1291467136
iptables_chain_bytes_total{chain="INPUT",policy="ACCEPT",table="nat"} 16724
iptables_chain_bytes_total{chain="INPUT",policy="ACCEPT",table="security"} 1291121184
iptables_chain_bytes_total{chain="LIBVIRT_FWI",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="LIBVIRT_FWO",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="LIBVIRT_FWX",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="LIBVIRT_INP",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="LIBVIRT_OUT",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="LIBVIRT_PRT",policy="ACCEPT",table="mangle"} 0
iptables_chain_bytes_total{chain="LIBVIRT_PRT",policy="ACCEPT",table="nat"} 0
iptables_chain_bytes_total{chain="OUTPUT",policy="ACCEPT",table="filter"} 639197815
iptables_chain_bytes_total{chain="OUTPUT",policy="ACCEPT",table="mangle"} 652390274
iptables_chain_bytes_total{chain="OUTPUT",policy="ACCEPT",table="nat"} 49151
iptables_chain_bytes_total{chain="OUTPUT",policy="ACCEPT",table="raw"} 652399242
iptables_chain_bytes_total{chain="OUTPUT",policy="ACCEPT",table="security"} 652399242
iptables_chain_bytes_total{chain="POSTROUTING",policy="ACCEPT",table="mangle"} 652416550
iptables_chain_bytes_total{chain="POSTROUTING",policy="ACCEPT",table="nat"} 220031
iptables_chain_bytes_total{chain="PREROUTING",policy="ACCEPT",table="mangle"} 1291467136
iptables_chain_bytes_total{chain="PREROUTING",policy="ACCEPT",table="nat"} 343629
iptables_chain_bytes_total{chain="PREROUTING",policy="ACCEPT",table="raw"} 1291476956
iptables_chain_bytes_total{chain="sshuttle-12300",policy="ACCEPT",table="nat"} 0
# HELP iptables_chain_packets_total Total packets flowing through a given chain
# TYPE iptables_chain_packets_total counter
iptables_chain_packets_total{chain="DOCKER",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="DOCKER",policy="ACCEPT",table="nat"} 0
iptables_chain_packets_total{chain="DOCKER-ISOLATION-STAGE-1",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="DOCKER-ISOLATION-STAGE-2",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="FORWARD",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="FORWARD",policy="ACCEPT",table="mangle"} 0
iptables_chain_packets_total{chain="FORWARD",policy="ACCEPT",table="security"} 0
iptables_chain_packets_total{chain="INPUT",policy="ACCEPT",table="filter"} 1243840979
iptables_chain_packets_total{chain="INPUT",policy="ACCEPT",table="mangle"} 1291467136
iptables_chain_packets_total{chain="INPUT",policy="ACCEPT",table="nat"} 16724
iptables_chain_packets_total{chain="INPUT",policy="ACCEPT",table="security"} 1291121184
iptables_chain_packets_total{chain="LIBVIRT_FWI",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="LIBVIRT_FWO",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="LIBVIRT_FWX",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="LIBVIRT_INP",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="LIBVIRT_OUT",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="LIBVIRT_PRT",policy="ACCEPT",table="mangle"} 0
iptables_chain_packets_total{chain="LIBVIRT_PRT",policy="ACCEPT",table="nat"} 0
iptables_chain_packets_total{chain="OUTPUT",policy="ACCEPT",table="filter"} 639197815
iptables_chain_packets_total{chain="OUTPUT",policy="ACCEPT",table="mangle"} 652390274
iptables_chain_packets_total{chain="OUTPUT",policy="ACCEPT",table="nat"} 49151
iptables_chain_packets_total{chain="OUTPUT",policy="ACCEPT",table="raw"} 652399242
iptables_chain_packets_total{chain="OUTPUT",policy="ACCEPT",table="security"} 652399242
iptables_chain_packets_total{chain="POSTROUTING",policy="ACCEPT",table="mangle"} 652416550
iptables_chain_packets_total{chain="POSTROUTING",policy="ACCEPT",table="nat"} 220031
iptables_chain_packets_total{chain="PREROUTING",policy="ACCEPT",table="mangle"} 1291467136
iptables_chain_packets_total{chain="PREROUTING",policy="ACCEPT",table="nat"} 343629
iptables_chain_packets_total{chain="PREROUTING",policy="ACCEPT",table="raw"} 1291476956
iptables_chain_packets_total{chain="sshuttle-12300",policy="ACCEPT",table="nat"} 0
# HELP iptables_chains_total Total number of chains in a table
# TYPE iptables_chains_total gauge
iptables_chains_total{table="filter"} 11
iptables_chains_total{table="mangle"} 6
iptables_chains_total{table="nat"} 7
iptables_chains_total{table="raw"} 2
iptables_chains_total{table="security"} 3
# HELP iptables_rule_bytes_total Total bytes matching a given rule
# TYPE iptables_rule_bytes_total counter
iptables_rule_bytes_total{chain="DOCKER",rule="-i docker0 -j RETURN",table="nat"} 0
iptables_rule_bytes_total{chain="DOCKER-ISOLATION-STAGE-1",rule="-j RETURN",table="filter"} 0
iptables_rule_bytes_total{chain="DOCKER-ISOLATION-STAGE-2",rule="-j RETURN",table="filter"} 0
iptables_rule_bytes_total{chain="FORWARD",rule="-i docker0 ! -o docker0 -j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="FORWARD",rule="-i docker0 -o docker0 -j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="FORWARD",rule="-j LIBVIRT_FWI",table="filter"} 0
iptables_rule_bytes_total{chain="FORWARD",rule="-j LIBVIRT_FWO",table="filter"} 0
iptables_rule_bytes_total{chain="FORWARD",rule="-j LIBVIRT_FWX",table="filter"} 0
iptables_rule_bytes_total{chain="INPUT",rule="-j LIBVIRT_INP",table="filter"} 1291467196
iptables_rule_bytes_total{chain="LIBVIRT_FWI",rule="-d 192.168.84.0/24 -o virbr0 -m conntrack --ctstate RELATED,ESTABLISHED -j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="LIBVIRT_FWI",rule="-o virbr0 -j REJECT --reject-with icmp-port-unreachable",table="filter"} 0
iptables_rule_bytes_total{chain="LIBVIRT_FWO",rule="-i virbr0 -j REJECT --reject-with icmp-port-unreachable",table="filter"} 0
iptables_rule_bytes_total{chain="LIBVIRT_FWO",rule="-s 192.168.84.0/24 -i virbr0 -j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="LIBVIRT_FWX",rule="-i virbr0 -o virbr0 -j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="LIBVIRT_INP",rule="-i virbr0 -p tcp -m tcp --dport 53 -j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="LIBVIRT_INP",rule="-i virbr0 -p tcp -m tcp --dport 67 -j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="LIBVIRT_INP",rule="-i virbr0 -p udp -m udp --dport 53 -j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="LIBVIRT_INP",rule="-i virbr0 -p udp -m udp --dport 67 -j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="LIBVIRT_OUT",rule="-o virbr0 -p tcp -m tcp --dport 53 -j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="LIBVIRT_OUT",rule="-o virbr0 -p tcp -m tcp --dport 68 -j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="LIBVIRT_OUT",rule="-o virbr0 -p udp -m udp --dport 53 -j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="LIBVIRT_OUT",rule="-o virbr0 -p udp -m udp --dport 68 -j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="LIBVIRT_PRT",rule="-o virbr0 -p udp -m udp --dport 68 -j CHECKSUM --checksum-fill",table="mangle"} 0
iptables_rule_bytes_total{chain="LIBVIRT_PRT",rule="-s 192.168.84.0/24 ! -d 192.168.122.0/24 -j MASQUERADE",table="nat"} 0
iptables_rule_bytes_total{chain="LIBVIRT_PRT",rule="-s 192.168.84.0/24 ! -d 192.168.122.0/24 -p tcp -j MASQUERADE --to-ports 1024-65535",table="nat"} 0
iptables_rule_bytes_total{chain="LIBVIRT_PRT",rule="-s 192.168.84.0/24 ! -d 192.168.122.0/24 -p udp -j MASQUERADE --to-ports 1024-65535",table="nat"} 0
iptables_rule_bytes_total{chain="LIBVIRT_PRT",rule="-s 192.168.84.0/24 -d 224.0.0.0/24 -j RETURN",table="nat"} 167
iptables_rule_bytes_total{chain="LIBVIRT_PRT",rule="-s 192.168.84.0/24 -d 255.255.255.255/32 -j RETURN",table="nat"} 0
iptables_rule_bytes_total{chain="OUTPUT",rule="-j LIBVIRT_OUT",table="filter"} 652390620
iptables_rule_bytes_total{chain="OUTPUT",rule="-j sshuttle-12300",table="nat"} 250545
iptables_rule_bytes_total{chain="POSTROUTING",rule="-j LIBVIRT_PRT",table="mangle"} 652416896
iptables_rule_bytes_total{chain="POSTROUTING",rule="-j LIBVIRT_PRT",table="nat"} 274670
iptables_rule_bytes_total{chain="POSTROUTING",rule="-s 172.17.0.0/16 ! -o docker0 -j MASQUERADE",table="nat"} 0
iptables_rule_bytes_total{chain="PREROUTING",rule="-j sshuttle-12300",table="nat"} 350113
iptables_rule_bytes_total{chain="sshuttle-12300",rule="-d 127.0.0.1/32 -p tcp -j RETURN",table="nat"} 0
iptables_rule_bytes_total{chain="sshuttle-12300",rule="-m addrtype --dst-type LOCAL -j RETURN",table="nat"} 13654
iptables_rule_bytes_total{chain="sshuttle-12300",rule="-m ttl --ttl-eq 63 -j RETURN",table="nat"} 0
iptables_rule_bytes_total{chain="sshuttle-12300",rule="-p tcp -j REDIRECT --to-ports 12300",table="nat"} 183900
# HELP iptables_rule_packets_total Total packets matching a given rule
# TYPE iptables_rule_packets_total counter
iptables_rule_packets_total{chain="DOCKER",rule="-i docker0 -j RETURN",table="nat"} 0
iptables_rule_packets_total{chain="DOCKER-ISOLATION-STAGE-1",rule="-j RETURN",table="filter"} 0
iptables_rule_packets_total{chain="DOCKER-ISOLATION-STAGE-2",rule="-j RETURN",table="filter"} 0
iptables_rule_packets_total{chain="FORWARD",rule="-i docker0 ! -o docker0 -j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="FORWARD",rule="-i docker0 -o docker0 -j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="FORWARD",rule="-j LIBVIRT_FWI",table="filter"} 0
iptables_rule_packets_total{chain="FORWARD",rule="-j LIBVIRT_FWO",table="filter"} 0
iptables_rule_packets_total{chain="FORWARD",rule="-j LIBVIRT_FWX",table="filter"} 0
iptables_rule_packets_total{chain="INPUT",rule="-j LIBVIRT_INP",table="filter"} 1128660
iptables_rule_packets_total{chain="LIBVIRT_FWI",rule="-d 192.168.84.0/24 -o virbr0 -m conntrack --ctstate RELATED,ESTABLISHED -j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="LIBVIRT_FWI",rule="-o virbr0 -j REJECT --reject-with icmp-port-unreachable",table="filter"} 0
iptables_rule_packets_total{chain="LIBVIRT_FWO",rule="-i virbr0 -j REJECT --reject-with icmp-port-unreachable",table="filter"} 0
iptables_rule_packets_total{chain="LIBVIRT_FWO",rule="-s 192.168.84.0/24 -i virbr0 -j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="LIBVIRT_FWX",rule="-i virbr0 -o virbr0 -j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="LIBVIRT_INP",rule="-i virbr0 -p tcp -m tcp --dport 53 -j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="LIBVIRT_INP",rule="-i virbr0 -p tcp -m tcp --dport 67 -j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="LIBVIRT_INP",rule="-i virbr0 -p udp -m udp --dport 53 -j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="LIBVIRT_INP",rule="-i virbr0 -p udp -m udp --dport 67 -j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="LIBVIRT_OUT",rule="-o virbr0 -p tcp -m tcp --dport 53 -j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="LIBVIRT_OUT",rule="-o virbr0 -p tcp -m tcp --dport 68 -j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="LIBVIRT_OUT",rule="-o virbr0 -p udp -m udp --dport 53 -j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="LIBVIRT_OUT",rule="-o virbr0 -p udp -m udp --dport 68 -j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="LIBVIRT_PRT",rule="-o virbr0 -p udp -m udp --dport 68 -j CHECKSUM --checksum-fill",table="mangle"} 0
iptables_rule_packets_total{chain="LIBVIRT_PRT",rule="-s 192.168.84.0/24 ! -d 192.168.122.0/24 -j MASQUERADE",table="nat"} 0
iptables_rule_packets_total{chain="LIBVIRT_PRT",rule="-s 192.168.84.0/24 ! -d 192.168.122.0/24 -p tcp -j MASQUERADE --to-ports 1024-65535",table="nat"} 0
iptables_rule_packets_total{chain="LIBVIRT_PRT",rule="-s 192.168.84.0/24 ! -d 192.168.122.0/24 -p udp -j MASQUERADE --to-ports 1024-65535",table="nat"} 0
iptables_rule_packets_total{chain="LIBVIRT_PRT",rule="-s 192.168.84.0/24 -d 224.0.0.0/24 -j RETURN",table="nat"} 2
iptables_rule_packets_total{chain="LIBVIRT_PRT",rule="-s 192.168.84.0/24 -d 255.255.255.255/32 -j RETURN",table="nat"} 0
iptables_rule_packets_total{chain="OUTPUT",rule="-j LIBVIRT_OUT",table="filter"} 988683
iptables_rule_packets_total{chain="OUTPUT",rule="-j sshuttle-12300",table="nat"} 3804
iptables_rule_packets_total{chain="POSTROUTING",rule="-j LIBVIRT_PRT",table="mangle"} 988827
iptables_rule_packets_total{chain="POSTROUTING",rule="-j LIBVIRT_PRT",table="nat"} 4020
iptables_rule_packets_total{chain="POSTROUTING",rule="-s 172.17.0.0/16 ! -o docker0 -j MASQUERADE",table="nat"} 0
iptables_rule_packets_total{chain="PREROUTING",rule="-j sshuttle-12300",table="nat"} 3190
iptables_rule_packets_total{chain="sshuttle-12300",rule="-d 127.0.0.1/32 -p tcp -j RETURN",table="nat"} 0
iptables_rule_packets_total{chain="sshuttle-12300",rule="-m addrtype --dst-type LOCAL -j RETURN",table="nat"} 204
iptables_rule_packets_total{chain="sshuttle-12300",rule="-m ttl --ttl-eq 63 -j RETURN",table="nat"} 0
iptables_rule_packets_total{chain="sshuttle-12300",rule="-p tcp -j REDIRECT --to-ports 12300",table="nat"} 3065
# HELP iptables_rules_total Total number of rules in a chain in a table
# TYPE iptables_rules_total gauge
iptables_rules_total{chain="DOCKER",table="filter"} 0
iptables_rules_total{chain="DOCKER",table="nat"} 1
iptables_rules_total{chain="DOCKER-ISOLATION-STAGE-1",table="filter"} 1
iptables_rules_total{chain="DOCKER-ISOLATION-STAGE-2",table="filter"} 1
iptables_rules_total{chain="FORWARD",table="filter"} 5
iptables_rules_total{chain="FORWARD",table="mangle"} 0
iptables_rules_total{chain="FORWARD",table="security"} 0
iptables_rules_total{chain="INPUT",table="filter"} 1
iptables_rules_total{chain="INPUT",table="mangle"} 0
iptables_rules_total{chain="INPUT",table="nat"} 0
iptables_rules_total{chain="INPUT",table="security"} 0
iptables_rules_total{chain="LIBVIRT_FWI",table="filter"} 2
iptables_rules_total{chain="LIBVIRT_FWO",table="filter"} 2
iptables_rules_total{chain="LIBVIRT_FWX",table="filter"} 1
iptables_rules_total{chain="LIBVIRT_INP",table="filter"} 4
iptables_rules_total{chain="LIBVIRT_OUT",table="filter"} 4
iptables_rules_total{chain="LIBVIRT_PRT",table="mangle"} 1
iptables_rules_total{chain="LIBVIRT_PRT",table="nat"} 5
iptables_rules_total{chain="OUTPUT",table="filter"} 1
iptables_rules_total{chain="OUTPUT",table="mangle"} 0
iptables_rules_total{chain="OUTPUT",table="nat"} 1
iptables_rules_total{chain="OUTPUT",table="raw"} 0
iptables_rules_total{chain="OUTPUT",table="security"} 0
iptables_rules_total{chain="POSTROUTING",table="mangle"} 1
iptables_rules_total{chain="POSTROUTING",table="nat"} 2
iptables_rules_total{chain="PREROUTING",table="mangle"} 0
iptables_rules_total{chain="PREROUTING",table="nat"} 1
iptables_rules_total{chain="PREROUTING",table="raw"} 0
iptables_rules_total{chain="sshuttle-12300",table="nat"} 4
# HELP iptables_scrape_duration_milliseconds Duration in milliseconds of the scrape
# TYPE iptables_scrape_duration_milliseconds gauge
iptables_scrape_duration_milliseconds 2
# HELP iptables_scrape_success If the scrape was a success
# TYPE iptables_scrape_success gauge
iptables_scrape_success 1
```

# License

This project is dual licensed under the terms of either the MIT or Apache 2.0
at your option.
