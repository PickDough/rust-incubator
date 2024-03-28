use std::fs::File;
use std::io;
use std::sync::Arc;

use log::{error, info};
use tracing::instrument::WithSubscriber;
use tracing::{Event, Subscriber};
use tracing_core::Level;
use tracing_subscriber::filter::{LevelFilter, Targets};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::{format, FmtContext, FormatEvent, FormatFields, FormattedFields};
use tracing_subscriber::prelude::*;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::{filter, prelude::*};
use tracing_subscriber::{fmt, EnvFilter, Registry};

fn main() {
    tracing_log::LogTracer::init().expect("Failed to set logger");

    let timer = time::format_description::well_known::Rfc3339;
    let timer = fmt::time::UtcTime::new(timer);

    let stdout_log = tracing_subscriber::fmt::layer()
        .with_writer(io::stdout)
        .with_target(false)
        .with_file(false)
        .with_timer(timer.clone())
        .json()
        .with_filter(LevelFilter::from_level(Level::TRACE));
    let stderr_log = tracing_subscriber::fmt::layer()
        .with_writer(io::stderr)
        .with_timer(timer.clone())
        .pretty()
        .with_filter(LevelFilter::from_level(Level::WARN));
    let file_log = tracing_subscriber::fmt::layer()
        .with_writer(File::create("3_ecosystem/3_8_log/access.log").unwrap())
        .with_timer(timer)
        .json()
        .with_filter(Targets::new().with_target("access.log", Level::DEBUG));
    let subscriber = Registry::default()
        .with(file_log)
        .with(stdout_log)
        .with(stderr_log);

    tracing::subscriber::set_global_default(subscriber).expect("Unable to set global subscriber");

    // Log an info and an error message
    info!("This will be written to stdout");
    error!("This will be written to stderr");
    info!(target: "access.log", "xd");
}
