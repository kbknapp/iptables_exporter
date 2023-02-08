use crate::{iptables::Counter, parse::idx_after};

use anyhow::Result;

macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Comment {
    pub(crate) comment: String,
    pub(crate) counter: Counter,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Rule {
    pub(crate) rule: String,
    pub(crate) counter: Counter,
    pub(crate) comment: Option<Comment>,
}

impl Rule {
    pub(crate) fn parse_rule<S: AsRef<str>>(line: S) -> Result<(String, Self)> {
        let line = line.as_ref();
        let counter = Counter::parse_counter(line);
        let counter_end = idx_after(0, line, ']').unwrap_or(0);
        let flag = idx_after(counter_end, line, 'A').unwrap_or(0);
        let chain_start = idx_after(flag, line, ' ').unwrap_or(0);
        let chain_end = idx_after(chain_start + 2, line, ' ').unwrap_or(line.len());
        let chain = line[chain_start..chain_end].trim().into();
        let after_chain = line[chain_end..].trim();
        let re = regex!(r#"--comment +"((?:[^"\\]|\\.)*)""#);
        let re_no_quotes = regex!(r#"--comment +((?:[^ "\\]|\\.)*)"#);
        let comment = if let Some(caps) = re.captures(after_chain) {
            caps.get(1).map(|m| m.as_str().to_owned())
        } else if let Some(caps) = re_no_quotes.captures(after_chain) {
            caps.get(1).map(|m| m.as_str().to_owned())
        } else {
            None
        };

        Ok((
            chain,
            Self {
                rule: line[chain_end..].trim().into(),
                comment: comment.map(|s| Comment { comment: s, counter: Counter::default() }),
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
                comment: None,
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
                    comment: None,
                    counter: Counter {
                        packets: 607613,
                        bytes: 364557889,
                    },
                }
            )
        );
    }

    #[test]
    fn rule_parse_comment() {
        assert_eq!(
            Rule::parse_rule("[607613:364557889] -A POSTROUTING -j LIBVIRT_PRT -m comment --comment \"test comment\"").unwrap(),
            (
                "POSTROUTING".into(),
                Rule {
                    rule: "-j LIBVIRT_PRT -m comment --comment \"test comment\"".into(),
                    comment: Some("test comment".into()),
                    counter: Counter {
                        packets: 607613,
                        bytes: 364557889,
                    },
                }
            )
        );
    }

    #[test]
    fn rule_parse_comment_escapes() {
        assert_eq!(
            Rule::parse_rule(
                "[0:0] -A POSTROUTING -m comment --comment   \"test 'foo' \\\"comment\""
            )
            .unwrap(),
            (
                "POSTROUTING".into(),
                Rule {
                    rule: "-m comment --comment   \"test 'foo' \\\"comment\"".into(),
                    comment: Some("test 'foo' \\\"comment".into()),
                    counter: Counter::default()
                }
            )
        );
    }

    #[test]
    fn rule_parse_comment_no_quote() {
        assert_eq!(
            Rule::parse_rule("[0:0] -A POSTROUTING -m comment --comment   TEST-COMMENT").unwrap(),
            (
                "POSTROUTING".into(),
                Rule {
                    rule: "-m comment --comment   TEST-COMMENT".into(),
                    comment: Some("TEST-COMMENT".into()),
                    counter: Counter::default(),
                }
            )
        );
    }
}
