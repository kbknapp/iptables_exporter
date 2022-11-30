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
    /// Show verbose output at a level or higher. -v:  DEBUG, -vv: TRACE
    #[arg(long, short, action = ArgAction::Count)]
    pub(crate) verbose: u8,
    /// Supress output at a level or lower. -q: INFO, -qq: WARN, -qqq: ERROR (i.e. everything)
    #[arg(long, short, overrides_with = "verbose", action = ArgAction::Count)]
    pub(crate) quiet: u8,
}
