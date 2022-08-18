use std::{num::ParseIntError, path::PathBuf};

use clap::Parser;
use espflash::{
    cli::{
        logging::initialize_logger,
        update::check_for_update,
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
use log::{debug, LevelFilter};
use strum::VariantNames;

#[derive(Debug, Parser)]
#[clap(propagate_version = true, version)]
pub struct Opts {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    BoardInfo(BoardInfoOpts),
    Flash(FlashOpts),
    Monitor(MonitorOpts),
    PartitionTable(PartitionTableOpts),
    SaveImage(SaveImageOpts),
    WriteBin(WriteBinOpts),
}

/// Flash an application to a target device
#[derive(Debug, Parser)]
pub struct FlashOpts {
    #[clap(flatten)]
    connect_opts: ConnectOpts,
    #[clap(flatten)]
    flash_opts: BaseFlashOpts,
}

#[derive(Debug, Parser)]
pub struct SaveImageOpts {
    /// Image format to flash
    #[clap(long, possible_values = ImageFormat::VARIANTS)]
    format: Option<String>,
    /// ELF image to flash
    image: PathBuf,

    #[clap(flatten)]
    pub flash_config_opts: FlashConfigOpts,
    #[clap(flatten)]
    save_image_opts: BaseSaveImageOpts,
}

/// Writes a binary file to a specific address in the chip's flash
#[derive(Debug, Parser)]
pub struct WriteBinOpts {
    /// Address at which to write the binary file
    #[clap(value_parser = parse_uint32)]
    pub addr: u32,
    /// File containing the binary data to write
    pub bin_file: String,

    #[clap(flatten)]
    connect_opts: ConnectOpts,
}

fn parse_uint32(input: &str) -> Result<u32, ParseIntError> {
    parse_int::parse(input)
}

fn main() {
    initialize_logger(LevelFilter::Info);
    check_for_update(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let opts = Opts::parse();
    debug!("{:#?}", opts);
}
