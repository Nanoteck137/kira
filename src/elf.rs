//! Module to parse ELF files
#![allow(dead_code)]

#[derive(Debug)]
pub enum ElfError {
    /// Invalid ELF Header magic (0x00-0x03)
    InvalidMagic,

    /// Invalid ELF Header class (0x04)
    InvalidClass(u8),

    /// Invalid ELF Header data (0x05)
    InvalidData(u8),

    /// Failed to convert bytes to array for integer parsing
    TryFromSliceFailed(std::array::TryFromSliceError),

    /// Failed to cast 'string_table_index'
    StringTableIndexConvertionError,

    /// Failed to cast 'program_header_offset'
    ProgramHeaderOffsetConvertionError,

    /// Failed to cast 'program_header_entry_size'
    ProgramHeaderEntrySizeConvertionError,

    /// Failed to cast 'num_program_header_entries'
    NumProgramHeaderEntriesConvertionError,

    /// Failed to cast 'section_header_offset'
    SectionHeaderOffsetConvertionError,

    /// Failed to cast 'section_header_entry_size'
    SectionHeaderEntrySizeConvertionError,

    /// Failed to cast 'num_section_header_entries'
    NumSectionHeaderEntriesConvertionError,

    /// Invalid byte buffer size for ´ProgramHeader::parse´
    InvalidProgramHeaderBufferSize,
}

type Result<T> = std::result::Result<T, ElfError>;

#[derive(Debug)]
enum Class {
    Elf32,
    Elf64,
}

impl Class {
    fn parse(value: u8) -> Result<Self> {
        return match value {
            1 => Ok(Class::Elf32),
            2 => Ok(Class::Elf64),
            _ => Err(ElfError::InvalidClass(value))
        };
    }
}

#[derive(Debug)]
enum Data {
    LittleEndian,
    BigEndian
}

impl Data {
    fn parse(value: u8) -> Result<Self> {
        return match value {
            1 => Ok(Data::LittleEndian),
            2 => Ok(Data::BigEndian),
            _ => Err(ElfError::InvalidData(value))
        };
    }
}


#[derive(Debug)]
enum OsAbi {
    SystemV,

    Unknown(u8),
}

impl OsAbi {
    fn parse(value: u8) -> Self {
        return match value {
            0x00 => OsAbi::SystemV,
            _ => OsAbi::Unknown(value),
        };
    }
}

#[derive(Debug)]
enum Typ {
    /// Unknown
    None,

    /// Relocatable file
    Relocatable,

    /// Executable file
    Executable,

    /// Shared object
    Shared,

    /// Core file
    Core,

    /// Reserved inclusive range. Operating system specific
    OperatingSystem(u16),

    /// Reserved inclusive range. Processor specific
    Processor(u16),

    Unknown(u16),
}

impl Typ {
    fn parse(value: u16) -> Self {
        return match value {
            0x00 => Typ::None,
            0x01 => Typ::Relocatable,
            0x02 => Typ::Executable,
            0x03 => Typ::Shared,
            0x04 => Typ::Core,
            0xfe00..=0xfeff => Typ::OperatingSystem(value),
            0xff00..=0xffff => Typ::Processor(value),
            _ => Typ::Unknown(value),
        };
    }
}

#[derive(Debug)]
enum Machine {
    X86,
    Amd64,
    RiscV,

    Unknown(u16),
}

impl Machine {
    fn parse(value: u16) -> Self {
        return match value {
            0x03 => Machine::X86,
            0x3e => Machine::Amd64,
            0xf3 => Machine::RiscV,
            _ => Machine::Unknown(value),
        };
    }
}

#[derive(Debug)]
struct Header {
    offset: usize,
    entry_size: usize,
    num_entries: usize,
}

#[derive(Debug)]
enum ProgramHeaderTyp {
    /// Program header table entry unused
    Null,

    /// Loadable segment
    Load,

    /// Dynamic linking information
    Dynamic,

    /// Interpreter information
    Interp,

    /// Auxiliary information
    Note,

    /// Reserved
    Shlib,

    /// Segment containing program header table itself
    ProgramHeader,

    /// Thread-Local Storage template
    ThreadLocalStorage,

    /// Reserved inclusive range. Operating system specific
    OperatingSystem(u32),

    /// Reserved inclusive range. Processor specific
    Processor(u32),

    /// Unknown
    Unknown(u32),
}

