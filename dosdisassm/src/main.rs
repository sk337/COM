use clap::Parser;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

use disassembler::disassemble::{Disassembler, DisassemblerOptions};

/// Simple CLI for disassembling DOS .COM binaries
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path to the .COM binary file
    #[arg(short, long)]
    input: PathBuf,

    /// Optional output file
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Include labels
    #[arg(long, default_value_t = true)]
    labels: bool,

    /// Include instruction indenting after labels
    #[arg(long, default_value_t = true)]
    indent: bool,

    /// Include instruction address offsets
    #[arg(long, default_value_t = false)]
    offsets: bool,

    /// Annotate syscalls (int 21h)
    #[arg(long, default_value_t = true)]
    syscalls: bool,

    #[arg(long, default_value_t = false)]
    /// Include raw bytes in the output
    bytes: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if args.input.extension().map_or(true, |ext| ext != "com") {
        eprintln!(
            "Warn: Input file should have a .COM extension. this program will treat **ANY** file as a .COM file due to the nature of the DOS .COM file format not existing and being raw bytecode"
        );
    }

    let mut file = File::open(&args.input)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let disassembler = Disassembler::new(buffer);

    let opts = DisassemblerOptions {
        write_labels: args.labels,
        write_indent: args.indent,
        offset_comments: args.offsets,
        syscall_comments: args.syscalls,
        write_bytes: args.bytes,
    };

    match args.output {
        Some(path) => {
            let mut out_file = File::create(path)?;
            disassembler.disassemble_stream(&mut out_file, opts)?;
        }
        None => {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            disassembler.disassemble_stream(&mut handle, opts)?;
        }
    }

    Ok(())
}
