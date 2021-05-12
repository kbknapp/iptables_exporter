use std::{
    env,
    net::SocketAddr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use clap::*;
use prometheus::{IntGauge, Registry};
use prometheus_hyper::Server;
use tokio::time::{Duration, Instant};
use tracing::{debug, info};

#[macro_use]
mod macros;
mod cli;
mod error;
mod iptables;
mod parse;

use iptables::{iptables_save, IptablesState, Metrics};

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
    let mut metrics = unwrap_or_exit!(Metrics::new(&registry));
    let scrape_duration = unwrap_or_exit!(IntGauge::new(
        "iptables_scrape_duration_milliseconds",
        "Duration in milliseconds of the scrape",
    ));

    let scrape_success = unwrap_or_exit!(IntGauge::new(
        "iptables_scrape_success",
        "If the scrape was a success"
    ));
    debug!("Registering scrape metrics...");
    unwrap_or_exit!(registry.register(Box::new(scrape_duration.clone())));
    unwrap_or_exit!(registry.register(Box::new(scrape_success.clone())));

    info!("Spawning server...");
    tokio::spawn(Server::run(
        Arc::clone(&registry),
        SocketAddr::new(args.listen_address, args.listen_port),
        shutdown_signal(Arc::clone(&running)),
    ));

    let mut collect_int = tokio::time::interval(Duration::from_secs(args.collect_interval));
    while running.load(Ordering::Relaxed) {
        info!("Collecting metrics...");
        let before = Instant::now();
        let out = unwrap_or_exit!(iptables_save().await);
        let mut state = IptablesState::new();
        unwrap_or_exit!(state.parse(&*out).await);

        debug!("Updating metrics...");
        metrics.update(&state).await;
        let after = Instant::now();

        let elapsed = after.duration_since(before);
        scrape_duration.set((elapsed.as_secs() * 1000 + elapsed.subsec_millis() as u64) as i64);
        scrape_success.set(1);

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
