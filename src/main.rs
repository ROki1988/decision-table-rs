use anyhow::Result;
use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, value_t_or_exit,
    Arg,
};
use csv::{StringRecord, Writer};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Rule {
    name: String,
    pattern: Vec<String>,
}

fn main() -> Result<()> {
    let matches = app_from_crate!()
        .args(&[Arg::with_name("input")
            .help("set input yaml file")
            .takes_value(true)
            .value_name("FILE")
            .required(true)
            .short("i")
            .long("input")])
        .get_matches();

    let input: PathBuf = value_t_or_exit!(matches, "input", PathBuf);

    let rs: Vec<Rule> = serde_yaml::from_reader(std::fs::File::open(input)?)?;
    let mut wtr = Writer::from_writer(std::io::stdout());

    // write header
    for r in rs.iter() {
        wtr.write_field(r.name.as_str())?;
    }
    wtr.write_record(None::<&[u8]>)?;

    for r in rs
        .iter()
        .map(|x| x.pattern.iter())
        .multi_cartesian_product()
    {
        wtr.write_record(&StringRecord::from(r))?
    }

    Ok(())
}
