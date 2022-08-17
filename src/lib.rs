use std::{path::PathBuf, str::FromStr};

use clap::Parser;
use strum::VariantNames;
use strum_macros::{Display, EnumVariantNames};

pub mod cargo_espflash;
pub mod espflash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum Chip {
    Esp32,
    Esp32c2,
    Esp32c3,
    Esp32s2,
    Esp32s3,
    Esp8266,
}

impl FromStr for Chip {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().replace('-', "").as_str() {
            "esp32" => Ok(Chip::Esp32),
            "esp32c2" => Ok(Chip::Esp32c2),
            "esp32c3" => Ok(Chip::Esp32c3),
            "esp32s2" => Ok(Chip::Esp32s2),
            "esp32s3" => Ok(Chip::Esp32s3),
            "esp8266" => Ok(Chip::Esp8266),
            chip => Err(format!("Unrecognized chip name '{chip}'!")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumVariantNames)]
#[strum(serialize_all = "kebab-case")]
pub enum ImageFormat {
    IdfBootloader,
    DirectBoot,
}

impl FromStr for ImageFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "idf-bootloader" => Ok(ImageFormat::IdfBootloader),
            "direct-boot" => Ok(ImageFormat::DirectBoot),
            format => Err(format!("Unrecognized image format '{format}'!")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumVariantNames)]
pub enum FlashFrequency {}

impl FromStr for FlashFrequency {
    type Err = String;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumVariantNames)]
pub enum FlashMode {}

impl FromStr for FlashMode {
    type Err = String;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumVariantNames)]
pub enum FlashSize {}

impl FromStr for FlashSize {
    type Err = String;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Parser)]
pub struct ConnectOpts {
    /// Serial port connected to target device
    pub serial: Option<String>,
    /// Baud rate at which to communicate with target device
    #[clap(short, long)]
    pub speed: Option<u32>,
}

#[derive(Debug, Parser)]
pub struct FlashConfigOpts {
    /// Flash mode to use
    #[clap(long, possible_values = FlashMode::VARIANTS, value_name = "MODE")]
    pub flash_mode: Option<FlashMode>,
    /// Flash size of the target
    #[clap(long, possible_values = FlashSize::VARIANTS, value_name = "SIZE")]
    pub flash_size: Option<FlashSize>,
    /// Flash frequency
    #[clap(long, possible_values = FlashFrequency::VARIANTS, value_name = "FREQUENCY")]
    pub flash_freq: Option<FlashFrequency>,
}

/// Display information about the connected board and exit without flashing
#[derive(Debug, Parser)]
pub struct BoardInfoOpts {
    #[clap(flatten)]
    opts: ConnectOpts,
}

#[derive(Debug, Parser)]
pub struct FlashOpts {
    /// Path to a binary (.bin) bootloader file
    #[clap(long)]
    pub bootloader: Option<PathBuf>,
    /// Open a serial monitor after flashing
    #[clap(long)]
    pub monitor: bool,
    /// Path to a CSV file containing partition table
    #[clap(long)]
    pub partition_table: Option<PathBuf>,
    /// Load the application to RAM instead of Flash
    #[clap(long)]
    pub ram: bool,
}

/// Open the serial monitor without flashing
#[derive(Debug, Parser)]
pub struct MonitorOpts {
    #[clap(flatten)]
    opts: ConnectOpts,
}

/// Operations for partitions tables
#[derive(Debug, Parser)]
pub struct PartitionTableOpts {
    /// Convert CSV parition table to binary representation
    #[clap(long, conflicts_with = "to-csv")]
    to_binary: bool,
    /// Convert binary partition table to CSV representation
    #[clap(long, conflicts_with = "to-binary")]
    to_csv: bool,
    /// Input partition table
    partition_table: PathBuf,
    /// Optional output file name, if unset will output to stdout
    #[clap(short, long)]
    output: Option<PathBuf>,
}

/// Save the image to disk instead of flashing to device
#[derive(Debug, Parser)]
pub struct SaveImageOpts {
    /// Custom bootloader for merging
    #[clap(long)]
    pub bootloader: Option<PathBuf>,
    /// Chip to create an image for
    #[clap(long, possible_values = Chip::VARIANTS)]
    pub chip: Chip,
    /// File name to save the generated image to
    pub file: PathBuf,
    /// Boolean flag to merge binaries into single binary
    #[clap(long)]
    pub merge: bool,
    /// Custom partition table for merging
    #[clap(long)]
    pub partition_table: Option<PathBuf>,
}
