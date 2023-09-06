use crate::formula_parser::*;
use crate::minimal_elf::*;

#[derive(Debug)]
pub struct Equation {
    function_name: String,
    tree: ParseNode,
    number_of_arguments: usize,
}

pub fn generate_code_section(equations: &Vec<Equation>) -> Vec<u8> {
    let res = equation_to_code(equations.get(0).expect("Supposed one to be returned"));
    res
}

pub fn assemble_binary(equations: &Vec<Equation>) -> Vec<u8> {
    let mut assembly = Vec::new();

    assembly.append(&mut assemble_elf_header(0x138));
    assembly.append(&mut assemble_program_header(0x179));
    assembly.append(&mut assemble_section_header(0x188, 0x179));
    assembly.append(&mut generate_code_section(&equations));

    let entry_point_offset = assembly.len() as u64;

    assembly.append(&mut entry_point_code(0x281a0));
    let message_buffer_offset = assembly.len() as u64;
    assembly.append(&mut message_buffer());

    let string_table_offset = assembly.len() as u64;

    assembly.append(&mut assemble_string_table());

    let file_size = assembly.len() as u64;

    assembly.clear();

    // need a second pass here to update the binary with calculated values
    assembly.append(&mut assemble_elf_header(entry_point_offset));
    assembly.append(&mut assemble_program_header(string_table_offset));
    assembly.append(&mut assemble_section_header(file_size, string_table_offset));
    assembly.append(&mut generate_code_section(&equations));
    assembly.append(&mut entry_point_code(FILE_LOAD_VA + message_buffer_offset));
    assembly.append(&mut message_buffer());
    assembly.append(&mut assemble_string_table());
    assembly
}

pub fn assemble_elf_header(entry_point_offset: u64) -> Vec<u8> {
    let elf = ElfHeader {
        signature: *b"\x7fELF",
        class: 2,
        endianness: 1,
        elf_version: 1,
        os_abi: 0,
        extended_abi: 0,
        elf_file_type: 2,
        target_architecture: 0x3e,
        additional_elf_version: 1,        
        entry_point: FILE_LOAD_VA + entry_point_offset, // calculate entry_point + file_load_va
        program_header_offset: 0x40, // calculate program_headers_start
        section_header_offset: 0x78, // calculate section_headers_start
        flags: 0,
        size_of_elf_header: 64,
        size_of_program_header_entry: 0x38,
        number_of_program_header_entries: 1,
        size_of_section_header_entry: 0x40,
        number_of_section_header_entries: 3,
        index_of_string_table: 2,    
    };

    let res = encode(&elf);
    res
}

pub fn assemble_program_header(segment_size: u64) -> Vec<u8> {
    let ph = ProgramHeader {
        program_header_type: 1,
        program_header_flags: 7,
        loadable_segment_offset: 0,
        virtual_address: FILE_LOAD_VA, //calculate
        physical_address: FILE_LOAD_VA, //calculate
        segment_size_in_file: segment_size, //calculate?
        segment_size_in_memory: segment_size, //calculate?
        segment_aligment: 0x200000,
    };

    let res = encode(&ph);
    res
}

pub fn assemble_section_header(file_size: u64, string_table_offset: u64) -> Vec<u8> {
    let sh = SectionsHeader {
        null_section_header_1: "\x00".repeat(32).as_bytes().try_into().unwrap(),
        null_section_header_2: "\x00".repeat(32).as_bytes().try_into().unwrap(),
        offset_of_text: 1, //calculate text_section_name - string_table
        loadable_bits: 1,
        flags: 7,
        virtual_address: FILE_LOAD_VA, //calculate file_load_va
        offset_in_file: 0,
        size_of_section: file_size, //calculate file_end
        linked_section_index: 0,
        info: 0,
        aligment: 16,
        entry_size: 0,
        string_table: 7, //calculate string_table_name - string_table
        string_table_index: 3,
        loadable: 0,
        string_table_address: FILE_LOAD_VA + string_table_offset, //calculate file_load_va + string_table
        string_table_offset: string_table_offset, //calculate string_table
        string_table_size: 0x11, //calculate string_table_end - string_table
        reserved1: 0,
        reserved2: 0,
        reserved3: 1,
        reserved4: 0,
    };

    let res = encode(&sh);
    res
}

pub fn assemble_string_table() -> Vec<u8> {
    let st = StringTable {
        empty_string: 0,
        text_section_name: ".text\x00".as_bytes().try_into().unwrap(),
        string_table_name: ".shstrtab\x00".to_string().as_bytes().try_into().unwrap(),
    };

    let res = encode(&st);
    res
}

