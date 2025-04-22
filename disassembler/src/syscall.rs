use std::fmt::Display;

use crate::consts::Address;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
/// An enum to represent the syscall numbers
pub enum SyscallType {
    /// Program terminate
    ProgramTerminate = 0x00,
    /// Character input
    CharacterInput = 0x01,
    /// Character output
    CharacterOutput = 0x02,
    /// Auxiliary input
    AuxiliaryInput = 0x03,
    /// Auxiliary output
    AuxiliaryOutput = 0x04,
    /// Printer output
    PrinterOutput = 0x05,
    /// Direct console I/O
    DirectConsoleIO = 0x06,
    /// Direct console input without echo
    DirectConsoleInputNoEcho = 0x07,
    /// Console input without echo
    ConsoleInputNoEcho = 0x08,
    /// Display string
    DisplayString = 0x09,
    /// Buffered keyboard input
    BufferedKeyboardInput = 0x0A,
    /// Get input status
    GetInputStatus = 0x0B,
    /// Flush input buffer and input
    FlushInputBuffer = 0x0C,
    /// Disk reset
    DiskReset = 0x0D,
    /// Set default drive
    SetDefaultDrive = 0x0E,
    /// Open file
    OpenFile = 0x0F,
    /// Close file
    CloseFile = 0x10,
    /// Find first file
    FindFirstFile = 0x11,
    /// Find next file
    FindNextFile = 0x12,
    /// Delete file
    DeleteFile = 0x13,
    /// Sequential read
    SequentialRead = 0x14,
    /// Sequential write
    SequentialWrite = 0x15,
    /// Create or truncate file
    CreateOrTruncateFile = 0x16,
    /// Rename file
    RenameFile = 0x17,
    /// Reserved
    Reserved18 = 0x18,
    /// Get default drive
    GetDefaultDrive = 0x19,
    /// Set disk transfer address
    SetDiskTransferAddress = 0x1A,
    /// Get allocation info for default drive
    GetAllocInfoDefault = 0x1B,
    /// Get allocation info for specified drive
    GetAllocInfoSpecified = 0x1C,
    /// Reserved
    Reserved1D = 0x1D,
    /// Reserved
    Reserved1E = 0x1E,
    /// Get disk parameter block for default drive
    GetDPBDefault = 0x1F,
    /// Reserved
    Reserved20 = 0x20,
    /// Random read
    RandomRead = 0x21,
    /// Random write
    RandomWrite = 0x22,
    /// Get file size in records
    GetFileSizeRecords = 0x23,
    /// Set random record number
    SetRandomRecordNumber = 0x24,
    /// Set interrupt vector
    SetInterruptVector = 0x25,
    /// Create PSP
    CreatePSP = 0x26,
    /// Random block read
    RandomBlockRead = 0x27,
    /// Random block write
    RandomBlockWrite = 0x28,
    /// Parse filename
    ParseFilename = 0x29,
    /// Get date
    GetDate = 0x2A,
    /// Set date
    SetDate = 0x2B,
    /// Get time
    GetTime = 0x2C,
    /// Set time
    SetTime = 0x2D,
    /// Set verify flag
    SetVerifyFlag = 0x2E,
    /// Get disk transfer address
    GetDiskTransferAddress = 0x2F,
    /// Get DOS version
    GetDosVersion = 0x30,
    /// Terminate and stay resident
    TerminateAndStayResident = 0x31,
    /// Get disk parameter block for specified drive
    GetDPBSpecified = 0x32,
    /// Get or set Ctrl-Break
    GetOrSetCtrlBreak = 0x33,
    /// Get InDOS flag pointer
    GetInDOSFlag = 0x34,
    /// Get interrupt vector
    GetInterruptVector = 0x35,
    /// Get free disk space
    GetFreeDiskSpace = 0x36,
    /// Get or set switch character
    GetOrSetSwitchChar = 0x37,
    /// Get or set country info
    GetOrSetCountryInfo = 0x38,
    /// Create subdirectory
    CreateSubdirectory = 0x39,
    /// Remove subdirectory
    RemoveSubdirectory = 0x3A,
    /// Change current directory
    ChangeCurrentDirectory = 0x3B,
    /// Create or truncate file
    CreateFile = 0x3C,
    /// Open file
    OpenFile2 = 0x3D,
    /// Close file
    CloseFile2 = 0x3E,
    /// Read file or device
    ReadFileOrDevice = 0x3F,
    /// Write file or device
    WriteFileOrDevice = 0x40,
    /// Delete file
    DeleteFile2 = 0x41,
    /// Move file pointer
    MoveFilePointer = 0x42,
    /// Get or set file attributes
    GetOrSetFileAttr = 0x43,
    /// I/O control for devices
    IOControl = 0x44,
    /// Duplicate handle
    DuplicateHandle = 0x45,
    /// Redirect handle
    RedirectHandle = 0x46,
    /// Get current directory
    GetCurrentDirectory = 0x47,
    /// Allocate memory
    AllocateMemory = 0x48,
    /// Release memory
    ReleaseMemory = 0x49,
    /// Reallocate memory
    ReallocateMemory = 0x4A,
    /// Execute program
    ExecuteProgram = 0x4B,
    /// Terminate with return code
    TerminateWithCode = 0x4C,
    /// Get program return code
    GetProgramReturnCode = 0x4D,
    /// Find first file
    FindFirstFile2 = 0x4E,
    /// Find next file
    FindNextFile2 = 0x4F,
    /// Set current PSP
    SetCurrentPSP = 0x50,
    /// Get current PSP
    GetCurrentPSP = 0x51,
    /// Get DOS internal pointers
    GetDosInternalPointers = 0x52,
    /// Create disk parameter block
    CreateDPB = 0x53,
    /// Get verify flag
    GetVerifyFlag = 0x54,
    /// Create program PSP
    CreateProgramPSP = 0x55,
    /// Rename file
    RenameFile2 = 0x56,
    /// Get or set file date and time
    GetOrSetFileDateTime = 0x57,
    /// Get or set allocation strategy
    GetOrSetAllocStrategy = 0x58,
    /// Get extended error info
    GetExtendedError = 0x59,
    /// Create unique file
    CreateUniqueFile = 0x5A,
    /// Create new file
    CreateNewFile = 0x5B,
    /// Lock or unlock file
    LockOrUnlockFile = 0x5C,
    /// File sharing functions
    FileSharingFunctions = 0x5D,
    /// Network functions
    NetworkFunctions = 0x5E,
    /// Network redirection functions
    NetworkRedirectionFunctions = 0x5F,
    /// Qualify filename
    QualifyFilename = 0x60,
    /// Reserved
    Reserved61 = 0x61,
    /// Get current PSP (alt)
    GetCurrentPSPAlt = 0x62,
    /// Get DBCS lead byte table pointer
    GetDBCSLeadByteTable = 0x63,
    /// Set wait for external event flag
    SetWaitForEvent = 0x64,
    /// Get extended country info
    GetExtendedCountryInfo = 0x65,
    /// Get or set code page
    GetOrSetCodePage = 0x66,
    /// Set handle count
    SetHandleCount = 0x67,
    /// Commit file
    CommitFile = 0x68,
    /// Get or set media id
    GetOrSetMediaID = 0x69,
    /// Commit file (alt)
    CommitFileAlt = 0x6A,
    /// Reserved
    Reserved6B = 0x6B,
    /// Extended open/create file
    ExtendedOpenCreateFile = 0x6C,
}

impl SyscallType {
    /// Returns the syscall number as a u16
    pub fn as_u16(&self) -> u16 {
        self.clone() as u16
    }

    /// parses a u16 into a syscall number
    pub fn from_u16(n: u16) -> Option<Self> {
        if n > 0x6C {
            return None;
        } else {
            return Some(unsafe { std::mem::transmute(n) });
        }
    }
}

impl Display for SyscallType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "{:?} 0x{:02x}", self, self.as_u16());
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A struct to represent a syscall
pub struct Syscall {
    /// The syscall number
    pub number: SyscallType,
    /// The address of the syscall
    pub address: Address,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A wrapper type around Vec<Syscall> for implementing Display
pub struct SyscallList(pub Vec<Syscall>);

impl SyscallList {
    /// Creates a new SyscallList
    ///
    /// # Returns
    ///
    /// A new instance of `SyscallList` with an empty vector of syscalls
    pub fn new() -> Self {
        SyscallList(Vec::new())
    }

    /// get a syscall by its address
    pub fn get_by_address(&self, address: Address) -> Option<&Syscall> {
        self.0.iter().find(|syscall| syscall.address == address)
    }
}
