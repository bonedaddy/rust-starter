//! TODO: this needs to be overhauled and is temporary just to get logging in place

use tracing_subscriber::{filter::LevelFilter, util::SubscriberInitExt};

/// initializes a logger, specifically avoiding
/// a panic if the logger has already been initialized
///
/// it is equivalent to calling tracing_subscriber::fmt::init() but with
/// the enablement of file, and line numbers
pub fn init_logger(debug: bool) {
    let rust_log_conf = std::env::var("RUST_LOG").unwrap_or_default();
    let rust_log_conf = if rust_log_conf.is_empty() {
        if debug {
            "debug".to_string()
        } else {
            "info".to_string()
        }
    } else {
        format!("{rust_log_conf},sqlx=warn,sqlx_ext=warn")
    };
    std::env::set_var("RUST_LOG", &rust_log_conf);
    use tracing_subscriber::fmt::Subscriber;
    let builder = Subscriber::builder();

    #[cfg(feature = "env-filter")]
    let builder = builder.with_env_filter(crate::EnvFilter::from_default_env());

    // If `env-filter` is disabled, remove the default max level filter from the
    // subscriber; it will be added to the `Targets` filter instead if no filter
    // is set in `RUST_LOG`.
    // Replacing the default `LevelFilter` with an `EnvFilter` would imply this,
    // but we can't replace the builder's filter with a `Targets` filter yet.
    #[cfg(not(feature = "env-filter"))]
    let builder = builder.with_max_level(LevelFilter::TRACE);

    let builder = builder.with_file(true).with_line_number(true);

    let subscriber = builder.finish();
    #[cfg(not(feature = "env-filter"))]
    let subscriber = {
        use std::{env, str::FromStr};
        use tracing_subscriber::{filter::Targets, layer::SubscriberExt};
        let targets = match env::var("RUST_LOG") {
            Ok(var) => Targets::from_str(&var)
                .map_err(|e| {
                    eprintln!("Ignoring `RUST_LOG={var:?}`: {e}");
                })
                .unwrap_or_default(),
            Err(env::VarError::NotPresent) => {
                Targets::new().with_default(Subscriber::DEFAULT_MAX_LEVEL)
            }
            Err(e) => {
                eprintln!("Ignoring `RUST_LOG`: {e}");
                Targets::new().with_default(Subscriber::DEFAULT_MAX_LEVEL)
            }
        };
        subscriber.with(targets)
    };
    if let Err(err) = subscriber.try_init() {
        log::error!("failed to initialize log system {:#?}", err);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_logger() {
        std::thread::spawn(|| {
            init_logger(false);
            log::info!("info log");
            log::debug!("debug log");
        })
        .join()
        .unwrap();
    }
    #[test]
    fn test_debug_logger() {
        std::thread::spawn(|| {
            init_logger(true);
            log::info!("info log");
            log::debug!("debug log");
        })
        .join()
        .unwrap();
    }
}