pub fn entry_point_code(message_buffer_offset: u64) -> Vec<u8> {
    let mut message_buffer_address = message_buffer_offset.to_le_bytes().to_vec();
    // let mut ep = b"\xb8\x01\x00\x00\x00\
    //             \xbf\x01\x00\x00\x00\
    //             \x48\xbe\xa0\x81\x02\x00\x00\x00\x00\x00\
    //             \xba\x0e\x00\x00\x00\
    //             \x0f\x05\
    //             \xb8\x3c\x00\x00\x00\
    //             \xbf\x00\x00\x00\x00\
    //             \x0f\x05\
    //             ";
    let mut vec = Vec::new();
    vec.append(&mut b"\xb8\x01\x00\x00\x00\
                    \xbf\x01\x00\x00\x00\
                    \x48\xbe\
                ".to_vec());
    vec.append(&mut message_buffer_address);
    vec.append(&mut b"\xba\x0e\x00\x00\x00\
                \x0f\x05\
                \xb8\x3c\x00\x00\x00\
                \xbf\x00\x00\x00\x00\
                \x0f\x05\
                ".to_vec());
    vec 
}

pub fn message_buffer() -> Vec<u8> {
    let message = b"\x48\x65\x6c\x6c\x6f\x2c\x20\x77\x6f\x72\x6c\x64\x0a\x00";
    message.to_vec()
}

pub fn parse_input_formula(input: &String) -> Vec<Equation> {
    let mut equations = Vec::new();

    for formula in input.split(";") {
        let arr = formula.trim().split("=").collect::<Vec<_>>();
        let function_name = arr[0].chars()
                                .take_while(|&ch| ch != '(')
                                .collect::<String>();
        let start_pos = arr[0].find('(').unwrap();
        let end_pos = arr[0].find(')').unwrap();
        let number_of_arguments = &arr[0][start_pos + 1..end_pos]
                                                    .split(',')
                                                    .collect::<Vec<_>>()
                                                    .len();
        let equation = arr[1];
        let f = parse(&equation.to_owned());
        // println!("{}", formula_parser::print(&f.unwrap()));
        let tree = &f.unwrap();
        equations.push(Equation {function_name: function_name, tree: tree.clone(), number_of_arguments: *number_of_arguments});
        // println!("{:?}", tree);
    }

    equations
}

fn combine(tree: &ParseNode) -> Vec<u8> {
    match tree.entry {
        GrammarItem::Paren => {
            combine(tree.children.get(0).expect("parens need one child"))
        }
        GrammarItem::Sum => {
            let mut lhs = combine(tree.children.get(0).expect("sums need two children"));
            let mut rhs = combine(tree.children.get(1).expect("sums need two children"));
            let mut v = Vec::new();
            v.append(&mut lhs);
            v.append(&mut rhs);
            v.append(&mut b"\x48\x01\xc8".to_vec());
            v.append(&mut b"\x50".to_vec());
            v
        }
        GrammarItem::Product => {
            let mut lhs = combine(tree.children.get(0).expect("products need two children"));
            let mut rhs = combine(tree.children.get(1).expect("products need two children"));
            let mut v = Vec::new();
            v.append(&mut lhs);
            v.append(&mut rhs);
            v.append(&mut b"\x48\xf7\xe1".to_vec());
            v
        }
        GrammarItem::Div => {
            let mut lhs = combine(tree.children.get(0).expect("divider need two children"));
            let mut rhs = combine(tree.children.get(1).expect("divider need two children"));
            let mut v = Vec::new();
            v.append(&mut lhs);
            v.append(&mut rhs);
            let arg_from_stack = lhs.is_empty() || rhs.is_empty();
            if arg_from_stack {
                v.append(&mut b"\x48\x8b\x04\x24".to_vec());
            }
            v.append(&mut b"\x48\xf7\xf1".to_vec());
            if arg_from_stack {
                v.append(&mut b"\x5a".to_vec());
            }
            v
        }
        GrammarItem::Number(n) => {
            let mut v = b"\xb9".to_vec();
            v.append(&mut (n as u32).to_le_bytes().to_vec());
            v
        },
        GrammarItem::Arg(n) => {
            let v = match n {
                'x' => b"\x48\x8b\x45\x24",
                'y' => b"\x48\x8b\x4d\x16",
                _ => b"\x48\x8b\x4d\x16",
            };
            v.to_vec()
        },
    }
}

fn equation_to_code(eq: &Equation) -> Vec<u8> {
    let mut res = Vec::new(); 

    res.append(&mut b"\x55".to_vec());
    res.append(&mut b"\x48\x89\xe5".to_vec());

    res.append(&mut combine(&eq.tree));

    res.append(&mut b"\xc3".to_vec());
    // println!("{:?}", res);

    res

    // let avg = b"\x55\
    //             \x48\x89\xe5\
    //             \x48\x8b\x45\x18\
    //             \x48\x8b\x4d\x10\
    //             \x48\x01\xc8\
    //             \x50\
    //             \x48\x8b\x04\x24\
    //             \xb9\x02\x00\x00\x00\
    //             \x48\xf7\xf1\
    //             \x5a\
    //             \x88\x04\x25\x33\x82\x02\x00\
    //             \xb8\x01\x00\x00\x00\
    //             \xbf\x01\x00\x00\x00\
    //             \x48\xbe\x33\x82\x02\x00\x00\x00\x00\x00\
    //             \xba\x10\x00\x00\x00\
    //             \x0f\x05\
    //             \x5d\
    //             \xc3\
    //             ";
    // avg.to_vec()    
}