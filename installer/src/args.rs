use std::path::PathBuf;

use clap::Parser;

/// A Command Line Interface (CLI) for the Installing and updating of the DOS Disassembler
/// if the install path is not provided, the program will use the default installation path
/// if the uninstall flag is set, the program will uninstall the disassembler if it is installed in the default installation path
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// The path to the installation directory. If not provided, the program will use the default installation path.
    #[arg(short, long)]
    pub install_path: Option<PathBuf>,

    /// uninstall the program
    #[arg(long)]
    pub uninstall: bool,

    /// add to the system path
    #[arg(short, long, default_value_t = true)]
    pub add_to_path: bool,

    /// Create Shortcuts desktop and start menu shortcuts
    #[arg(short, long, default_value_t = true)]
    pub create_shortcuts: bool,
}
