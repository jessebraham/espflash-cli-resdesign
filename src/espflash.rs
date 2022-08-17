use std::{num::ParseIntError, path::PathBuf};

use clap::Parser;
use clap_verbosity_flag::Verbosity;
use strum::VariantNames;

use crate::{
    BoardInfoOpts,
    ConnectOpts,
    FlashConfigOpts,
    ImageFormat,
    MonitorOpts,
    PartitionTableOpts,
};

#[derive(Debug, Parser)]
#[clap(propagate_version = true, version)]
pub struct Opts {
    #[clap(subcommand)]
    subcommand: Subcommand,
    #[clap(flatten)]
    pub verbose: Verbosity,
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
    flash_opts: crate::FlashOpts,
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
    save_image_opts: crate::SaveImageOpts,
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
