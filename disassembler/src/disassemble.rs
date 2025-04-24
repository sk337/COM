use crate::comment::{Comment, CommentList, CommentType};
use crate::consts::{Address, COM_OFFSET, SIZE};
use crate::label::{Label, LabelList, LabelType};
use crate::string::{StringConstant, StringConstantList};
use crate::syscall::{Syscall, SyscallList, SyscallType};
use iced_x86::{
    Decoder, DecoderOptions, Encoder, Formatter, Instruction, Mnemonic, NasmFormatter, OpKind,
    Register,
};
use std::collections::hash_map;
use std::fmt::{self, Display};
use std::io::{self, Cursor, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
/// A struct for disassembling a binary file
///
/// This struct contains a list of labels, instructions, and other relevant data
/// for disassembling a binary file.
/// It provides methods for disassembling the binary data and formatting the output.
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
    /// a list of comments in the disassembled code
    pub comment_list: CommentList,
    /// A list of string constants in the disassembled code
    pub string_constant_list: StringConstantList,
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
    /// Whether to write misc comments
    pub misc_comments: bool,
}

impl Default for DisassemblerOptions {
    fn default() -> Self {
        DisassemblerOptions {
            write_labels: true,
            write_indent: true,
            offset_comments: false,
            syscall_comments: false,
            write_bytes: false,
            misc_comments: true,
        }
    }
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
    /// use disassembler::disassemble::Disassembler;
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
            comment_list: CommentList::new(),
            string_constant_list: StringConstantList::new(),
        };
        disassembler.disassemble();
        disassembler.search_labels();

        disassembler
    }

    fn find_string_constant(&mut self, address: Address) {
        let index = (address - Address(0x100));
        let mut out = String::new();
        for i in index..self.data.len() {
            if self.data[i] == 0x24 {
                out.push('$');
                break;
            } else if self.data[i] == 0x00 {
                break;
            }
            out.push(self.data[i] as char);
        }

        if out.len() > 0 {
            let string_constant = StringConstant {
                start: address,
                end: address + out.len() as u16,
                value: out,
            };
            self.string_constant_list.0.push(string_constant);
        }
    }

    fn create_syscall_comments(&mut self, syscall: &Syscall) {
        let s_type = syscall.number;
        if s_type == SyscallType::DisplayString {
            if let Some(address) = self.register_tracker.get(&Register::DX).copied() {
                self.find_string_constant(address);
                let comment = Comment {
                    comment_type: CommentType::PRE,
                    comment_text: "Start of string data".to_string(),
                    address,
                };
                self.comment_list.0.push(comment);
            }
        }
    }

    fn disassemble(&mut self) {
        let new_data = self.data.clone();
        let mut decoder = Decoder::with_ip(SIZE, &new_data, 0x100, DecoderOptions::NONE);

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
                        let sys_call_type = SyscallType::from_u16(
                            *self.register_tracker.get(&Register::AH).unwrap_or(&0),
                        );
                        if sys_call_type.is_none() {
                            continue;
                        }
                        let syscalltype = sys_call_type.unwrap();
                        let syscall = Syscall {
                            number: syscalltype,
                            address: instruction.ip() as Address,
                        };
                        self.create_syscall_comments(&syscall);
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
                        name: format!("_start"),
                    };
                    self.labels.0.push(label);

                    let comment = Comment {
                        comment_type: CommentType::PRE,
                        comment_text: "Start of program".to_string(),
                        address: instruction.near_branch_target() as Address,
                    };

                    self.comment_list.0.push(comment);
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
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to a writer implementing the `Write` trait
    /// * `opts` - A struct containing options for the disassembler
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure
    ///
    /// # Example
    ///
    /// ```
    /// use std::io::stdout;
    /// use disassembler::disassemble::{Disassembler, DisassemblerOptions};
    ///
    /// let data = vec![0xB8, 0x04, 0x00, 0xCD, 0x21]; // Example binary data
    /// let disassembler = Disassembler::new(data);
    /// disassembler.disassemble_stream(&mut stdout(), DisassemblerOptions::default());
    /// ```
    ///
    pub fn disassemble_stream<W: Write>(
        &self,
        f: &mut W,
        opts: DisassemblerOptions,
    ) -> io::Result<()> {
        let mut formatter = NasmFormatter::new();
        formatter.options_mut().set_digit_separator("'");
        formatter.options_mut().set_hex_prefix("0x");
        formatter.options_mut().set_hex_suffix("");
        formatter
            .options_mut()
            .set_number_base(iced_x86::NumberBase::Hexadecimal);

        let mut encoder = Encoder::new(SIZE);

        let mut indent = false;
        for instruction in &self.instructions.0 {
            let string_constant = self
                .string_constant_list
                .get_string_constant(instruction.ip() as Address);

            let label = self.labels.get_by_address(instruction.ip() as Address);
            let comments = self.comment_list.get_comments(instruction.ip() as Address);
            for comment in comments.clone() {
                if opts.misc_comments && comment.comment_type == CommentType::PRE {
                    if indent {
                        write!(f, "    ")?;
                    }
                    write!(f, "{}\n", comment)?;
                }
            }

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

            if let Some(string_constant) = string_constant {
                if instruction.ip() as Address == string_constant.start {
                    write!(f, "; {}\n", string_constant.as_db_statement())?
                }
            }

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

            for comment in comments.clone() {
                if opts.misc_comments && comment.comment_type == CommentType::INLINE {
                    write!(f, "{}", comment)?;
                }
            }

            writeln!(f)?;

            let has_post_comments = comments
                .iter()
                .any(|comment| comment.comment_type == CommentType::POST);
            for comment in comments.clone() {
                if opts.misc_comments && comment.comment_type == CommentType::POST {
                    if indent {
                        write!(f, "    ")?;
                    }
                    write!(f, "{}", comment)?;
                }
            }

            if has_post_comments {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Display for Disassembler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Pick whatever defaults you feel are “normal”.
        // You can also make these configurable through `Disassembler` fields.
        let opts = DisassemblerOptions::default();

        // Buffer the stream output in-memory…
        let mut buf = Cursor::new(Vec::<u8>::new());
        self.disassemble_stream(&mut buf, opts)
            .map_err(|_| fmt::Error)?;

        // …and then write it into the formatter.
        // SAFETY: `disassemble_stream` only writes valid UTF-8.
        let text = String::from_utf8(buf.into_inner()).map_err(|_| fmt::Error)?;
        f.write_str(&text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use std::io::Write;            // for Cursor
    // use std::io::Cursor;

    /// Helper: one tiny DOS‑COM program, starting at 0x100.
    ///
    /// Layout (addresses relative to COM load‑address 0x100):
    ///
    ///  ┌─────────────┐
    ///  │100 EB 04    │ jmp  START        (creates label)
    ///  │102 90 90 90 │ nop padding
    ///  │106 B4 09    │ START: mov ah, 09 (sets AH=09h)
    ///  │108 CD 21    │        int 21h    (syscall recognised)
    ///  │10A C3       │        ret
    ///  └─────────────┘
    fn sample_program() -> Vec<u8> {
        vec![
            0xEB, 0x04, // jmp short START (→0x106)
            0x90, 0x90, 0x90, 0x90, // padding NOPs
            0xB4, 0x09, // mov ah, 09h
            0xCD, 0x21, // int 21h
            0xC3, // ret
        ]
    }

    fn build_disassembler() -> Disassembler {
        Disassembler::new(sample_program())
    }

    // ──────────────────────────────────────────────────────────────────────────
    // 1.  InstructionList basics
    // ──────────────────────────────────────────────────────────────────────────
    #[test]
    fn instruction_list_is_empty_on_new() {
        let list = InstructionList::new();
        assert!(list.0.is_empty(), "new() should start with an empty vec");
        assert_eq!(format!("{list}"), "");
    }

    // ──────────────────────────────────────────────────────────────────────────
    // 2.  Register tracking + syscall detection
    // ──────────────────────────────────────────────────────────────────────────
    #[test]
    fn disassembler_tracks_ah_and_syscall() {
        let d = build_disassembler();

        // AH should contain 0x09 after the MOV
        assert_eq!(
            d.register_tracker.get(&Register::AH).copied(),
            Some(0x09),
            "AH register must be detected as 0x09"
        );

        // Exactly one DOS interrupt 21h should be recognised
        assert_eq!(d.syscall_list.0.len(), 1, "INT 21h syscall not detected");
        assert_eq!(
            d.syscall_list.0[0].address, // where the syscall lives
            0x108,
            "Syscall address should match INT 21h offset"
        );
    }

    // ──────────────────────────────────────────────────────────────────────────
    // 3.  Jump / function‑label discovery
    // ──────────────────────────────────────────────────────────────────────────
    #[test]
    fn jump_creates_start_label() {
        let d = build_disassembler();

        let lbl = d
            .labels
            .get_by_address(0x0106)
            .expect("Label for 0x0106 must exist");
        assert_eq!(lbl.name, "_start");
        assert_eq!(lbl.label_type, LabelType::LABEL);
    }

    // ──────────────────────────────────────────────────────────────────────────
    // 4.  Stream formatting – smoke‑test every option
    // ──────────────────────────────────────────────────────────────────────────
    #[test]
    fn disassemble_stream_emits_expected_text() {
        let d = build_disassembler();
        let opts = DisassemblerOptions {
            write_labels: true,
            write_indent: true,
            offset_comments: true,
            syscall_comments: true,
            write_bytes: true,
            misc_comments: true,
        };

        let mut buf = Vec::<u8>::new();
        d.disassemble_stream(&mut buf, opts)
            .expect("stream display should succeed");

        let out = String::from_utf8(buf).expect("output is valid UTF-8");

        // Essential sign‑posts
        assert!(out.contains("_start"), "Label should be printed");
        assert!(
            out.contains("jmp _start ; label"),
            "Jump should be rewritten to symbolic label"
        );
        assert!(
            out.contains("int 0x21"),
            "INT 21h should appear in NASM formatter output"
        );
        assert!(out.contains("; 0x0100"), "Offset comments must be present");
        assert!(
            out.contains("; bytes:"),
            "Raw-bytes comment should be present"
        );
        // There should be *some* syscall comment appended after int 21h
        assert!(
            out.lines()
                .any(|l| l.contains("int 0x21") && l.contains(" ; ")),
            "INT 21h line should contain a semicolon-separated syscall name/value"
        );
    }
}
