//! A simple utility to tokenize ASN files.

use std::io;

use clap::Parser;

use asn1_compiler::{
    generator::{Codec, Derive, Visibility},
    Asn1Compiler,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(last = true)]
    files: Vec<String>,

    /// Name of the Rust Module to write generated code to.
    #[arg(short, long)]
    module: String,

    #[arg(short)]
    debug: bool,

    /// Visibility of Generated Structures and members:
    #[arg(long, value_enum, default_value_t=Visibility::Public)]
    visibility: Visibility,

    /// ASN.1 Codecs to be Supported during code generation.
    /// Specify multiple times for multiple codecs. (eg. --codec aper --codec uper)
    #[arg(long, required = true)]
    codec: Vec<Codec>,

    /// Generate code for these derive macros during code generation.
    #[arg(long)]
    derive: Vec<Derive>,
}

fn main() -> io::Result<()> {
    let mut cli = Cli::parse();

    if cli.files.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "No Input files Specified",
        ));
    }

    if !cli.derive.contains(&Derive::Debug) {
        cli.derive.push(Derive::Debug);
    }

    let mut compiler = Asn1Compiler::new(
        &cli.module,
        cli.debug,
        &cli.visibility,
        cli.codec.clone(),
        cli.derive.clone(),
    );
    compiler.compile_files(&cli.files)?;

    Ok(())
}
