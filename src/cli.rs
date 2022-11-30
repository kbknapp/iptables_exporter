use std::{env, net::IpAddr};

use clap::{crate_authors, ArgAction, Parser};

static VERSION: &str = env!("VERSION_WITH_GIT_HASH");
static AUTHORS: &str = crate_authors!();

/// A Prometheus exporter for iptables
#[derive(Parser)]
#[command(author = AUTHORS, version = VERSION)]
pub(crate) struct Args {
    /// How often metrics are gathered
    #[arg(long, default_value = "5", value_name = "SECS")]
    pub(crate) collect_interval: u64,
    /// The listen port for scraping metrics
    #[arg(short = 'p', long, default_value = "9455", value_name = "PORT")]
    pub(crate) listen_port: u16,
    /// The listen address scraping metrics
    #[arg(short, long, default_value = "0.0.0.0", value_name = "ADDR")]
    pub(crate) listen_address: IpAddr,
    /// Which backends to scrape for metrics, multiple targets can be enabled at
    /// once by using this flag multiple times
    #[arg(short = 't', long, visible_alias = "scrape-target", default_value = "iptables", action = ArgAction::Append, value_enum, value_name = "TARGET")]
    pub(crate) scrape_targets: Vec<ScrapeTarget>,
    /// Show verbose output at a level or higher. -v:  DEBUG, -vv: TRACE
    #[arg(long, short, action = ArgAction::Count)]
    pub(crate) verbose: u8,
    /// Supress output at a level or lower. -q: INFO, -qq: WARN, -qqq: ERROR
    /// (i.e. everything)
    #[arg(long, short, overrides_with = "verbose", action = ArgAction::Count)]
    pub(crate) quiet: u8,
}

#[derive(clap::ValueEnum, PartialEq, Eq, Copy, Clone, Debug, strum::AsRefStr, strum::Display)]
pub(crate) enum ScrapeTarget {
    /// enable 'iptables-save' for metrics
    #[strum(serialize = "iptables")]
    Iptables,
    /// enable 'ip6tables-save' for metrics
    #[strum(serialize = "ip6tables")]
    Ip6tables,
    /// enable 'iptables-legacy-save' for metrics
    #[strum(serialize = "iptables-legacy")]
    IptablesLegacy,
    /// enable 'ip6tables-legacy-save' for metrics
    #[strum(serialize = "ip6tables-legacy")]
    Ip6tablesLegacy,
}
