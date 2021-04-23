extern crate csv;
pub mod errors;
use crate::errors::Error;
use serde::{Deserialize, Serialize};
extern crate serde_json;
// use serde_json::Value;
use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::process;

#[derive(Debug)]
pub struct Config {
    pub input_file: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, Error> {
        args.next(); // skip program name

        let input_file = match args.next() {
            Some(a) => a,
            None => return Err(Error::from("No input file specified.")),
        };

        Ok(Config { input_file })
    }
}

fn exists(pth: &Path) -> bool {
    fs::metadata(pth).is_ok()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogMessage {
    level: String,
    message: String,
    source: String,
    timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Log {
    timestamp: u64,
    message: LogMessage,
    logStream: String,
    logGroup: String,
}

type SumoRecord = (String, String, String);

pub fn parse_logs(cfg: &Config) -> Result<(), Error> {
    let fname = Path::new(&cfg.input_file);
    if !exists(&fname) {
        return Err(Error::General("Input file does not exist.".to_string()));
    }
    let fstream = File::open(fname)?;
    let mut rdr = csv::Reader::from_reader(fstream);

    // let headers = rdr.headers()?;
    // println!("Headers: {:?}", headers);

    for result in rdr.deserialize() {
        let record: SumoRecord = result?;
        let (_, _, raw) = record; // NOTE: other metadata in the log available here
        let raw: Log = serde_json::from_str(&raw)?;
        do_with_log(&raw);
    }

    Ok(())
}

fn main() {
    let cfg = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing command-line arguments: {}", err);
        process::exit(1);
    });

    parse_logs(&cfg).unwrap_or_else(|err| {
        eprintln!("Problem parsing logs: {}", err);
        process::exit(1);
    });
}

pub fn do_with_log(log: &Log) {
    // TODO: add your code here
    println!("{:?}", log);
}
