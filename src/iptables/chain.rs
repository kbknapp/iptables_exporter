use anyhow::Result;

use crate::{
    error::IptablesError,
    iptables::{Counter, Policy, Rule},
    parse::idx_after,
};

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct Chain {
    pub(crate) name: String,
    pub(crate) counter: Counter,
    // Technically we could use policy to determine if it's builtin or not
    builtin: bool,
    policy: Option<Policy>,
    pub(crate) rules: Vec<Rule>,
}

impl Chain {
    pub(crate) fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            counter: Counter::default(),
            builtin: true,
            policy: None,
            rules: Vec::new(),
        }
    }

    // :POSTROUTING ACCEPT [3907:262379]
    //              ^       ^    ^ Bytes
    //              ^       ^ Packets
    //              ^ Policy
    //
    // :DOCKER-ISOLATION - [0:0]
    //                   ^ Custom
    pub(crate) fn parse_chain<S: AsRef<str>>(line: S) -> Result<Self> {
        let line = line.as_ref();
        let len = line.len();

        let name_start = idx_after(0, line, ':').unwrap_or(0);
        let name_end = idx_after(name_start, line, ' ').unwrap_or(len - 2); // -2 for name_end + 2 below

        let mut chain = Chain::new(line[name_start + 1..name_end].trim());

        match line[name_end..name_end + 2].trim() {
            "-" => chain.builtin = false,
            _ => {
                let policy_start = name_end;
                let policy_end = idx_after(policy_start + 1, line, ' ').unwrap_or(len);

                let policy = line[policy_start..policy_end].trim();
                chain.policy = Some(
                    policy
                        .parse::<Policy>()
                        .map_err(|_| IptablesError::InvalidPolicy(policy.into()))?,
                );
            }
        }

        chain.counter = Counter::parse_counter(line);

        Ok(chain)
    }

    pub(crate) fn policy(&self) -> &str {
        match self.policy {
            Some(Policy::Drop) => "DROP",
            Some(Policy::Accept) | None => "ACCEPT",
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn chain_parse_builtin() {
        assert_eq!(
            Chain::parse_chain(":POSTROUTING ACCEPT [3907:262379]").unwrap(),
            Chain {
                name: "POSTROUTING".into(),
                builtin: true,
                policy: Some(Policy::Accept),
                counter: Counter {
                    packets: 3907,
                    bytes: 262379
                },
                rules: Vec::new(),
            }
        );
    }

    #[test]
    fn chain_parse_custom() {
        assert_eq!(
            Chain::parse_chain(":DOCKER-ISOLATION - [0:0]").unwrap(),
            Chain {
                name: "DOCKER-ISOLATION".into(),
                builtin: false,
                policy: None,
                counter: Counter {
                    packets: 0,
                    bytes: 0,
                },
                rules: Vec::new(),
            }
        );
    }
}
