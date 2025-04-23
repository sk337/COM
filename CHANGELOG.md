# Change Log

## v0.1.0
- Initial release of the `dosdisassm` project.
- Added a CLI tool for disassembling DOS `.COM` binaries.
- Implemented support for labels, instruction indenting, and syscall annotations.
- Added the ability to include raw bytes and offsets in the disassembly output.
- Provided a library (`disassembler`) for handling disassembly logic, including:
  - Instruction parsing and formatting.
  - Label and syscall detection.
  - Register tracking.
- Included tests for core functionality in both the CLI and library.
- Added build and release pipelines for Linux, Windows, and macOS.