use clap::Parser;

fn main() -> anyhow::Result<()> {
    let opts = espflash::cargo_espflash::Opts::parse();
    // let opts = espflash::espflash::Opts::parse();

    tracing_subscriber::fmt()
        .with_max_level(convert_filter(opts.verbose.log_level_filter()))
        .init();

    println!("{:#?}", opts);

    Ok(())
}

fn convert_filter(filter: log::LevelFilter) -> tracing_subscriber::filter::LevelFilter {
    match filter {
        log::LevelFilter::Off => tracing_subscriber::filter::LevelFilter::OFF,
        log::LevelFilter::Error => tracing_subscriber::filter::LevelFilter::ERROR,
        log::LevelFilter::Warn => tracing_subscriber::filter::LevelFilter::WARN,
        log::LevelFilter::Info => tracing_subscriber::filter::LevelFilter::INFO,
        log::LevelFilter::Debug => tracing_subscriber::filter::LevelFilter::DEBUG,
        log::LevelFilter::Trace => tracing_subscriber::filter::LevelFilter::TRACE,
    }
}
