extern crate icon_baker;
extern crate crossterm;

mod parse;
mod error;
mod command;

use std::{env, io, path::{PathBuf}};
use icon_baker::{resample, image::DynamicImage, SourceImage};

#[derive(Clone, Debug)]
pub enum Output {
    Path(PathBuf),
    Stdout
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ResamplingFilter {
    Nearest,
    Linear,
    Cubic
}

pub type Entries<K> = Vec<(K, PathBuf, ResamplingFilter)>;

#[macro_export]
macro_rules! syntax {
    ($err:expr) => { Err(Error::Syntax($err)) };
}

const VERSION: &str = "0.1.4-beta";
const TITLE: &str = r"
 _____               ______ _      
|_   _|              | ___ (_)     
  | |  ___ ___  _ __ | |_/ /_  ___ 
  | | / __/ _ \| '_ \|  __/| |/ _ \
 _| || (_| (_) | | | | |   | |  __/
 \___/\___\___/|_| |_\_|   |_|\___|";
const USAGE: &str = "$ icon-pie ((-e <file path> <size>... [-r (nearest | linear | cubic)])... (-ico | -icns | -png) [<output path>]) | -h | --help | -v | --version";

const COMMANDS: [&str;7] = [
    "Specify an entry's source image, target sizes and resampling filter (optional).",
    "Specify a re-sampling filter: `nearest`, `linear` or `cubic`. If no filter is specified the app defaults to `nearest`.",
    "Outputs to an `.ico` file. If no output path is specified the app outputs to `stdout`.",
    "Outputs to an `.icns` file. If no output path is specified the app outputs to `stdout`.",
    "Outputs a `.png` sequence as a `.tar` file. If no output path is specified the app outputs to `stdout`.",
    "Help.",
    "Display version information.",
];

const EXAMPLES: [&str;3] = [
    "$ icon-pie -e big.svg 32 64 128 -ico icon.ico",
    "$ icon-pie -e small.png 32 64 -e big.svg 128 -icns icon.icns",
    "$ icon-pie -e small.png 32 64 -r linear -e big.svg 128 -png icon.tar"
];

impl ResamplingFilter {
    pub fn call(&self, source: &SourceImage, size: u32) -> io::Result<DynamicImage> {
        match self {
            ResamplingFilter::Nearest => resample::nearest(source, size),
            ResamplingFilter::Linear  => resample::linear(source, size),
            ResamplingFilter::Cubic   => resample::cubic(source, size)
        }
    }
}

fn main() -> io::Result<()> {
    let cmd = parse::args()
        .map_err(exit)?;
    
    cmd.eval().map_err(exit)
}

#[inline]
fn exit(err: error::Error) -> io::Error {
   eprintln!("{}", err);
   err.into() 
}

fn args() -> Vec<String> {
    let output: Vec<String> = env::args_os()
        .map(|os_str| String::from(os_str.to_string_lossy()))
        .collect();

    Vec::from(&output[1..])
}
