use std::{fs::File, io::Read, io::stdout};
use disassembler::disassemble::{Disassembler, DisassemblerOptions};
fn main() {
    let filename = "../com/build/hello.com";
    let mut file = File::open(filename).expect("Unable to open file");
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).expect("Unable to read file");

    // Create a new disassembler instance
    let disassembler = Disassembler::new(buffer);
    
    // println!("Disassembled Instructions:\n{}", disassembler.instructions);
    // println!("Labels:\n{}", disassembler.labels);
    // println!("{}", disassembler);
    let _ = disassembler.disassemble_stream(&mut stdout(), DisassemblerOptions {
        write_labels: true,
        write_indent: true,
        syscall_comments: true,
        offset_comments: false,
    });

    // println!("{:?}", disassembler.syscall_list);
    // println!("{:?}", disassembler.register_tracker);

}