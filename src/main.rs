use fs_err as fs;
use std::io::{Read, Write};
use std::path::PathBuf;

use clap::Parser;
use cmark2tex::markdown_to_tex;

#[derive(clap::Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, verbatim_doc_comment, env)]
    /// Input cmark/markdown file
    input: PathBuf,

    #[arg(short, long, verbatim_doc_comment, env)]
    /// Output file, will be overwritten
    output: PathBuf,
}

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::try_parse()?;

    let mut content = String::new();
    let mut input = fs::File::open(&args.input)?;

    input.read_to_string(&mut content)?;

    let mut output = fs::File::create(&args.output)?;

    let tex = markdown_to_tex(content)?;
    output.write(tex.as_bytes())?;

    Ok(())
}
