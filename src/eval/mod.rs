use std::{io::{self, stdout}, fs, path::PathBuf, collections::HashMap};
use crate::{error::Error, Output, IconType, ResamplingFilter, Entries};
use icon_baker::{resample, Icon, ico::Ico, icns::Icns, favicon::Favicon, SourceImage};

macro_rules! resample {
    ($r:expr) => {
        match $r {
            ResamplingFilter::Nearest => resample::nearest,
            ResamplingFilter::Linear  => resample::linear,
            ResamplingFilter::Cubic   => resample::cubic
        }
    };
}

macro_rules! io {
    ($err:expr) => {
        Error::Io($err, Output::Stdout)
    };
    ($err:expr, $path:expr) => {
        Error::Io($err, Output::Path($path))
    };
}

macro_rules! not_found {
    () => {
        io::Error::from(io::ErrorKind::NotFound)
    };
}

pub fn icon(entries: &Entries, icon_type: IconType, output: &Output) -> Result<(), Error> {
    match icon_type {
        IconType::Ico     => write(&mut get_icon::<Ico >(entries)?,    output),
        IconType::Icns    => write(&mut get_icon::<Icns>(entries)?,    output),
        IconType::Favicon => write(&mut get_icon::<Favicon>(entries)?, output)
    }
}

fn get_icon<I: Icon>(entries: &Entries) -> Result<I, Error> {
    let mut icon = I::new();
    let source_map = get_source_map(entries)?;

    for (size, path, filter) in entries {
        let src = source_map.get(path)
            .expect("Variable 'source_map' should have a key for String 'path'");

        match icon.add_entry(resample!(filter), src, *size) {
            Ok(()) => continue,
            Err(icon_baker::Error::Io(err)) => return Err(io!(err, path.clone())),
            Err(err) => return Err(Error::IconBaker(err))
        }
    }

    Ok(icon)
}

fn get_source_map(
    entries: &Entries
) -> Result<HashMap<&PathBuf, SourceImage>, Error> {
    let mut source_map = HashMap::with_capacity(entries.len());

    for (_, path, _) in entries {
        if let None = source_map.get(path) {
            if let Some(source) = SourceImage::from_path(path) {
                source_map.insert(path, source);
            } else {
                return Err(io!(not_found!(), path.clone()));
            }
        }
    }

    Ok(source_map)
}

fn write<I: Icon>(icon: &mut I, output: &Output) -> Result<(), Error> {
    match output {
        Output::Path(path) => match fs::File::create(path.clone()) {
            Ok(mut file) => icon.write(&mut file)
                .map_err(|err| io!(err, path.clone())),

            Err(err) => Err(io!(err, path.clone()))
        },
        Output::Stdout => icon.write(&mut stdout())
            .map_err(|err| io!(err))
    }
}