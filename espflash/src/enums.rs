use std::str::FromStr;

use strum_macros::{Display, EnumVariantNames};

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
        use Chip::*;

        match s.to_lowercase().as_str() {
            "esp32" => Ok(Esp32),
            "esp32c2" => Ok(Esp32c2),
            "esp32c3" => Ok(Esp32c3),
            "esp32s2" => Ok(Esp32s2),
            "esp32s3" => Ok(Esp32s3),
            "esp8266" => Ok(Esp8266),
            chip => Err(format!("invalid chip name '{chip}'!")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumVariantNames)]
#[strum(serialize_all = "kebab-case")]
pub enum ImageFormat {
    IdfBoot,
    DirectBoot,
    McuBoot,
}

impl FromStr for ImageFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ImageFormat::*;

        match s.to_lowercase().as_str() {
            "idf-boot" => Ok(IdfBoot),
            "direct-boot" => Ok(DirectBoot),
            "mcu-boot" => Ok(McuBoot),
            format => Err(format!("invalid image format '{format}'!")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumVariantNames)]
pub enum FlashFrequency {
    #[strum(serialize = "12M")]
    Flash12M,
    #[strum(serialize = "15M")]
    Flash15M,
    #[strum(serialize = "16M")]
    Flash16M,
    #[strum(serialize = "20M")]
    Flash20M,
    #[strum(serialize = "24M")]
    Flash24M,
    #[strum(serialize = "26M")]
    Flash26M,
    #[strum(serialize = "30M")]
    Flash30M,
    #[strum(serialize = "40M")]
    Flash40M,
    #[strum(serialize = "48M")]
    Flash48M,
    #[strum(serialize = "60M")]
    Flash60M,
    #[strum(serialize = "80M")]
    Flash80M,
}

impl FromStr for FlashFrequency {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FlashFrequency::*;

        match s.to_uppercase().as_str() {
            "12M" => Ok(Flash12M),
            "15M" => Ok(Flash15M),
            "16M" => Ok(Flash16M),
            "20M" => Ok(Flash20M),
            "24M" => Ok(Flash24M),
            "26M" => Ok(Flash26M),
            "30M" => Ok(Flash30M),
            "40M" => Ok(Flash40M),
            "48M" => Ok(Flash48M),
            "60M" => Ok(Flash60M),
            "80M" => Ok(Flash80M),
            freq => Err(format!("invalid flash frequency '{freq}'!")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumVariantNames)]
#[strum(serialize_all = "UPPERCASE")]
pub enum FlashMode {
    Qio,
    Qout,
    Dio,
    Dout,
}

impl FromStr for FlashMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FlashMode::*;

        match s.to_uppercase().as_str() {
            "QIO" => Ok(Qio),
            "QOUT" => Ok(Qout),
            "DIO" => Ok(Dio),
            "DOUT" => Ok(Dout),
            mode => Err(format!("invalid flash mode '{mode}'!")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumVariantNames)]
pub enum FlashSize {
    #[strum(serialize = "256KB")]
    Flash256Kb = 0x12,
    #[strum(serialize = "512KB")]
    Flash512Kb = 0x13,
    #[strum(serialize = "1MB")]
    Flash1Mb   = 0x14,
    #[strum(serialize = "2MB")]
    Flash2Mb   = 0x15,
    #[strum(serialize = "4MB")]
    Flash4Mb   = 0x16,
    #[strum(serialize = "8MB")]
    Flash8Mb   = 0x17,
    #[strum(serialize = "16MB")]
    Flash16Mb  = 0x18,
    #[strum(serialize = "32MB")]
    Flash32Mb  = 0x19,
    #[strum(serialize = "64MB")]
    Flash64Mb  = 0x1a,
    #[strum(serialize = "128MB")]
    Flash128Mb = 0x21,
}

impl FromStr for FlashSize {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FlashSize::*;

        match s.to_uppercase().as_str() {
            "256KB" => Ok(Flash256Kb),
            "512KB" => Ok(Flash512Kb),
            "1MB" => Ok(Flash1Mb),
            "2MB" => Ok(Flash2Mb),
            "4MB" => Ok(Flash4Mb),
            "8MB" => Ok(Flash8Mb),
            "16MB" => Ok(Flash16Mb),
            "32MB" => Ok(Flash32Mb),
            "64MB" => Ok(Flash64Mb),
            "128MB" => Ok(Flash128Mb),
            size => Err(format!("invalid flash size '{size}'!")),
        }
    }
}
