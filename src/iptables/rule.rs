use crate::{iptables::Counter, parse::idx_after};

use anyhow::Result;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Rule {
    pub(crate) rule: String,
    pub(crate) counter: Counter,
}

impl Rule {
    pub(crate) fn parse_rule<S: AsRef<str>>(line: S) -> Result<(String, Self)> {
        let line = line.as_ref();
        let counter = Counter::parse_counter(line);
        let counter_end = idx_after(0, line, ']').unwrap_or(0);
        let flag = idx_after(counter_end, line, 'A').unwrap_or(0);
        let chain_start = idx_after(flag, line, ' ').unwrap_or(0);
        let chain_end = idx_after(chain_start + 2, line, ' ').unwrap_or_else(|| line.len());
        let chain = line[chain_start..chain_end].trim().into();
        Ok((
            chain,
            Self {
                rule: line[chain_end..].trim().into(),
                counter,
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rule_parse_no_traffic() {
        assert_eq!(
            Rule::parse_rule("[0:0] -A LIBVIRT_PRT -s 192.168.122.0/24 ! -d 192.168.122.0/24 -p tcp -j MASQUERADE --to-ports 1024-65535").unwrap(),
            ("LIBVIRT_PRT".into(),
             Rule {
                rule: "-s 192.168.122.0/24 ! -d 192.168.122.0/24 -p tcp -j MASQUERADE --to-ports 1024-65535".into(),
                counter: Counter {
                    packets: 0,
                    bytes: 0,
                },
            })
        );
    }

    #[test]
    fn rule_parse_traffic() {
        assert_eq!(
            Rule::parse_rule("[607613:364557889] -A POSTROUTING -j LIBVIRT_PRT").unwrap(),
            (
                "POSTROUTING".into(),
                Rule {
                    rule: "-j LIBVIRT_PRT".into(),
                    counter: Counter {
                        packets: 607613,
                        bytes: 364557889,
                    },
                }
            )
        );
    }
}
