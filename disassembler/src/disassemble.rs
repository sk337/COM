use crate::consts::{Address, SIZE};
use crate::label::{Label, LabelList, LabelType};
use crate::syscall::{Syscall, SyscallList, SyscallType};
use iced_x86::{
    Decoder, DecoderOptions, Encoder, Formatter, Instruction, Mnemonic, NasmFormatter, OpKind,
    Register,
};
use std::collections::hash_map;
use std::fmt::Display;
use std::io::{self, Write};

#[derive(Debug)]
/// A wrapper type around Vec<Instruction> for implementing Display
pub struct InstructionList(pub Vec<Instruction>);

impl InstructionList {
    /// Creates a new InstructionList
    ///
    /// # Returns
    ///
    /// A new instance of `InstructionList` with an empty vector of instructions
    pub fn new() -> Self {
        InstructionList(Vec::new())
    }
}

impl Display for InstructionList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for instruction in self.0.iter() {
            write!(f, "{}\n", instruction)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
/// A struct for disassembling a binary file
pub struct Disassembler {
    /// A list of labels in the disassembled code
    pub labels: LabelList,
    /// A list of instructions in the disassembled code
    pub instructions: InstructionList,
    /// The raw binary bytecode data
    pub data: Vec<u8>,
    /// A list of syscalls in the disassembled code
    pub syscall_list: SyscallList,
    /// A hashmap to track register values
    pub register_tracker: hash_map::HashMap<Register, u16>,
}

/// Options for the disassembler
pub struct DisassemblerOptions {
    /// Whether to write labels
    pub write_labels: bool,
    /// Whether to write instructions
    pub write_indent: bool,
    /// Whether to write offsets
    pub offset_comments: bool,
    /// Whether to write syscall comments
    pub syscall_comments: bool,
    /// write bytes next to the instruction
    pub write_bytes: bool,
}

impl Disassembler {
    /// Creates a new disassembler from the given binary data
    ///
    /// # Arguments
    ///
    /// * `data` - A vector of bytes representing the binary data to disassemble
    ///
    /// # Returns
    ///
    /// A new instance of `Disassembler` with the provided data
    ///
    /// # Example
    ///
    /// ```
    /// use disassembler::Disassembler;
    ///
    /// let data = vec![0xB8, 0x04, 0x00, 0xCD, 0x21]; // Example binary data
    /// let disassembler = Disassembler::new(data);
    /// ```
    pub fn new(data: Vec<u8>) -> Self {
        let mut disassembler = Disassembler {
            labels: LabelList::new(),
            instructions: InstructionList::new(),
            data,
            syscall_list: SyscallList::new(),
            register_tracker: hash_map::HashMap::new(),
        };
        disassembler.disassemble();
        disassembler.search_labels();
        disassembler
    }

    fn disassemble(&mut self) {
        let mut decoder = Decoder::with_ip(SIZE, &self.data, 0x100, DecoderOptions::NONE);

        while decoder.can_decode() {
            let instruction = decoder.decode();
            // check if the Ah reg is being set
            if instruction.mnemonic() == Mnemonic::Mov {
                let regis = instruction.op0_register();
                if instruction.op1_kind() == OpKind::Immediate8 {
                    self.register_tracker
                        .insert(regis, instruction.immediate8() as u16);
                } else if instruction.op1_kind() == OpKind::Immediate16 {
                    self.register_tracker
                        .insert(regis, instruction.immediate16() as u16);
                } else if instruction.op1_kind() == OpKind::Register {
                    if let Some(value) = self.register_tracker.get(&instruction.op1_register()) {
                        self.register_tracker.insert(regis, *value);
                    } else {
                        self.register_tracker.insert(regis, 0);
                    }
                }
            }

            if instruction.mnemonic() == Mnemonic::Int {
                if instruction.op0_kind() == OpKind::Immediate8 {
                    if instruction.immediate8() == 0x21 {
                        let syscalltype = SyscallType::from_u16(
                            *self.register_tracker.get(&Register::AH).unwrap_or(&0),
                        );
                        if syscalltype.is_none() {
                            continue;
                        }
                        let syscalltype = syscalltype.unwrap();
                        let syscall = Syscall {
                            number: syscalltype,
                            address: instruction.ip() as Address,
                        };
                        self.syscall_list.0.push(syscall);
                    }
                }
            }

            self.instructions.0.push(instruction.clone());
        }
    }

