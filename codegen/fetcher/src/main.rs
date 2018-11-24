extern crate curl;
extern crate pest;
extern crate scraper;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate structopt;
#[macro_use]
extern crate tera;

mod fetcher;
mod generator;
mod format;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Fetch {
    #[structopt(short = "u", long = "url")]
    url: Option<String>,
    #[structopt(name = "OUTPUT", parse(from_os_str))]
    output: PathBuf,
}

#[derive(Debug, StructOpt)]
pub struct Generate {
    #[structopt(name = "OPLIST", parse(from_os_str))]
    oplist: PathBuf,
    #[structopt(name = "TEMPLATE", parse(from_os_str))]
    template: PathBuf,
    #[structopt(name = "OUTPUT", parse(from_os_str))]
    output: PathBuf,
}

#[derive(Debug, StructOpt)]
pub enum Opt {
    #[structopt(name = "fetch")]
    Fetch(Fetch),
    #[structopt(name = "generate")]
    Generate(Generate),
}

#[derive(Debug)]
pub struct Error(String);

pub type Result<T> = std::result::Result<T, Error>;

impl From<String> for Error {
    fn from(s: String) -> Error {
        Error(s)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(s: std::num::ParseIntError) -> Error {
        Error(s.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(s: std::io::Error) -> Error {
        Error(s.to_string())
    }
}

impl std::convert::From<tera::Error> for Error {
    fn from(s: tera::Error) -> Error {
        Error(s.to_string())
    }
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    env_logger::init();

    match opt {
        Opt::Fetch(opt) => fetcher::run(&opt),
        Opt::Generate(opt) => generator::run(&opt),
    }
}
