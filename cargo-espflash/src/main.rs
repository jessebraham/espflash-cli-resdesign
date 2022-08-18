use clap::{Args, Parser, Subcommand};
use espflash::{
    cli::{
        logging::{initialize_logger, LevelFilter},
        update::check_for_update,
        BoardInfoArgs,
        ConnectArgs,
        FlashArgs as BaseFlashArgs,
        FlashConfigArgs,
        MonitorArgs,
        PartitionTableArgs,
        SaveImageArgs as BaseSaveImageArgs,
    },
    enums::ImageFormat,
};
use log::debug;
use strum::VariantNames;

#[derive(Debug, Parser)]
#[clap(bin_name = "cargo", propagate_version = true, version)]
struct Cli {
    #[clap(subcommand)]
    subcommand: CargoSubcommand,
}

#[derive(Debug, Subcommand)]
enum CargoSubcommand {
    Espflash {
        #[clap(subcommand)]
        subcommand: Commands,
    },
}

#[derive(Debug, Subcommand)]
enum Commands {
    BoardInfo(BoardInfoArgs),
    Flash(FlashArgs),
    Monitor(MonitorArgs),
    PartitionTable(PartitionTableArgs),
    SaveImage(SaveImageArgs),
}

#[derive(Debug, Args)]
struct BuildArgs {
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
    pub flash_config_args: FlashConfigArgs,
}

/// Build and flash an application to a target device
#[derive(Debug, Args)]
struct FlashArgs {
    #[clap(flatten)]
    build_args: BuildArgs,
    #[clap(flatten)]
    connect_args: ConnectArgs,
    #[clap(flatten)]
    flash_args: BaseFlashArgs,
}

#[derive(Debug, Args)]
struct SaveImageArgs {
    #[clap(flatten)]
    build_args: BuildArgs,
    #[clap(flatten)]
    save_image_args: BaseSaveImageArgs,
}

fn main() {
    initialize_logger(LevelFilter::INFO);

    // Attempt to parse any provided comand-line arguments, or print the help
    // message and terminate if the invocation is not correct.
    let CargoSubcommand::Espflash { subcommand: args } = Cli::parse().subcommand;
    debug!("{:#?}", args);

    // Only check for updates once the command-line arguments have been processed,
    // to avoid printing any update notifications when the help message is
    // displayed.
    check_for_update(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}
