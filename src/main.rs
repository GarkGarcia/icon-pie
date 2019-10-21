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
