# `iptables_exporter`


An asynchronous Prometheus exporter for `iptables`

`iptables_exporter` runs one of several backend "scrape targets" such as
`iptables-save --counter` and scrapes the output to build Prometheus metrics.
Because these scrape targets require `root` privileges, this tool must be run as
`root` (or via `sudo`) or with the following capabilities in both the ambient
and bounding set:

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

# Scrape Targets Supported

- `iptables-save`
- `ip6tables-save`
- `iptables-legacy-save`
- `ip6tables-legacy-save`

Multiple scrape targets can be enabled at once by using the
`-t|--scrape-targets` flag multiple times. Such as:

```
$ iptables_exporter -t iptables -t iptables-legacy -t ip6tables
```

By default only `iptables` is enabled.

The metrics provided will be prefixed with the various scrape targets, such as
`iptables_*`, `iptables_legacy_*`, etc. 

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
Usage: iptables_exporter [OPTIONS]

Options:
      --collect-interval <SECS>
          How often metrics are gathered

          [default: 5]

  -p, --listen-port <PORT>
          The listen port for scraping metrics

          [default: 9455]

  -l, --listen-address <ADDR>
          The listen address scraping metrics

          [default: 0.0.0.0]

  -t, --scrape-targets <TARGET>
          Which backends to scrape for metrics, multiple targets can be enabled at
          once by using this flag multiple times

          [default: iptables]
          [aliases: scrape-target]

          Possible values:
          - iptables:         enable 'iptables-save' for metrics
          - ip6tables:        enable 'ip6tables-save' for metrics
          - iptables-legacy:  enable 'iptables-legacy-save' for metrics
          - ip6tables-legacy: enable 'ip6tables-legacy-save' for metrics

  -v, --verbose...
          Show verbose output at a level or higher. -v:  DEBUG, -vv: TRACE

  -q, --quiet...
          Supress output at a level or lower. -q: INFO, -qq: WARN, -qqq: ERROR (i.e.
          everything)

  -h, --help
          Print help information (use `-h` for a summary)

  -V, --version
          Print version information
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
iptables_chain_bytes_total{chain="DOCKER-USER",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="FORWARD",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="INPUT",policy="ACCEPT",table="filter"} 98893683
iptables_chain_bytes_total{chain="INPUT",policy="ACCEPT",table="nat"} 0
iptables_chain_bytes_total{chain="OUTPUT",policy="ACCEPT",table="filter"} 196455
iptables_chain_bytes_total{chain="OUTPUT",policy="ACCEPT",table="nat"} 0
iptables_chain_bytes_total{chain="POSTROUTING",policy="ACCEPT",table="nat"} 0
iptables_chain_bytes_total{chain="PREROUTING",policy="ACCEPT",table="nat"} 0
iptables_chain_bytes_total{chain="ts-forward",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ts-input",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ts-postrouting",policy="ACCEPT",table="nat"} 0
iptables_chain_bytes_total{chain="ufw-after-forward",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ufw-after-input",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ufw-after-logging-forward",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ufw-after-logging-input",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ufw-after-logging-output",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ufw-after-output",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ufw-before-forward",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ufw-before-input",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ufw-before-logging-forward",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ufw-before-logging-input",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ufw-before-logging-output",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ufw-before-output",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ufw-logging-allow",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ufw-logging-deny",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ufw-not-local",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ufw-reject-forward",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ufw-reject-input",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ufw-reject-output",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ufw-skip-to-policy-forward",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ufw-skip-to-policy-input",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ufw-skip-to-policy-output",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ufw-track-forward",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ufw-track-input",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ufw-track-output",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ufw-user-forward",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ufw-user-input",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ufw-user-limit",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ufw-user-limit-accept",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ufw-user-logging-forward",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ufw-user-logging-input",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ufw-user-logging-output",policy="ACCEPT",table="filter"} 0
iptables_chain_bytes_total{chain="ufw-user-output",policy="ACCEPT",table="filter"} 0
# HELP iptables_chain_packets_total Total packets flowing through a given chain
# TYPE iptables_chain_packets_total counter
iptables_chain_packets_total{chain="DOCKER",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="DOCKER",policy="ACCEPT",table="nat"} 0
iptables_chain_packets_total{chain="DOCKER-ISOLATION-STAGE-1",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="DOCKER-ISOLATION-STAGE-2",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="DOCKER-USER",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="FORWARD",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="INPUT",policy="ACCEPT",table="filter"} 1036441
iptables_chain_packets_total{chain="INPUT",policy="ACCEPT",table="nat"} 0
iptables_chain_packets_total{chain="OUTPUT",policy="ACCEPT",table="filter"} 2498
iptables_chain_packets_total{chain="OUTPUT",policy="ACCEPT",table="nat"} 0
iptables_chain_packets_total{chain="POSTROUTING",policy="ACCEPT",table="nat"} 0
iptables_chain_packets_total{chain="PREROUTING",policy="ACCEPT",table="nat"} 0
iptables_chain_packets_total{chain="ts-forward",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ts-input",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ts-postrouting",policy="ACCEPT",table="nat"} 0
iptables_chain_packets_total{chain="ufw-after-forward",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ufw-after-input",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ufw-after-logging-forward",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ufw-after-logging-input",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ufw-after-logging-output",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ufw-after-output",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ufw-before-forward",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ufw-before-input",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ufw-before-logging-forward",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ufw-before-logging-input",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ufw-before-logging-output",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ufw-before-output",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ufw-logging-allow",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ufw-logging-deny",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ufw-not-local",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ufw-reject-forward",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ufw-reject-input",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ufw-reject-output",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ufw-skip-to-policy-forward",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ufw-skip-to-policy-input",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ufw-skip-to-policy-output",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ufw-track-forward",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ufw-track-input",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ufw-track-output",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ufw-user-forward",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ufw-user-input",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ufw-user-limit",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ufw-user-limit-accept",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ufw-user-logging-forward",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ufw-user-logging-input",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ufw-user-logging-output",policy="ACCEPT",table="filter"} 0
iptables_chain_packets_total{chain="ufw-user-output",policy="ACCEPT",table="filter"} 0
# HELP iptables_chains_total Total number of chains in a table
# TYPE iptables_chains_total gauge
iptables_chains_total{table="filter"} 41
iptables_chains_total{table="nat"} 6
# HELP iptables_comment_bytes_total Total bytes matching a given comment inside a rule
# TYPE iptables_comment_bytes_total counter
iptables_comment_bytes_total{chain="ufw-user-input",comment="\\'dapp_Nginx%20Full\\'",table="filter"} 0
iptables_comment_bytes_total{chain="ufw-user-input",comment="\\'dapp_OpenSSH\\'",table="filter"} 0
# HELP iptables_comment_packets_total Total packets matching a given comment inside a rule
# TYPE iptables_comment_packets_total counter
iptables_comment_packets_total{chain="ufw-user-input",comment="\\'dapp_Nginx%20Full\\'",table="filter"} 0
iptables_comment_packets_total{chain="ufw-user-input",comment="\\'dapp_OpenSSH\\'",table="filter"} 0
# HELP iptables_rule_bytes_total Total bytes matching a given rule
# TYPE iptables_rule_bytes_total counter
iptables_rule_bytes_total{chain="DOCKER",rule="! -i br-6379b058093a -p tcp -m tcp --dport 3000 -j DNAT --to-destination 172.18.0.3:3000",table="nat"} 103564
iptables_rule_bytes_total{chain="DOCKER",rule="-d 127.0.0.1/32 ! -i br-6379b058093a -p tcp -m tcp --dport 3022 -j DNAT --to-destination 172.18.0.3:22",table="nat"} 0
iptables_rule_bytes_total{chain="DOCKER",rule="-d 172.18.0.3/32 ! -i br-6379b058093a -o br-6379b058093a -p tcp -m tcp --dport 22 -j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="DOCKER",rule="-d 172.18.0.3/32 ! -i br-6379b058093a -o br-6379b058093a -p tcp -m tcp --dport 3000 -j ACCEPT",table="filter"} 103564
iptables_rule_bytes_total{chain="DOCKER",rule="-i br-6379b058093a -j RETURN",table="nat"} 4056
iptables_rule_bytes_total{chain="DOCKER",rule="-i docker0 -j RETURN",table="nat"} 0
iptables_rule_bytes_total{chain="DOCKER-ISOLATION-STAGE-1",rule="-i br-6379b058093a ! -o br-6379b058093a -j DOCKER-ISOLATION-STAGE-2",table="filter"} 84363279
iptables_rule_bytes_total{chain="DOCKER-ISOLATION-STAGE-1",rule="-i docker0 ! -o docker0 -j DOCKER-ISOLATION-STAGE-2",table="filter"} 0
iptables_rule_bytes_total{chain="DOCKER-ISOLATION-STAGE-1",rule="-j RETURN",table="filter"} 70219112923
iptables_rule_bytes_total{chain="DOCKER-ISOLATION-STAGE-2",rule="-j RETURN",table="filter"} 84363279
iptables_rule_bytes_total{chain="DOCKER-ISOLATION-STAGE-2",rule="-o br-6379b058093a -j DROP",table="filter"} 0
iptables_rule_bytes_total{chain="DOCKER-ISOLATION-STAGE-2",rule="-o docker0 -j DROP",table="filter"} 0
iptables_rule_bytes_total{chain="DOCKER-USER",rule="-j RETURN",table="filter"} 106189380614
iptables_rule_bytes_total{chain="FORWARD",rule="-i br-6379b058093a ! -o br-6379b058093a -j ACCEPT",table="filter"} 96793557
iptables_rule_bytes_total{chain="FORWARD",rule="-i br-6379b058093a -o br-6379b058093a -j ACCEPT",table="filter"} 38645160
iptables_rule_bytes_total{chain="FORWARD",rule="-i docker0 ! -o docker0 -j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="FORWARD",rule="-i docker0 -o docker0 -j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="FORWARD",rule="-j DOCKER-ISOLATION-STAGE-1",table="filter"} 70219112923
iptables_rule_bytes_total{chain="FORWARD",rule="-j DOCKER-USER",table="filter"} 70219112923
iptables_rule_bytes_total{chain="FORWARD",rule="-j ts-forward",table="filter"} 70248886353
iptables_rule_bytes_total{chain="FORWARD",rule="-j ufw-after-forward",table="filter"} 0
iptables_rule_bytes_total{chain="FORWARD",rule="-j ufw-after-logging-forward",table="filter"} 0
iptables_rule_bytes_total{chain="FORWARD",rule="-j ufw-before-forward",table="filter"} 0
iptables_rule_bytes_total{chain="FORWARD",rule="-j ufw-before-logging-forward",table="filter"} 0
iptables_rule_bytes_total{chain="FORWARD",rule="-j ufw-reject-forward",table="filter"} 0
iptables_rule_bytes_total{chain="FORWARD",rule="-j ufw-track-forward",table="filter"} 0
iptables_rule_bytes_total{chain="FORWARD",rule="-o br-6379b058093a -j DOCKER",table="filter"} 38800448
iptables_rule_bytes_total{chain="FORWARD",rule="-o br-6379b058093a -m conntrack --ctstate RELATED,ESTABLISHED -j ACCEPT",table="filter"} 106053786609
iptables_rule_bytes_total{chain="FORWARD",rule="-o docker0 -j DOCKER",table="filter"} 0
iptables_rule_bytes_total{chain="FORWARD",rule="-o docker0 -m conntrack --ctstate RELATED,ESTABLISHED -j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="INPUT",rule="-j ts-input",table="filter"} 144722461078
iptables_rule_bytes_total{chain="INPUT",rule="-j ufw-after-input",table="filter"} 99431756
iptables_rule_bytes_total{chain="INPUT",rule="-j ufw-after-logging-input",table="filter"} 98893683
iptables_rule_bytes_total{chain="INPUT",rule="-j ufw-before-input",table="filter"} 223670038486
iptables_rule_bytes_total{chain="INPUT",rule="-j ufw-before-logging-input",table="filter"} 223670038486
iptables_rule_bytes_total{chain="INPUT",rule="-j ufw-reject-input",table="filter"} 98893683
iptables_rule_bytes_total{chain="INPUT",rule="-j ufw-track-input",table="filter"} 98893683
iptables_rule_bytes_total{chain="OUTPUT",rule="! -d 127.0.0.0/8 -m addrtype --dst-type LOCAL -j DOCKER",table="nat"} 48072
iptables_rule_bytes_total{chain="OUTPUT",rule="-j ufw-after-logging-output",table="filter"} 114847240
iptables_rule_bytes_total{chain="OUTPUT",rule="-j ufw-after-output",table="filter"} 114847240
iptables_rule_bytes_total{chain="OUTPUT",rule="-j ufw-before-logging-output",table="filter"} 133420557102
iptables_rule_bytes_total{chain="OUTPUT",rule="-j ufw-before-output",table="filter"} 133420557102
iptables_rule_bytes_total{chain="OUTPUT",rule="-j ufw-reject-output",table="filter"} 114847240
iptables_rule_bytes_total{chain="OUTPUT",rule="-j ufw-track-output",table="filter"} 114847240
iptables_rule_bytes_total{chain="POSTROUTING",rule="-j MASQUERADE",table="nat"} 138400621
iptables_rule_bytes_total{chain="POSTROUTING",rule="-j ts-postrouting",table="nat"} 142259622
iptables_rule_bytes_total{chain="POSTROUTING",rule="-s 172.17.0.0/16 ! -o docker0 -j MASQUERADE",table="nat"} 15808
iptables_rule_bytes_total{chain="POSTROUTING",rule="-s 172.18.0.0/16 ! -o br-6379b058093a -j MASQUERADE",table="nat"} 5766232
iptables_rule_bytes_total{chain="POSTROUTING",rule="-s 172.18.0.3/32 -d 172.18.0.3/32 -p tcp -m tcp --dport 22 -j MASQUERADE",table="nat"} 0
iptables_rule_bytes_total{chain="POSTROUTING",rule="-s 172.18.0.3/32 -d 172.18.0.3/32 -p tcp -m tcp --dport 3000 -j MASQUERADE",table="nat"} 0
iptables_rule_bytes_total{chain="PREROUTING",rule="-m addrtype --dst-type LOCAL -j DOCKER",table="nat"} 125651037
iptables_rule_bytes_total{chain="ts-forward",rule="-i tailscale0 -j MARK --set-xmark 0x40000/0xff0000",table="filter"} 27766129
iptables_rule_bytes_total{chain="ts-forward",rule="-m mark --mark 0x40000/0xff0000 -j ACCEPT",table="filter"} 27766129
iptables_rule_bytes_total{chain="ts-forward",rule="-o tailscale0 -j ACCEPT",table="filter"} 5600284
iptables_rule_bytes_total{chain="ts-postrouting",rule="-m mark --mark 0x40000/0xff0000 -j MASQUERADE",table="nat"} 0
iptables_rule_bytes_total{chain="ufw-after-input",rule="-m addrtype --dst-type BROADCAST -j ufw-skip-to-policy-input",table="filter"} 0
iptables_rule_bytes_total{chain="ufw-after-input",rule="-p tcp -m tcp --dport 139 -j ufw-skip-to-policy-input",table="filter"} 31348
iptables_rule_bytes_total{chain="ufw-after-input",rule="-p tcp -m tcp --dport 445 -j ufw-skip-to-policy-input",table="filter"} 461276
iptables_rule_bytes_total{chain="ufw-after-input",rule="-p udp -m udp --dport 137 -j ufw-skip-to-policy-input",table="filter"} 43685
iptables_rule_bytes_total{chain="ufw-after-input",rule="-p udp -m udp --dport 138 -j ufw-skip-to-policy-input",table="filter"} 392
iptables_rule_bytes_total{chain="ufw-after-input",rule="-p udp -m udp --dport 67 -j ufw-skip-to-policy-input",table="filter"} 980
iptables_rule_bytes_total{chain="ufw-after-input",rule="-p udp -m udp --dport 68 -j ufw-skip-to-policy-input",table="filter"} 392
iptables_rule_bytes_total{chain="ufw-after-logging-forward",rule="-m limit --limit 3/min --limit-burst 10 -j LOG --log-prefix \"[UFW BLOCK] \"",table="filter"} 0
iptables_rule_bytes_total{chain="ufw-after-logging-input",rule="-m limit --limit 3/min --limit-burst 10 -j LOG --log-prefix \"[UFW BLOCK] \"",table="filter"} 14484045
iptables_rule_bytes_total{chain="ufw-before-forward",rule="-j ufw-user-forward",table="filter"} 0
iptables_rule_bytes_total{chain="ufw-before-forward",rule="-m conntrack --ctstate RELATED,ESTABLISHED -j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="ufw-before-forward",rule="-p icmp -m icmp --icmp-type 11 -j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="ufw-before-forward",rule="-p icmp -m icmp --icmp-type 12 -j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="ufw-before-forward",rule="-p icmp -m icmp --icmp-type 3 -j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="ufw-before-forward",rule="-p icmp -m icmp --icmp-type 8 -j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="ufw-before-input",rule="-d 224.0.0.251/32 -p udp -m udp --dport 5353 -j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="ufw-before-input",rule="-d 239.255.255.250/32 -p udp -m udp --dport 1900 -j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="ufw-before-input",rule="-i lo -j ACCEPT",table="filter"} 109905406104
iptables_rule_bytes_total{chain="ufw-before-input",rule="-j ufw-not-local",table="filter"} 139808177
iptables_rule_bytes_total{chain="ufw-before-input",rule="-j ufw-user-input",table="filter"} 139808177
iptables_rule_bytes_total{chain="ufw-before-input",rule="-m conntrack --ctstate INVALID -j DROP",table="filter"} 12630859
iptables_rule_bytes_total{chain="ufw-before-input",rule="-m conntrack --ctstate INVALID -j ufw-logging-deny",table="filter"} 12630859
iptables_rule_bytes_total{chain="ufw-before-input",rule="-m conntrack --ctstate RELATED,ESTABLISHED -j ACCEPT",table="filter"} 113600086803
iptables_rule_bytes_total{chain="ufw-before-input",rule="-p icmp -m icmp --icmp-type 11 -j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="ufw-before-input",rule="-p icmp -m icmp --icmp-type 12 -j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="ufw-before-input",rule="-p icmp -m icmp --icmp-type 3 -j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="ufw-before-input",rule="-p icmp -m icmp --icmp-type 8 -j ACCEPT",table="filter"} 12106543
iptables_rule_bytes_total{chain="ufw-before-input",rule="-p udp -m udp --sport 67 --dport 68 -j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="ufw-before-output",rule="-j ufw-user-output",table="filter"} 114847240
iptables_rule_bytes_total{chain="ufw-before-output",rule="-m conntrack --ctstate RELATED,ESTABLISHED -j ACCEPT",table="filter"} 23400303758
iptables_rule_bytes_total{chain="ufw-before-output",rule="-o lo -j ACCEPT",table="filter"} 109905406104
iptables_rule_bytes_total{chain="ufw-logging-allow",rule="-m limit --limit 3/min --limit-burst 10 -j LOG --log-prefix \"[UFW ALLOW] \"",table="filter"} 0
iptables_rule_bytes_total{chain="ufw-logging-deny",rule="-m conntrack --ctstate INVALID -m limit --limit 3/min --limit-burst 10 -j RETURN",table="filter"} 12553133
iptables_rule_bytes_total{chain="ufw-logging-deny",rule="-m limit --limit 3/min --limit-burst 10 -j LOG --log-prefix \"[UFW BLOCK] \"",table="filter"} 23494
iptables_rule_bytes_total{chain="ufw-not-local",rule="-j DROP",table="filter"} 0
iptables_rule_bytes_total{chain="ufw-not-local",rule="-m addrtype --dst-type BROADCAST -j RETURN",table="filter"} 0
iptables_rule_bytes_total{chain="ufw-not-local",rule="-m addrtype --dst-type LOCAL -j RETURN",table="filter"} 139808177
iptables_rule_bytes_total{chain="ufw-not-local",rule="-m addrtype --dst-type MULTICAST -j RETURN",table="filter"} 0
iptables_rule_bytes_total{chain="ufw-not-local",rule="-m limit --limit 3/min --limit-burst 10 -j ufw-logging-deny",table="filter"} 0
iptables_rule_bytes_total{chain="ufw-skip-to-policy-forward",rule="-j DROP",table="filter"} 0
iptables_rule_bytes_total{chain="ufw-skip-to-policy-input",rule="-j DROP",table="filter"} 538073
iptables_rule_bytes_total{chain="ufw-skip-to-policy-output",rule="-j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="ufw-track-output",rule="-p tcp -m conntrack --ctstate NEW -j ACCEPT",table="filter"} 71823344
iptables_rule_bytes_total{chain="ufw-track-output",rule="-p udp -m conntrack --ctstate NEW -j ACCEPT",table="filter"} 42827441
iptables_rule_bytes_total{chain="ufw-user-input",rule="-p tcp -m multiport --dports 80,443  -j ACCEPT",table="filter"} 22646850
iptables_rule_bytes_total{chain="ufw-user-input",rule="-p tcp -m tcp --dport 1022 -j ACCEPT",table="filter"} 4572
iptables_rule_bytes_total{chain="ufw-user-input",rule="-p tcp -m tcp --dport 110 -j DROP",table="filter"} 53612
iptables_rule_bytes_total{chain="ufw-user-input",rule="-p tcp -m tcp --dport 22  -j ACCEPT",table="filter"} 17335109
iptables_rule_bytes_total{chain="ufw-user-input",rule="-p tcp -m tcp --dport 25 -j DROP",table="filter"} 199380
iptables_rule_bytes_total{chain="ufw-user-input",rule="-p tcp -m tcp --dport 30000 -j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="ufw-user-input",rule="-p tcp -m tcp --dport 51820 -j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="ufw-user-input",rule="-p tcp -m tcp --dport 587 -j DROP",table="filter"} 56064
iptables_rule_bytes_total{chain="ufw-user-input",rule="-p tcp -m tcp --dport 9001 -j DROP",table="filter"} 24844
iptables_rule_bytes_total{chain="ufw-user-input",rule="-p tcp -m tcp --dport 993 -j DROP",table="filter"} 52200
iptables_rule_bytes_total{chain="ufw-user-input",rule="-p udp -m udp --dport 110 -j DROP",table="filter"} 58
iptables_rule_bytes_total{chain="ufw-user-input",rule="-p udp -m udp --dport 25 -j DROP",table="filter"} 2397
iptables_rule_bytes_total{chain="ufw-user-input",rule="-p udp -m udp --dport 30000 -j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="ufw-user-input",rule="-p udp -m udp --dport 51820 -j ACCEPT",table="filter"} 0
iptables_rule_bytes_total{chain="ufw-user-input",rule="-p udp -m udp --dport 587 -j DROP",table="filter"} 196
iptables_rule_bytes_total{chain="ufw-user-input",rule="-p udp -m udp --dport 993 -j DROP",table="filter"} 254
iptables_rule_bytes_total{chain="ufw-user-limit",rule="-j REJECT --reject-with icmp-port-unreachable",table="filter"} 0
iptables_rule_bytes_total{chain="ufw-user-limit",rule="-m limit --limit 3/min -j LOG --log-prefix \"[UFW LIMIT BLOCK] \"",table="filter"} 0
iptables_rule_bytes_total{chain="ufw-user-limit-accept",rule="-j ACCEPT",table="filter"} 0
# HELP iptables_rule_packets_total Total packets matching a given rule
# TYPE iptables_rule_packets_total counter
iptables_rule_packets_total{chain="DOCKER",rule="! -i br-6379b058093a -p tcp -m tcp --dport 3000 -j DNAT --to-destination 172.18.0.3:3000",table="nat"} 1880
iptables_rule_packets_total{chain="DOCKER",rule="-d 127.0.0.1/32 ! -i br-6379b058093a -p tcp -m tcp --dport 3022 -j DNAT --to-destination 172.18.0.3:22",table="nat"} 0
iptables_rule_packets_total{chain="DOCKER",rule="-d 172.18.0.3/32 ! -i br-6379b058093a -o br-6379b058093a -p tcp -m tcp --dport 22 -j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="DOCKER",rule="-d 172.18.0.3/32 ! -i br-6379b058093a -o br-6379b058093a -p tcp -m tcp --dport 3000 -j ACCEPT",table="filter"} 1880
iptables_rule_packets_total{chain="DOCKER",rule="-i br-6379b058093a -j RETURN",table="nat"} 8
iptables_rule_packets_total{chain="DOCKER",rule="-i docker0 -j RETURN",table="nat"} 0
iptables_rule_packets_total{chain="DOCKER-ISOLATION-STAGE-1",rule="-i br-6379b058093a ! -o br-6379b058093a -j DOCKER-ISOLATION-STAGE-2",table="filter"} 195813
iptables_rule_packets_total{chain="DOCKER-ISOLATION-STAGE-1",rule="-i docker0 ! -o docker0 -j DOCKER-ISOLATION-STAGE-2",table="filter"} 0
iptables_rule_packets_total{chain="DOCKER-ISOLATION-STAGE-1",rule="-j RETURN",table="filter"} 140281499
iptables_rule_packets_total{chain="DOCKER-ISOLATION-STAGE-2",rule="-j RETURN",table="filter"} 195813
iptables_rule_packets_total{chain="DOCKER-ISOLATION-STAGE-2",rule="-o br-6379b058093a -j DROP",table="filter"} 0
iptables_rule_packets_total{chain="DOCKER-ISOLATION-STAGE-2",rule="-o docker0 -j DROP",table="filter"} 0
iptables_rule_packets_total{chain="DOCKER-USER",rule="-j RETURN",table="filter"} 212747854
iptables_rule_packets_total{chain="FORWARD",rule="-i br-6379b058093a ! -o br-6379b058093a -j ACCEPT",table="filter"} 286249
iptables_rule_packets_total{chain="FORWARD",rule="-i br-6379b058093a -o br-6379b058093a -j ACCEPT",table="filter"} 644086
iptables_rule_packets_total{chain="FORWARD",rule="-i docker0 ! -o docker0 -j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="FORWARD",rule="-i docker0 -o docker0 -j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="FORWARD",rule="-j DOCKER-ISOLATION-STAGE-1",table="filter"} 140281499
iptables_rule_packets_total{chain="FORWARD",rule="-j DOCKER-USER",table="filter"} 140281499
iptables_rule_packets_total{chain="FORWARD",rule="-j ts-forward",table="filter"} 140534327
iptables_rule_packets_total{chain="FORWARD",rule="-j ufw-after-forward",table="filter"} 0
iptables_rule_packets_total{chain="FORWARD",rule="-j ufw-after-logging-forward",table="filter"} 0
iptables_rule_packets_total{chain="FORWARD",rule="-j ufw-before-forward",table="filter"} 0
iptables_rule_packets_total{chain="FORWARD",rule="-j ufw-before-logging-forward",table="filter"} 0
iptables_rule_packets_total{chain="FORWARD",rule="-j ufw-reject-forward",table="filter"} 0
iptables_rule_packets_total{chain="FORWARD",rule="-j ufw-track-forward",table="filter"} 0
iptables_rule_packets_total{chain="FORWARD",rule="-o br-6379b058093a -j DOCKER",table="filter"} 646949
iptables_rule_packets_total{chain="FORWARD",rule="-o br-6379b058093a -m conntrack --ctstate RELATED,ESTABLISHED -j ACCEPT",table="filter"} 211814656
iptables_rule_packets_total{chain="FORWARD",rule="-o docker0 -j DOCKER",table="filter"} 0
iptables_rule_packets_total{chain="FORWARD",rule="-o docker0 -m conntrack --ctstate RELATED,ESTABLISHED -j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="INPUT",rule="-j ts-input",table="filter"} 28446758
iptables_rule_packets_total{chain="INPUT",rule="-j ufw-after-input",table="filter"} 1047231
iptables_rule_packets_total{chain="INPUT",rule="-j ufw-after-logging-input",table="filter"} 1036441
iptables_rule_packets_total{chain="INPUT",rule="-j ufw-before-input",table="filter"} 41402627
iptables_rule_packets_total{chain="INPUT",rule="-j ufw-before-logging-input",table="filter"} 41402627
iptables_rule_packets_total{chain="INPUT",rule="-j ufw-reject-input",table="filter"} 1036441
iptables_rule_packets_total{chain="INPUT",rule="-j ufw-track-input",table="filter"} 1036441
iptables_rule_packets_total{chain="OUTPUT",rule="! -d 127.0.0.0/8 -m addrtype --dst-type LOCAL -j DOCKER",table="nat"} 450
iptables_rule_packets_total{chain="OUTPUT",rule="-j ufw-after-logging-output",table="filter"} 1843105
iptables_rule_packets_total{chain="OUTPUT",rule="-j ufw-after-output",table="filter"} 1843105
iptables_rule_packets_total{chain="OUTPUT",rule="-j ufw-before-logging-output",table="filter"} 39955411
iptables_rule_packets_total{chain="OUTPUT",rule="-j ufw-before-output",table="filter"} 39955411
iptables_rule_packets_total{chain="OUTPUT",rule="-j ufw-reject-output",table="filter"} 1843105
iptables_rule_packets_total{chain="OUTPUT",rule="-j ufw-track-output",table="filter"} 1843105
iptables_rule_packets_total{chain="POSTROUTING",rule="-j MASQUERADE",table="nat"} 2279040
iptables_rule_packets_total{chain="POSTROUTING",rule="-j ts-postrouting",table="nat"} 2344043
iptables_rule_packets_total{chain="POSTROUTING",rule="-s 172.17.0.0/16 ! -o docker0 -j MASQUERADE",table="nat"} 104
iptables_rule_packets_total{chain="POSTROUTING",rule="-s 172.18.0.0/16 ! -o br-6379b058093a -j MASQUERADE",table="nat"} 97464
iptables_rule_packets_total{chain="POSTROUTING",rule="-s 172.18.0.3/32 -d 172.18.0.3/32 -p tcp -m tcp --dport 22 -j MASQUERADE",table="nat"} 0
iptables_rule_packets_total{chain="POSTROUTING",rule="-s 172.18.0.3/32 -d 172.18.0.3/32 -p tcp -m tcp --dport 3000 -j MASQUERADE",table="nat"} 0
iptables_rule_packets_total{chain="PREROUTING",rule="-m addrtype --dst-type LOCAL -j DOCKER",table="nat"} 1518992
iptables_rule_packets_total{chain="ts-forward",rule="-i tailscale0 -j MARK --set-xmark 0x40000/0xff0000",table="filter"} 134257
iptables_rule_packets_total{chain="ts-forward",rule="-m mark --mark 0x40000/0xff0000 -j ACCEPT",table="filter"} 134257
iptables_rule_packets_total{chain="ts-forward",rule="-o tailscale0 -j ACCEPT",table="filter"} 126295
iptables_rule_packets_total{chain="ts-postrouting",rule="-m mark --mark 0x40000/0xff0000 -j MASQUERADE",table="nat"} 0
iptables_rule_packets_total{chain="ufw-after-input",rule="-m addrtype --dst-type BROADCAST -j ufw-skip-to-policy-input",table="filter"} 0
iptables_rule_packets_total{chain="ufw-after-input",rule="-p tcp -m tcp --dport 139 -j ufw-skip-to-policy-input",table="filter"} 741
iptables_rule_packets_total{chain="ufw-after-input",rule="-p tcp -m tcp --dport 445 -j ufw-skip-to-policy-input",table="filter"} 9444
iptables_rule_packets_total{chain="ufw-after-input",rule="-p udp -m udp --dport 137 -j ufw-skip-to-policy-input",table="filter"} 560
iptables_rule_packets_total{chain="ufw-after-input",rule="-p udp -m udp --dport 138 -j ufw-skip-to-policy-input",table="filter"} 14
iptables_rule_packets_total{chain="ufw-after-input",rule="-p udp -m udp --dport 67 -j ufw-skip-to-policy-input",table="filter"} 17
iptables_rule_packets_total{chain="ufw-after-input",rule="-p udp -m udp --dport 68 -j ufw-skip-to-policy-input",table="filter"} 14
iptables_rule_packets_total{chain="ufw-after-logging-forward",rule="-m limit --limit 3/min --limit-burst 10 -j LOG --log-prefix \"[UFW BLOCK] \"",table="filter"} 0
iptables_rule_packets_total{chain="ufw-after-logging-input",rule="-m limit --limit 3/min --limit-burst 10 -j LOG --log-prefix \"[UFW BLOCK] \"",table="filter"} 315837
iptables_rule_packets_total{chain="ufw-before-forward",rule="-j ufw-user-forward",table="filter"} 0
iptables_rule_packets_total{chain="ufw-before-forward",rule="-m conntrack --ctstate RELATED,ESTABLISHED -j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="ufw-before-forward",rule="-p icmp -m icmp --icmp-type 11 -j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="ufw-before-forward",rule="-p icmp -m icmp --icmp-type 12 -j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="ufw-before-forward",rule="-p icmp -m icmp --icmp-type 3 -j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="ufw-before-forward",rule="-p icmp -m icmp --icmp-type 8 -j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="ufw-before-input",rule="-d 224.0.0.251/32 -p udp -m udp --dport 5353 -j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="ufw-before-input",rule="-d 239.255.255.250/32 -p udp -m udp --dport 1900 -j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="ufw-before-input",rule="-i lo -j ACCEPT",table="filter"} 15985056
iptables_rule_packets_total{chain="ufw-before-input",rule="-j ufw-not-local",table="filter"} 1733486
iptables_rule_packets_total{chain="ufw-before-input",rule="-j ufw-user-input",table="filter"} 1733486
iptables_rule_packets_total{chain="ufw-before-input",rule="-m conntrack --ctstate INVALID -j DROP",table="filter"} 32617
iptables_rule_packets_total{chain="ufw-before-input",rule="-m conntrack --ctstate INVALID -j ufw-logging-deny",table="filter"} 32617
iptables_rule_packets_total{chain="ufw-before-input",rule="-m conntrack --ctstate RELATED,ESTABLISHED -j ACCEPT",table="filter"} 23334141
iptables_rule_packets_total{chain="ufw-before-input",rule="-p icmp -m icmp --icmp-type 11 -j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="ufw-before-input",rule="-p icmp -m icmp --icmp-type 12 -j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="ufw-before-input",rule="-p icmp -m icmp --icmp-type 3 -j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="ufw-before-input",rule="-p icmp -m icmp --icmp-type 8 -j ACCEPT",table="filter"} 317327
iptables_rule_packets_total{chain="ufw-before-input",rule="-p udp -m udp --sport 67 --dport 68 -j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="ufw-before-output",rule="-j ufw-user-output",table="filter"} 1843105
iptables_rule_packets_total{chain="ufw-before-output",rule="-m conntrack --ctstate RELATED,ESTABLISHED -j ACCEPT",table="filter"} 22127250
iptables_rule_packets_total{chain="ufw-before-output",rule="-o lo -j ACCEPT",table="filter"} 15985056
iptables_rule_packets_total{chain="ufw-logging-allow",rule="-m limit --limit 3/min --limit-burst 10 -j LOG --log-prefix \"[UFW ALLOW] \"",table="filter"} 0
iptables_rule_packets_total{chain="ufw-logging-deny",rule="-m conntrack --ctstate INVALID -m limit --limit 3/min --limit-burst 10 -j RETURN",table="filter"} 31777
iptables_rule_packets_total{chain="ufw-logging-deny",rule="-m limit --limit 3/min --limit-burst 10 -j LOG --log-prefix \"[UFW BLOCK] \"",table="filter"} 133
iptables_rule_packets_total{chain="ufw-not-local",rule="-j DROP",table="filter"} 0
iptables_rule_packets_total{chain="ufw-not-local",rule="-m addrtype --dst-type BROADCAST -j RETURN",table="filter"} 0
iptables_rule_packets_total{chain="ufw-not-local",rule="-m addrtype --dst-type LOCAL -j RETURN",table="filter"} 1733486
iptables_rule_packets_total{chain="ufw-not-local",rule="-m addrtype --dst-type MULTICAST -j RETURN",table="filter"} 0
iptables_rule_packets_total{chain="ufw-not-local",rule="-m limit --limit 3/min --limit-burst 10 -j ufw-logging-deny",table="filter"} 0
iptables_rule_packets_total{chain="ufw-skip-to-policy-forward",rule="-j DROP",table="filter"} 0
iptables_rule_packets_total{chain="ufw-skip-to-policy-input",rule="-j DROP",table="filter"} 10790
iptables_rule_packets_total{chain="ufw-skip-to-policy-output",rule="-j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="ufw-track-output",rule="-p tcp -m conntrack --ctstate NEW -j ACCEPT",table="filter"} 1197023
iptables_rule_packets_total{chain="ufw-track-output",rule="-p udp -m conntrack --ctstate NEW -j ACCEPT",table="filter"} 643584
iptables_rule_packets_total{chain="ufw-user-input",rule="-p tcp -m multiport --dports 80,443  -j ACCEPT",table="filter"} 384990
iptables_rule_packets_total{chain="ufw-user-input",rule="-p tcp -m tcp --dport 1022 -j ACCEPT",table="filter"} 109
iptables_rule_packets_total{chain="ufw-user-input",rule="-p tcp -m tcp --dport 110 -j DROP",table="filter"} 1299
iptables_rule_packets_total{chain="ufw-user-input",rule="-p tcp -m tcp --dport 22  -j ACCEPT",table="filter"} 292613
iptables_rule_packets_total{chain="ufw-user-input",rule="-p tcp -m tcp --dport 25 -j DROP",table="filter"} 3939
iptables_rule_packets_total{chain="ufw-user-input",rule="-p tcp -m tcp --dport 30000 -j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="ufw-user-input",rule="-p tcp -m tcp --dport 51820 -j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="ufw-user-input",rule="-p tcp -m tcp --dport 587 -j DROP",table="filter"} 1351
iptables_rule_packets_total{chain="ufw-user-input",rule="-p tcp -m tcp --dport 9001 -j DROP",table="filter"} 608
iptables_rule_packets_total{chain="ufw-user-input",rule="-p tcp -m tcp --dport 993 -j DROP",table="filter"} 1281
iptables_rule_packets_total{chain="ufw-user-input",rule="-p udp -m udp --dport 110 -j DROP",table="filter"} 1
iptables_rule_packets_total{chain="ufw-user-input",rule="-p udp -m udp --dport 25 -j DROP",table="filter"} 47
iptables_rule_packets_total{chain="ufw-user-input",rule="-p udp -m udp --dport 30000 -j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="ufw-user-input",rule="-p udp -m udp --dport 51820 -j ACCEPT",table="filter"} 0
iptables_rule_packets_total{chain="ufw-user-input",rule="-p udp -m udp --dport 587 -j DROP",table="filter"} 7
iptables_rule_packets_total{chain="ufw-user-input",rule="-p udp -m udp --dport 993 -j DROP",table="filter"} 8
iptables_rule_packets_total{chain="ufw-user-limit",rule="-j REJECT --reject-with icmp-port-unreachable",table="filter"} 0
iptables_rule_packets_total{chain="ufw-user-limit",rule="-m limit --limit 3/min -j LOG --log-prefix \"[UFW LIMIT BLOCK] \"",table="filter"} 0
iptables_rule_packets_total{chain="ufw-user-limit-accept",rule="-j ACCEPT",table="filter"} 0
# HELP iptables_rules_total Total number of rules in a chain in a table
# TYPE iptables_rules_total gauge
iptables_rules_total{chain="DOCKER",table="filter"} 2
iptables_rules_total{chain="DOCKER",table="nat"} 4
iptables_rules_total{chain="DOCKER-ISOLATION-STAGE-1",table="filter"} 3
iptables_rules_total{chain="DOCKER-ISOLATION-STAGE-2",table="filter"} 3
iptables_rules_total{chain="DOCKER-USER",table="filter"} 1
iptables_rules_total{chain="FORWARD",table="filter"} 17
iptables_rules_total{chain="INPUT",table="filter"} 7
iptables_rules_total{chain="INPUT",table="nat"} 0
iptables_rules_total{chain="OUTPUT",table="filter"} 6
iptables_rules_total{chain="OUTPUT",table="nat"} 1
iptables_rules_total{chain="POSTROUTING",table="nat"} 6
iptables_rules_total{chain="PREROUTING",table="nat"} 3
iptables_rules_total{chain="ts-forward",table="filter"} 4
iptables_rules_total{chain="ts-input",table="filter"} 3
iptables_rules_total{chain="ts-postrouting",table="nat"} 1
iptables_rules_total{chain="ufw-after-forward",table="filter"} 0
iptables_rules_total{chain="ufw-after-input",table="filter"} 7
iptables_rules_total{chain="ufw-after-logging-forward",table="filter"} 1
iptables_rules_total{chain="ufw-after-logging-input",table="filter"} 1
iptables_rules_total{chain="ufw-after-logging-output",table="filter"} 0
iptables_rules_total{chain="ufw-after-output",table="filter"} 0
iptables_rules_total{chain="ufw-before-forward",table="filter"} 6
iptables_rules_total{chain="ufw-before-input",table="filter"} 13
iptables_rules_total{chain="ufw-before-logging-forward",table="filter"} 0
iptables_rules_total{chain="ufw-before-logging-input",table="filter"} 0
iptables_rules_total{chain="ufw-before-logging-output",table="filter"} 0
iptables_rules_total{chain="ufw-before-output",table="filter"} 3
iptables_rules_total{chain="ufw-logging-allow",table="filter"} 1
iptables_rules_total{chain="ufw-logging-deny",table="filter"} 2
iptables_rules_total{chain="ufw-not-local",table="filter"} 5
iptables_rules_total{chain="ufw-reject-forward",table="filter"} 0
iptables_rules_total{chain="ufw-reject-input",table="filter"} 0
iptables_rules_total{chain="ufw-reject-output",table="filter"} 0
iptables_rules_total{chain="ufw-skip-to-policy-forward",table="filter"} 1
iptables_rules_total{chain="ufw-skip-to-policy-input",table="filter"} 1
iptables_rules_total{chain="ufw-skip-to-policy-output",table="filter"} 1
iptables_rules_total{chain="ufw-track-forward",table="filter"} 0
iptables_rules_total{chain="ufw-track-input",table="filter"} 0
iptables_rules_total{chain="ufw-track-output",table="filter"} 2
iptables_rules_total{chain="ufw-user-forward",table="filter"} 0
iptables_rules_total{chain="ufw-user-input",table="filter"} 17
iptables_rules_total{chain="ufw-user-limit",table="filter"} 2
iptables_rules_total{chain="ufw-user-limit-accept",table="filter"} 1
iptables_rules_total{chain="ufw-user-logging-forward",table="filter"} 0
iptables_rules_total{chain="ufw-user-logging-input",table="filter"} 0
iptables_rules_total{chain="ufw-user-logging-output",table="filter"} 0
iptables_rules_total{chain="ufw-user-output",table="filter"} 0
# HELP iptables_scrape_duration_milliseconds Duration in milliseconds of the scrape
# TYPE iptables_scrape_duration_milliseconds gauge
iptables_scrape_duration_milliseconds 6
# HELP iptables_scrape_success If the scrape was a success
# TYPE iptables_scrape_success gauge
iptables_scrape_success 1
# HELP prometheus_exporter_request_duration_seconds HTTP request durations in seconds
# TYPE prometheus_exporter_request_duration_seconds histogram
prometheus_exporter_request_duration_seconds_bucket{le="0.005"} 4
prometheus_exporter_request_duration_seconds_bucket{le="0.01"} 4
prometheus_exporter_request_duration_seconds_bucket{le="0.025"} 4
prometheus_exporter_request_duration_seconds_bucket{le="0.05"} 4
prometheus_exporter_request_duration_seconds_bucket{le="0.1"} 4
prometheus_exporter_request_duration_seconds_bucket{le="0.25"} 4
prometheus_exporter_request_duration_seconds_bucket{le="0.5"} 4
prometheus_exporter_request_duration_seconds_bucket{le="1"} 4
prometheus_exporter_request_duration_seconds_bucket{le="2.5"} 4
prometheus_exporter_request_duration_seconds_bucket{le="5"} 4
prometheus_exporter_request_duration_seconds_bucket{le="10"} 4
prometheus_exporter_request_duration_seconds_bucket{le="+Inf"} 4
prometheus_exporter_request_duration_seconds_sum 0.006526718000000001
prometheus_exporter_request_duration_seconds_count 4
# HELP prometheus_exporter_requests_total HTTP requests received
# TYPE prometheus_exporter_requests_total counter
prometheus_exporter_requests_total 5
# HELP prometheus_exporter_response_size_bytes HTTP response sizes in bytes
# TYPE prometheus_exporter_response_size_bytes gauge
prometheus_exporter_response_size_bytes 43102
```

# License

This project is dual licensed under the terms of either the MIT or Apache 2.0
at your option.