impl ProgramHeaderTyp {
    fn parse(value: u32) -> Self {
        return match value {
            0x00000000 => Self::Null,
            0x00000001 => Self::Load,
            0x00000002 => Self::Dynamic,
            0x00000003 => Self::Interp,
            0x00000004 => Self::Note,
            0x00000005 => Self::Shlib,
            0x00000006 => Self::ProgramHeader,
            0x00000007 => Self::ThreadLocalStorage,

            0x60000000..=0x6FFFFFFF => Self::OperatingSystem(value),
            0x70000000..=0x7FFFFFFF => Self::Processor(value),

            _ => Self::Unknown(value),
        };
    }
}

#[derive(Debug)]
pub struct ProgramHeader {
    typ: ProgramHeaderTyp,
    flags: u32,

    offset: u64,

    vaddr: u64,
    paddr: u64,

    file_size: u64,
    memory_size: u64,

    alignment: u64,
}

impl ProgramHeader {
    fn parse(bytes: &[u8]) -> Result<ProgramHeader> {
        if bytes.len() < 56 {
            return Err(ElfError::InvalidProgramHeaderBufferSize);
        }

        let typ = u32::from_le_bytes(
            bytes[0..4].try_into()
                .map_err(|e| ElfError::TryFromSliceFailed(e))?);
        let typ = ProgramHeaderTyp::parse(typ);

        let flags = u32::from_le_bytes(
            bytes[4..8].try_into()
                .map_err(|e| ElfError::TryFromSliceFailed(e))?);

        let offset = u64::from_le_bytes(
            bytes[8..16].try_into()
                .map_err(|e| ElfError::TryFromSliceFailed(e))?);

        let vaddr = u64::from_le_bytes(
            bytes[16..24].try_into()
                .map_err(|e| ElfError::TryFromSliceFailed(e))?);

        let paddr = u64::from_le_bytes(
            bytes[24..32].try_into()
                .map_err(|e| ElfError::TryFromSliceFailed(e))?);

        let file_size = u64::from_le_bytes(
            bytes[32..40].try_into()
                .map_err(|e| ElfError::TryFromSliceFailed(e))?);

        let memory_size = u64::from_le_bytes(
            bytes[40..48].try_into()
                .map_err(|e| ElfError::TryFromSliceFailed(e))?);

        let alignment = u64::from_le_bytes(
            bytes[48..56].try_into()
                .map_err(|e| ElfError::TryFromSliceFailed(e))?);

        Ok(ProgramHeader {
            typ,
            flags,

            offset,

            vaddr,
            paddr,

            file_size,
            memory_size,

            alignment,
        })
    }
}

pub struct ProgramHeaderIter<'a> {
    elf: &'a Elf<'a>,
    current_index: usize,
}

impl<'a> Iterator for ProgramHeaderIter<'a> {
    type Item = ProgramHeader;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.elf.program_header.num_entries {
            return None;
        }

        let program_header = self.elf.program_header(self.current_index).ok()?;
        self.current_index += 1;

        Some(program_header)
    }
}

pub struct Elf<'a> {
    bytes: &'a [u8],
    class: Class,
    data: Data,
    os_abi: OsAbi,
    os_abi_version: u8,
    typ: Typ,
    machine: Machine,
    entry: u64,

    program_header: Header,
    section_header: Header,

    string_table_index: usize,
}

impl<'a> std::fmt::Debug for Elf<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) ->
        std::result::Result<(), std::fmt::Error>
    {
        f.debug_struct("Elf")
            .field("class", &self.class)
            .field("data", &self.data)
            .field("os_abi", &self.os_abi)
            .field("os_abi_version", &self.os_abi_version)
            .field("typ", &self.typ)
            .field("machine", &self.machine)
            .field("entry", &format_args!("{:#x}", self.entry))
            .field("program_header", &self.program_header)
            .field("section_header", &self.section_header)
            .field("string_table_index", &self.string_table_index)
            .finish()
    }
}

