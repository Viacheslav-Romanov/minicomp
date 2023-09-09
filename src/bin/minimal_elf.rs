use serde::{Serialize, Deserialize};

// Virtual address where the file is going to be loaded into
pub const FILE_LOAD_VA: u64 = 4096 * 40;

#[derive(Serialize, Deserialize, Debug)]
pub struct ElfHeader {
    pub signature: [u8;4],
    pub class: u8,
    pub endianness: u8,
    pub elf_version: u8,
    pub os_abi: u8,
    pub extended_abi: u64,
    pub elf_file_type: u16,
    pub target_architecture: u16,
    pub additional_elf_version: u32,
    pub entry_point: u64,
    pub program_header_offset: u64,
    pub section_header_offset: u64,
    pub flags: u32,
    pub size_of_elf_header: u16,
    pub size_of_program_header_entry: u16,
    pub number_of_program_header_entries: u16,
    pub size_of_section_header_entry: u16,
    pub number_of_section_header_entries: u16,
    pub index_of_string_table: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProgramHeader {
    pub program_header_type: u32,
    pub program_header_flags: u32,
    pub loadable_segment_offset: u64,
    pub virtual_address: u64,
    pub physical_address: u64,
    pub segment_size_in_file: u64,
    pub segment_size_in_memory: u64,
    pub segment_aligment: u64,
}

#[derive(Serialize, Debug)]
pub struct SectionsHeader {
    pub null_section_header_1: [u8;32],
    pub null_section_header_2: [u8;32],
    pub offset_of_text: u32,
    pub loadable_bits: u32,
    pub flags: u64,
    pub virtual_address: u64,
    pub offset_in_file: u64,
    pub size_of_section: u64,
    pub linked_section_index: u32,
    pub info: u32,
    pub aligment: u64,
    pub entry_size: u64,
    pub string_table: u32,
    pub string_table_index: u32,
    pub loadable: u64,
    pub string_table_address: u64,
    pub string_table_offset: u64,
    pub string_table_size: u64,
    pub reserved1: u32,
    pub reserved2: u32,
    pub reserved3: u64,
    pub reserved4: u64,
}

#[derive(Serialize, Debug)]
pub struct SectionHeader {
    pub name: u32,
    pub bits: u32,
    pub flags: u64,
    pub addr: u64,
    pub offset: u64,
    pub size: u64,
    pub link: u32,
    pub info: u32,
    pub addralign: u64,
    pub entsize: u64,  
}

#[derive(Serialize, Debug)]
pub struct SymabEntry {
	pub	name: u32,
	pub info: u8,
	pub other: u8,
	pub shndx: u16,
	pub value: u64,
	pub size: u64,   
}

// #[derive(Serialize, Debug)]
// pub struct StringTable {
//     pub empty_string: u8,
//     pub text_section_name: [u8;6],
//     pub string_table_name: [u8;10],
// }

pub fn encode<T: serde::Serialize>(data: T) -> Vec<u8> {
    let encoded: Vec<u8> = bincode::serialize(&data).unwrap();
    encoded
}