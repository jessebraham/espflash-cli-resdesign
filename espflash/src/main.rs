use std::{num::ParseIntError, path::PathBuf};

use clap::{Args, Parser, Subcommand};
use espflash::{
    cli::{
        logging::initialize_logger,
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
use log::{debug, LevelFilter};
use strum::VariantNames;

#[derive(Debug, Parser)]
#[clap(propagate_version = true, version)]
struct Cli {
    #[clap(subcommand)]
    subcommand: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    BoardInfo(BoardInfoArgs),
    Flash(FlashArgs),
    Monitor(MonitorArgs),
    PartitionTable(PartitionTableArgs),
    SaveImage(SaveImageArgs),
    WriteBin(WriteBinArgs),
}

/// Flash an application to a target device
#[derive(Debug, Args)]
struct FlashArgs {
    #[clap(flatten)]
    connect_args: ConnectArgs,
    #[clap(flatten)]
    flash_args: BaseFlashArgs,
}

#[derive(Debug, Args)]
struct SaveImageArgs {
    /// Image format to flash
    #[clap(long, possible_values = ImageFormat::VARIANTS)]
    format: Option<String>,
    /// ELF image to flash
    image: PathBuf,

    #[clap(flatten)]
    pub flash_config_args: FlashConfigArgs,
    #[clap(flatten)]
    save_image_args: BaseSaveImageArgs,
}

/// Writes a binary file to a specific address in the chip's flash
#[derive(Debug, Args)]
struct WriteBinArgs {
    /// Address at which to write the binary file
    #[clap(value_parser = parse_uint32)]
    pub addr: u32,
    /// File containing the binary data to write
    pub bin_file: String,

    #[clap(flatten)]
    connect_args: ConnectArgs,
}

fn parse_uint32(input: &str) -> Result<u32, ParseIntError> {
    parse_int::parse(input)
}

fn main() {
    initialize_logger(LevelFilter::Debug);

    // Attempt to parse any provided comand-line arguments, or print the help
    // message and terminate if the invocation is not correct.
    let args = Cli::parse().subcommand;
    debug!("{:#?}", args);

    // Only check for updates once the command-line arguments have been processed,
    // to avoid printing any update notifications when the help message is
    // displayed.
    check_for_update(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}
