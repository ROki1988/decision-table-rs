use anyhow::{anyhow, Result};
use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, value_t_or_exit,
    Arg,
};
use csv::{StringRecord, Writer};
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

    let ps: Vec<Rule> = serde_yaml::from_reader(std::fs::File::open(input)?)?;
    let mut wtr = Writer::from_writer(std::io::stdout());

    // write header
    for p in ps.iter() {
        wtr.write_field(p.name.as_str())?;
    }
    wtr.write_record(None::<&[u8]>)?;

    write_patterns(&ps, &mut StringRecord::new(), &mut wtr)
}

fn write_patterns<W: std::io::Write>(
    ps: &[Rule],
    buf: &mut StringRecord,
    wrt: &mut Writer<W>,
) -> Result<()> {
    match ps {
        [] => Err(anyhow!("Empty rules. Please check your input.")),
        [p] => {
            for x in p.pattern.iter() {
                let mut current = buf.clone();
                current.push_field(x);
                wrt.write_record(&current)?
            }
            Ok(())
        }
        [p, tail @ ..] => {
            for x in p.pattern.iter() {
                let mut current = buf.clone();
                current.push_field(x);
                write_patterns(tail, &mut current, wrt)?
            }
            Ok(())
        }
    }
}
