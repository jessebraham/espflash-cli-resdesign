use std::path::PathBuf;

use clap::Args;
use strum::VariantNames;

use crate::enums::{Chip, FlashFrequency, FlashMode, FlashSize};

pub mod logging {
    pub use tracing_subscriber::filter::LevelFilter;

    pub fn initialize_logger(filter: LevelFilter) {
        tracing_subscriber::fmt()
            .with_target(false)
            .with_max_level(filter)
            .init();
    }
}

pub mod update {
    use std::time::Duration;

    use log::info;
    use update_informer::{registry, Check};

    pub fn check_for_update(name: &str, version: &str) {
        // By setting the interval to 0 seconds we invalidate the cache with each
        // invocation and ensure we're getting up-to-date results
        let informer =
            update_informer::new(registry::Crates, name, version).interval(Duration::from_secs(0));

        if let Some(version) = informer.check_version().ok().flatten() {
            info!("🚀 A new version of {name} is available: {version}");
        }
    }
}

#[derive(Debug, Args)]
pub struct ConnectArgs {
    /// Baud rate at which to communicate with target device
    #[clap(short = 'b', long)]
    pub baud: Option<u32>,
    /// Serial port connected to target device
    #[clap(short = 'p', long)]
    pub port: Option<String>,
}

#[derive(Debug, Args)]
pub struct FlashConfigArgs {
    /// Flash frequency
    #[clap(short = 'f', long, possible_values = FlashFrequency::VARIANTS, value_name = "FREQ")]
    pub flash_freq: Option<FlashFrequency>,
    /// Flash mode to use
    #[clap(short = 'm', long, possible_values = FlashMode::VARIANTS, value_name = "MODE")]
    pub flash_mode: Option<FlashMode>,
    /// Flash size of the target
    #[clap(short = 's', long, possible_values = FlashSize::VARIANTS, value_name = "SIZE")]
    pub flash_size: Option<FlashSize>,
}

/// Display information about the connected board and exit without flashing
#[derive(Debug, Args)]
pub struct BoardInfoArgs {
    #[clap(flatten)]
    args: ConnectArgs,
}

#[derive(Debug, Args)]
pub struct FlashArgs {
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
#[derive(Debug, Args)]
pub struct MonitorArgs {
    #[clap(flatten)]
    args: ConnectArgs,
}

/// Operations for partitions tables
#[derive(Debug, Args)]
pub struct PartitionTableArgs {
    /// Optional output file name, if unset will output to stdout
    #[clap(short = 'o', long)]
    output: Option<PathBuf>,
    /// Input partition table
    partition_table: PathBuf,
    /// Convert CSV parition table to binary representation
    #[clap(long, conflicts_with = "to-csv")]
    to_binary: bool,
    /// Convert binary partition table to CSV representation
    #[clap(long, conflicts_with = "to-binary")]
    to_csv: bool,
}

/// Save the image to disk instead of flashing to device
#[derive(Debug, Args)]
pub struct SaveImageArgs {
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
