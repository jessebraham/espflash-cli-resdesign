use clap::Parser;
use clap_verbosity_flag::Verbosity;
use espflash::{
    cli::{
        logging::initialize_logger,
        BoardInfoOpts,
        ConnectOpts,
        FlashConfigOpts,
        FlashOpts as BaseFlashOpts,
        MonitorOpts,
        PartitionTableOpts,
        SaveImageOpts as BaseSaveImageOpts,
    },
    enums::ImageFormat,
};
use strum::VariantNames;

#[derive(Debug, Parser)]
#[clap(bin_name = "cargo", propagate_version = true, version)]
pub struct Opts {
    #[clap(subcommand)]
    subcommand: CargoSubcommand,
    #[clap(flatten)]
    pub verbose: Verbosity,
}

#[derive(Debug, Parser)]
pub enum CargoSubcommand {
    Espflash {
        #[clap(subcommand)]
        subcommand: Subcommand,
    },
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    BoardInfo(BoardInfoOpts),
    Flash(FlashOpts),
    Monitor(MonitorOpts),
    PartitionTable(PartitionTableOpts),
    SaveImage(SaveImageOpts),
}

#[derive(Debug, Parser)]
pub struct BuildOpts {
    /// Example to build and flash
    #[clap(long)]
    pub example: Option<String>,
    /// Comma delimited list of build features
    #[clap(long, use_value_delimiter = true)]
    pub features: Option<Vec<String>>,
    /// Image format to flash
    #[clap(long, possible_values = ImageFormat::VARIANTS)]
    pub format: Option<String>,
    /// Require Cargo.lock and cache are up to date
    #[clap(long)]
    pub frozen: bool,
    /// Require Cargo.lock is up to date
    #[clap(long)]
    pub locked: bool,
    /// Specify a (binary) package within a workspace to be built
    #[clap(long)]
    pub package: Option<String>,
    /// Build the application using the release profile
    #[clap(long)]
    pub release: bool,
    /// Target to build for
    #[clap(long)]
    pub target: Option<String>,
    /// Directory for all generated artifacts
    #[clap(long)]
    pub target_dir: Option<String>,
    /// Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
    #[clap(short = 'Z')]
    pub unstable: Option<Vec<String>>,

    #[clap(flatten)]
    pub flash_config_opts: FlashConfigOpts,
}

/// Build and flash an application to a target device
#[derive(Debug, Parser)]
pub struct FlashOpts {
    #[clap(flatten)]
    build_opts: BuildOpts,
    #[clap(flatten)]
    connect_opts: ConnectOpts,
    #[clap(flatten)]
    flash_opts: BaseFlashOpts,
}

#[derive(Debug, Parser)]
pub struct SaveImageOpts {
    #[clap(flatten)]
    build_opts: BuildOpts,
    #[clap(flatten)]
    save_image_opts: BaseSaveImageOpts,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    initialize_logger(opts.verbose.log_level_filter());

    println!("{:#?}", opts);

    Ok(())
}
