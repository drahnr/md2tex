use fs::File;
use fs_err as fs;
use std::io::{Read, Write};

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use cmark2tex::Error;

use cmark2tex::markdown_to_tex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new(crate_name!())
        .bin_name(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .arg(
            Arg::with_name("INPUT")
                .long("input")
                .short("i")
                .help("Input markdown files")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .long("output")
                .short("o")
                .help("Output tex or pdf file")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let mut content = String::new();
    let mut input = File::open(matches.value_of("INPUT").ok_or(Error::MissingArg)?)?;

    input.read_to_string(&mut content)?;

    let output_path = matches.value_of("OUTPUT").ok_or(Error::MissingArg)?;
    let mut output = File::create(output_path)?;

    let tex = markdown_to_tex(content)?;
    output.write(tex.as_bytes())?;
    Ok(())
}
