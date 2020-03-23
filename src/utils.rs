use crate::{
    Error, SupportedFormat,
};
use std::{
    fs,
    fs::File,
    io::{ErrorKind, Write},
};
use wolfram_wxf::{utils::parse_yaml, WolframValue};

pub fn parse_file(path: &str) -> Result<WolframValue, Error> {
    let s = fs::read_to_string(path)?;
    let f = parse_format(path);
    println!("Parsing the file {} as {:?}", path, f);
    match parse_format(path) {
        SupportedFormat::JSON => unimplemented!(),
        SupportedFormat::TOML => unimplemented!(),
        SupportedFormat::YAML => parse_yaml(&s).map_err(|_| Error::ParseFailed),
        SupportedFormat::Pickle => unimplemented!(),
    }
}

fn parse_format(input: &str) -> SupportedFormat {
    let suffix: &str = input.split('/').last().unwrap().split('.').last().unwrap();
    match suffix {
        "yml" | "yaml" => SupportedFormat::YAML,
        "json" => SupportedFormat::JSON,
        _ => SupportedFormat::TOML,
    }
}

pub fn write_to_file(path: &str, bytes: &[u8]) -> Result<(), Error> {
    println!("Generating {}", path);
    let mut file = File::create(path)?;
    file.write_all(bytes)?;
    Ok(())
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        match e.kind() {
            ErrorKind::NotFound => Error::FileNotFound,
            ErrorKind::PermissionDenied => Error::PermissionDenied,
            _ => Error::UnknownIOError,
        }
    }
}