impl<'a> Elf<'a> {
    pub fn parse(bytes: &'a [u8]) -> Result<Self> {
        // TODO(patrik): Add length checks

        if &bytes[0..4] != b"\x7fELF" {
            return Err(ElfError::InvalidMagic);
        }

        let class = Class::parse(bytes[4])?;
        let data = Data::parse(bytes[5])?;

        // TODO(patrik): Should this be included inside the ´Elf´ struct
        let _version = bytes[6];

        let os_abi = OsAbi::parse(bytes[7]);
        let os_abi_version = bytes[8];

        // NOTE(patrik): Some padding inside the header
        let _pad = &bytes[9..16];

        let typ = u16::from_le_bytes(
            bytes[16..18].try_into()
                .map_err(|e| ElfError::TryFromSliceFailed(e))?);
        let typ = Typ::parse(typ);

        let machine = u16::from_le_bytes(
            bytes[18..20].try_into()
                .map_err(|e| ElfError::TryFromSliceFailed(e))?);
        let machine = Machine::parse(machine);

        // TODO(patrik): Should this be included inside the ´Elf´ struct
        let _version2 = u32::from_le_bytes(
            bytes[20..24].try_into()
                .map_err(|e| ElfError::TryFromSliceFailed(e))?);

        let entry = u64::from_le_bytes(
            bytes[24..32].try_into()
                .map_err(|e| ElfError::TryFromSliceFailed(e))?);

        let program_header_offset = u64::from_le_bytes(
            bytes[32..40].try_into()
                .map_err(|e| ElfError::TryFromSliceFailed(e))?);

        let section_header_offset = u64::from_le_bytes(
            bytes[40..48].try_into()
                .map_err(|e| ElfError::TryFromSliceFailed(e))?);

        // TODO(patrik): Should this be included inside the ´Elf´ struct
        let _flags = u32::from_le_bytes(
            bytes[48..52].try_into()
                .map_err(|e| ElfError::TryFromSliceFailed(e))?);

        // TODO(patrik): Should this be included inside the ´Elf´ struct
        let _header_size = u16::from_le_bytes(
            bytes[52..54].try_into()
                .map_err(|e| ElfError::TryFromSliceFailed(e))?);

        let program_header_entry_size = u16::from_le_bytes(
            bytes[54..56].try_into()
                .map_err(|e| ElfError::TryFromSliceFailed(e))?);

        let num_program_header_entries = u16::from_le_bytes(
            bytes[56..58].try_into()
                .map_err(|e| ElfError::TryFromSliceFailed(e))?);

        let section_header_entry_size = u16::from_le_bytes(
            bytes[58..60].try_into()
                .map_err(|e| ElfError::TryFromSliceFailed(e))?);

        let num_section_header_entries = u16::from_le_bytes(
            bytes[60..62].try_into()
                .map_err(|e| ElfError::TryFromSliceFailed(e))?);

        let string_table_index = u16::from_le_bytes(
            bytes[62..64].try_into()
                .map_err(|e| ElfError::TryFromSliceFailed(e))?);
        let string_table_index: usize = string_table_index.try_into()
            .map_err(|_| ElfError::StringTableIndexConvertionError)?;

        let program_header = Header {
            offset: program_header_offset.try_into()
                .map_err(|_| ElfError::ProgramHeaderOffsetConvertionError)?,
            entry_size: program_header_entry_size.try_into()
                .map_err(|_| ElfError::ProgramHeaderEntrySizeConvertionError)?,
            num_entries: num_program_header_entries.try_into()
                .map_err(|_| ElfError::NumProgramHeaderEntriesConvertionError)?,
        };

        let section_header = Header {
            offset: section_header_offset.try_into()
                .map_err(|_| ElfError::SectionHeaderOffsetConvertionError)?,
            entry_size: section_header_entry_size.try_into()
                .map_err(|_| ElfError::SectionHeaderEntrySizeConvertionError)?,
            num_entries: num_section_header_entries.try_into()
                .map_err(|_| ElfError::NumSectionHeaderEntriesConvertionError)?,
        };

        Ok(Self {
            bytes,

            class,
            data,
            os_abi,
            os_abi_version,
            typ,
            machine,
            entry,

            program_header,
            section_header,

            string_table_index,
        })
    }

    pub fn program_header(&self, index: usize) -> Result<ProgramHeader> {
        let start = self.program_header.offset +
            index * self.program_header.entry_size;
        let end = start + self.program_header.entry_size;
        let bytes = &self.bytes[start..end];

        ProgramHeader::parse(bytes)
    }

    pub fn program_header_iter(&self) -> ProgramHeaderIter {
        ProgramHeaderIter {
            elf: self,
            current_index: 0
        }
    }
}
