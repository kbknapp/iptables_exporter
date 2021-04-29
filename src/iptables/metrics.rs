use anyhow::Result;
use prometheus::{IntCounterVec, IntGaugeVec, Opts, Registry};
use tracing::{debug, trace};

use crate::iptables::IptablesState;

pub(crate) struct Metrics {
    chains_total: IntGaugeVec,
    rules_total: IntGaugeVec,
    chain_bytes_total: IntCounterVec,
    chain_packets_total: IntCounterVec,
    rule_bytes_total: IntCounterVec,
    rule_packets_total: IntCounterVec,
}

impl Metrics {
    pub(crate) fn new(r: &Registry) -> Result<Self> {
        trace!("Metrics::new");

        let chains_total = IntGaugeVec::new(
            Opts::new("iptables_chains_total", "Total number of chains in a table"),
            &["table"],
        )?;

        let rules_total = IntGaugeVec::new(
            Opts::new(
                "iptables_rules_total",
                "Total number of rules in a chain in a table",
            ),
            &["table", "chain"],
        )?;

        let chain_bytes_total = IntCounterVec::new(
            Opts::new(
                "iptables_chain_bytes_total",
                "Total bytes flowing through a given chain",
            ),
            &["table", "chain", "policy"],
        )?;

        let chain_packets_total = IntCounterVec::new(
            Opts::new(
                "iptables_chain_packets_total",
                "Total packets flowing through a given chain",
            ),
            &["table", "chain", "policy"],
        )?;

        let rule_bytes_total = IntCounterVec::new(
            Opts::new(
                "iptables_rule_bytes_total",
                "Total bytes matching a given rule",
            ),
            &["table", "chain", "rule"],
        )?;

        let rule_packets_total = IntCounterVec::new(
            Opts::new(
                "iptables_rule_packets_total",
                "Total packets matching a given rule",
            ),
            &["table", "chain", "rule"],
        )?;

        debug!("Registering iptables metrics");
        r.register(Box::new(chain_bytes_total.clone()))?;
        r.register(Box::new(chain_packets_total.clone()))?;
        r.register(Box::new(rule_bytes_total.clone()))?;
        r.register(Box::new(rule_packets_total.clone()))?;
        r.register(Box::new(rules_total.clone()))?;
        r.register(Box::new(chains_total.clone()))?;

        Ok(Self {
            chains_total,
            rules_total,
            chain_bytes_total,
            chain_packets_total,
            rule_bytes_total,
            rule_packets_total,
        })
    }

    pub(crate) async fn update(&mut self, state: &IptablesState) {
        for t in &state.tables {
            let ct = self.chains_total.with_label_values(&[&t.name]);
            ct.set(t.chains.len() as i64);

            for (_, c) in t.chains.iter() {
                let cbt = self
                    .chain_bytes_total
                    .with_label_values(&[&t.name, &c.name, c.policy()]);
                let diff = c.counter.bytes() - cbt.get();
                cbt.inc_by(diff);

                let cpt =
                    self.chain_packets_total
                        .with_label_values(&[&t.name, &c.name, c.policy()]);
                let diff = c.counter.packets() - cpt.get();
                cpt.inc_by(diff);

                let rt = self.rules_total.with_label_values(&[&t.name, &c.name]);
                rt.set(c.rules.len() as i64);

                for r in &c.rules {
                    let rpt = self
                        .rule_packets_total
                        .with_label_values(&[&t.name, &c.name, &r.rule]);
                    let diff = r.counter.packets() - rpt.get();
                    rpt.inc_by(diff);

                    let rbt = self
                        .rule_bytes_total
                        .with_label_values(&[&t.name, &c.name, &r.rule]);
                    let diff = r.counter.bytes() - rbt.get();
                    rbt.inc_by(diff);
                }
            }
        }
    }
}
