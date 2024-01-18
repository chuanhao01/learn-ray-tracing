use std::{ffi::OsString, path::PathBuf};

use clap::{Parser, ValueEnum};
use image::{ImageFormat, RgbImage};

#[derive(Debug, Parser)]
#[command(name = "rsr")]
#[command(about = "Rust Simple RayTracer(RRT)", long_about=None)]
pub struct Cli {
    #[arg(default_value = "test")]
    name: OsString,
    #[arg(short, long, default_value_t=OutputFormat::PNG, value_enum)]
    output_format: OutputFormat,
}
impl Cli {
    pub fn save_image(&self, image: RgbImage) {
        let output_format = match self.output_format {
            OutputFormat::PNG => ImageFormat::Png,
            OutputFormat::PPM => ImageFormat::Pnm,
        };
        let output_ext = match self.output_format {
            OutputFormat::PNG => "png",
            OutputFormat::PPM => "ppm",
        };
        let mut path: PathBuf = PathBuf::from(self.name.clone());
        path.set_extension(output_ext);
        image.save_with_format(path, output_format).unwrap();
    }
}

#[derive(Debug, Clone, ValueEnum)]
#[allow(clippy::upper_case_acronyms)]
enum OutputFormat {
    PPM,
    PNG,
}

// rust-raytracer build scene --ppm(default png) path(default test.png in current dir with override, only do so at the end)
