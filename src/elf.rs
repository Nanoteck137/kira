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
pub struct Elf {
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

impl Elf {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
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

    pub fn section(index: usize) {
    }
}
