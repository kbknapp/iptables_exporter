#[macro_use]
mod macros;
mod cli;
mod error;
mod iptables;
mod parse;

use std::{
    collections::HashMap,
    env,
    net::SocketAddr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};

use clap::Parser;
use prometheus::{IntGauge, Registry};
use prometheus_hyper::Server;
use tokio::time::{Duration, Instant};
use tracing::{debug, info};

use crate::iptables::{iptables_save, IptablesState, Metrics};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = cli::Args::parse();

    match args.verbose {
        0 => match args.quiet {
            0 => env::set_var("RUST_LOG", "iptables_exporter=info"),
            1 => env::set_var("RUST_LOG", "iptables_exporter=warn"),
            2 => env::set_var("RUST_LOG", "iptables_exporter=error"),
            _ => env::set_var("RUST_LOG", "iptables_exporter=off"),
        },
        1 => env::set_var("RUST_LOG", "iptables_exporter=debug"),
        _ => env::set_var("RUST_LOG", "iptables_exporter=trace"),
    }

    tracing_subscriber::fmt::init();

    let running = Arc::new(AtomicBool::new(true));

    info!("Registering metrics...");
    let registry = Arc::new(Registry::new());
    let metrics = Arc::new(Mutex::new(unwrap_or_exit!(Metrics::new(
        &args.scrape_targets,
        &registry
    ))));

    let mut scrape_durations = HashMap::new();
    let mut scrape_successes = HashMap::new();
    for tgt in &args.scrape_targets {
        let tgt_str = tgt.to_string();
        let prefix = tgt_str.replace('-', "_");
        let d = unwrap_or_exit!(IntGauge::new(
            &format!("{prefix}_scrape_duration_milliseconds"),
            "Duration in milliseconds of the scrape",
        ));
        let s = unwrap_or_exit!(IntGauge::new(
            &format!("{prefix}_scrape_success"),
            "If the scrape was a success"
        ));
        scrape_durations.insert(tgt_str.clone(), d.clone());
        scrape_successes.insert(tgt_str.clone(), s.clone());
        debug!("Registering {tgt_str} scrape metrics...");
        unwrap_or_exit!(registry.register(Box::new(d)));
        unwrap_or_exit!(registry.register(Box::new(s)));
    }

    let scrape_durations = Arc::new(Mutex::new(scrape_durations));
    let scrape_successes = Arc::new(Mutex::new(scrape_successes));

    info!("Spawning server...");
    tokio::spawn(Server::run(
        Arc::clone(&registry),
        SocketAddr::new(args.listen_address, args.listen_port),
        shutdown_signal(Arc::clone(&running)),
    ));

    let mut collect_int = tokio::time::interval(Duration::from_secs(args.collect_interval));
    while running.load(Ordering::Relaxed) {
        // .cloned() required becuase of async move
        #[allow(clippy::unnecessary_to_owned)]
        for tgt in args.scrape_targets.iter().cloned() {
            let scrape_durations = scrape_durations.clone();
            let scrape_successes = scrape_successes.clone();
            let metrics = metrics.clone();
            tokio::task::spawn(async move {
                info!("Collecting {tgt} metrics...");
                let before = Instant::now();
                let out = unwrap_or_exit!(iptables_save(tgt).await);

                let mut state = IptablesState::new();
                unwrap_or_exit!(state.parse(&*out).await);

                debug!("Updating {tgt} metrics...");
                if let Ok(mut guard) = metrics.lock() {
                    guard.update(tgt, &state);
                };
                let after = Instant::now();

                let elapsed = after.duration_since(before);
                if let Ok(mut guard) = scrape_durations.lock() {
                    let guage = guard.get_mut(&tgt.to_string()).unwrap();
                    guage.set((elapsed.as_secs() * 1000 + elapsed.subsec_millis() as u64) as i64);
                }
                if let Ok(mut guard) = scrape_successes.lock() {
                    let guage = guard.get_mut(&tgt.to_string()).unwrap();
                    guage.set(1);
                }
            });
        }
        debug!("Sleeping...");
        collect_int.tick().await;
    }
    info!("Stopped the exporter");
}

async fn shutdown_signal(running: Arc<AtomicBool>) {
    // Wait for the CTRL+C Signal
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
    running.store(false, Ordering::Relaxed);
}
