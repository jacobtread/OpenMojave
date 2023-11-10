use std::time::SystemTime;

use log::LevelFilter;

/// Creates and applies the logger and the panic logger
pub fn setup_logger(level: LevelFilter) {
    let log_file = fern::log_file("engine.log").expect("Failed to create engine log file");

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(LevelFilter::Warn)
        .level_for("open_mojave", level)
        // Logging to stdout
        .chain(std::io::stdout())
        // Logging to file
        .chain(log_file)
        .apply()
        .expect("Failed to apply logger");

    // Enable panic logging
    log_panics::init();
}