    fn search_labels(&mut self) {
        for instruction in &self.instructions.0 {
            if instruction.is_jmp_short() {
                if instruction.ip() == 0x100 {
                    let label = Label {
                        address: instruction.near_branch_target() as Address,
                        label_type: LabelType::LABEL,
                        name: format!("START_0x{:04x}", instruction.near_branch_target()),
                    };
                    self.labels.0.push(label);
                } else {
                    let label = Label {
                        address: instruction.near_branch_target() as Address,
                        label_type: LabelType::LABEL,
                        name: format!("LABEL_0x{:04x}", instruction.near_branch_target()),
                    };
                    self.labels.0.push(label);
                }
            } else if instruction.is_call_near() {
                let label = Label {
                    address: instruction.near_branch_target() as Address,
                    label_type: LabelType::FUNCTION,
                    name: format!("FUNC_0x{:x}", instruction.near_branch_target()),
                };
                self.labels.0.push(label);
            }
        }
    }

    /// Disassembles the the code to a stream
    pub fn disassemble_stream<W: Write>(
        &self,
        f: &mut W,
        opts: DisassemblerOptions,
    ) -> io::Result<()> {
        let mut formatter = NasmFormatter::new();
        formatter.options_mut().set_digit_separator("'");
        formatter.options_mut().set_first_operand_char_index(12);
        formatter.options_mut().set_hex_prefix("0x");
        formatter.options_mut().set_hex_suffix("");
        formatter
            .options_mut()
            .set_number_base(iced_x86::NumberBase::Hexadecimal);

        let mut encoder = Encoder::new(SIZE);

        let mut indent = false;
        for instruction in &self.instructions.0 {
            let label = self.labels.get_by_address(instruction.ip() as Address);
            if let Some(label) = label {
                if opts.write_labels {
                    writeln!(f, "{label}")?;

                    indent = true;
                }
            }
            if indent && opts.write_indent {
                write!(f, "    ")?;
            }
            if instruction.mnemonic() == Mnemonic::Ret {
                indent = false;
            }

            // println!("{:?}", instruction.mnemonic());
            // if the instruction is a jump or call, check if it has a label
            if instruction.is_jmp_short() || instruction.is_call_near() {
                let address = self
                    .labels
                    .get_by_address(instruction.near_branch_target() as Address);

                if let Some(label) = address {
                    if instruction.is_jmp_short() {
                        write!(f, "jmp {} ; label", label.name)?;
                    } else {
                        write!(f, "call {} ; function", label.name)?;
                    }
                } else {
                    write!(f, "{}", instruction)?;
                }
            } else if (instruction.mnemonic() == Mnemonic::Int) && opts.syscall_comments {
                if instruction.op0_kind() == OpKind::Immediate8 {
                    if instruction.immediate8() == 0x21 {
                        let mut temp = String::new();
                        formatter.format(&instruction, &mut temp);
                        if opts.syscall_comments {
                            self.syscall_list
                                .get_by_address(instruction.ip() as Address)
                                .map(|syscall| write!(f, "{} ; {}", temp, syscall.number))
                                .unwrap_or_else(|| write!(f, "{}", temp))?;
                        } else {
                            write!(f, "{}", temp)?;
                        }
                    }
                } else {
                    let mut temp = String::new();
                    formatter.format(&instruction, &mut temp);
                    write!(f, "{}", temp)?;
                }
            } else {
                let mut temp = String::new();
                formatter.format(&instruction, &mut temp);
                write!(f, "{}", temp)?;
            }

            if opts.offset_comments {
                write!(f, " ; 0x{:04x}", instruction.ip())?;
            }

            if opts.write_bytes {
                write!(f, " ; bytes: ")?;
                let _ = encoder.encode(&instruction, 0x100);
                let bytes = encoder.take_buffer();
                for byte in bytes.iter() {
                    write!(f, "{:02x}", byte)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for Disassembler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut indent = false;
        for instruction in &self.instructions.0 {
            let label = self.labels.get_by_address(instruction.ip() as Address);
            if let Some(label) = label {
                writeln!(f, "{label}")?;
                indent = true;
            }
            if indent {
                write!(f, "    ")?;
            }

            if instruction.mnemonic() == Mnemonic::Ret {
                indent = false;
            }

            // if the instruction is a jump or call, check if it has a label
            if instruction.is_jmp_short() || instruction.is_call_near() {
                let address = self
                    .labels
                    .get_by_address(instruction.near_branch_target() as Address);

                if let Some(label) = address {
                    if instruction.is_jmp_short() {
                        write!(f, "jmp short {} ; label\n", label.name)?;
                    } else {
                        write!(f, "call {} ; function\n", label.name)?;
                    }
                } else {
                    write!(f, "{}\n", instruction)?;
                }
            } else {
                write!(f, "{}\n", instruction)?;
            }
        }
        Ok(())
    }
}
