use std::path::PathBuf;
use crate::ResamplingFilter;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Flag(Flag),
    Command(Cmd),
    Path(PathBuf),
    Size(u32),
    Filter(ResamplingFilter)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Cmd {
    Ico,
    Icns,
    Favicon
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Flag {
    Entry,
    Help,
    Version,
    Resample,
    Output,
    AppleTouch,
    WebApp
}

impl<'a> From<&'a str> for Token {
    fn from(s: &str) -> Self {
        match s {
            "ico" => Token::Command(Cmd::Ico),
            "icns" => Token::Command(Cmd::Icns),
            "favicon" => Token::Command(Cmd::Favicon),
            "-e" | "--entry" => Token::Flag(Flag::Entry),
            "-r" | "--resample" => Token::Flag(Flag::Resample),
            "nearest" => Token::Filter(ResamplingFilter::Nearest),
            "linear" => Token::Filter(ResamplingFilter::Linear),
            "cubic" => Token::Filter(ResamplingFilter::Cubic),
            "-h" | "--help" => Token::Flag(Flag::Help),
            "-v" | "--version" => Token::Flag(Flag::Version),
            "-o" | "--output" => Token::Flag(Flag::Output),
            "--apple-touch" => Token::Flag(Flag::AppleTouch),
            "--web-app" => Token::Flag(Flag::WebApp),
            _ => {
                if let Ok(size) = s.parse::<u32>() {
                    Token::Size(size)
                } else {
                    Token::Path(PathBuf::from(s))
                }
            }
        }
    }
}
