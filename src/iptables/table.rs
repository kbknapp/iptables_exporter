use std::collections::HashMap;

use anyhow::Result;

use crate::iptables::{Chain, Rule};

#[derive(Clone, Debug)]
pub(crate) struct Table {
    pub(crate) name: String,
    pub(crate) chains: HashMap<String, Chain>,
}

impl Table {
    pub(crate) fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            chains: HashMap::new(),
        }
    }

    pub(crate) fn parse_chain<S: AsRef<str>>(&mut self, line: S) -> Result<()> {
        let chain = Chain::parse_chain(line)?;
        self.chains.insert(chain.name.clone(), chain);
        Ok(())
    }

    pub(crate) fn parse_rule<S: AsRef<str>>(&mut self, line: S) -> Result<()> {
        let (chain, rule) = Rule::parse_rule(line)?;
        let chain = self
            .chains
            .entry(chain.clone())
            .or_insert_with(|| Chain::new(chain));

        chain.rules.push(rule);
        Ok(())
    }
}
