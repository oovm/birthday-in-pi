use crate::{Error, SupportedFormat};
use std::{fs, fs::File, io::Write};
use wolfram_wxf::{utils::parse_yaml, WolframValue};

pub fn parse_file(path: &str) -> Result<WolframValue, Error> {
    let s = fs::read_to_string(path)?;
    match parse_format(path) {
        SupportedFormat::Json => unimplemented!(),
        SupportedFormat::Toml => unimplemented!(),
        SupportedFormat::Yaml => Ok(parse_yaml(&s)),
        SupportedFormat::Pickle => unimplemented!(),
    }
}

fn parse_format(input: &str) -> SupportedFormat {
    //let suffix = input.split(".");
    SupportedFormat::Yaml
}

pub fn write_to_file(path: &str, bytes: &[u8]) -> Result<(), Error> {
    println!("Generating {}", path);
    let mut file = File::create(path)?;
    file.write_all(bytes)?;
    Ok(())
}


impl From<std::io::Error> for Error{
    fn from(_: std::io::Error) -> Self {
        Error::IO
    }
}