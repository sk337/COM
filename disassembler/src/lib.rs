#![deny(missing_docs)]
//! Disassembler Designed for COM files that outputs labeled assembly code in NASM syntax

/// a Module that contains Constants for the disassembler
pub mod consts;
/// a Module that dissasmbles the binary code
pub mod disassemble;
/// a Module that contains the label struct
pub mod label;
/// a Module that contains int 21h syscalls
pub mod syscall;
/// a Module for managing comments in the disassembly
pub mod comment;
/// a Module for defining string constants
pub mod string;