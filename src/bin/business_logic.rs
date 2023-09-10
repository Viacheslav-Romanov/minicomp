use crate::formula_parser::*;
use crate::minimal_elf::*;

#[derive(Debug)]
pub struct Equation {
    function_name: String,
    tree: ParseNode,
    arguments: Vec<char>,
}

pub fn generate_code_section(equations: &Vec<Equation>) -> Vec<u8> {
    let mut res = Vec::new();
    for eq in equations {
        res.append(&mut equation_to_code(eq));
    }
    
    res
}

pub fn assemble_binary(equations: &Vec<Equation>) -> Vec<u8> {
    let mut assembly = Vec::new();

    assembly.append(&mut assemble_elf_header(0x138));
    assembly.append(&mut assemble_program_header(0x179));
    assembly.append(&mut assemble_section_header(0x188, 0x179, 0));
    assembly.append(&mut assemble_symtab_section_header(0, 0));
    assembly.append(&mut assemble_strtab_section_header(0, 0));
    assembly.append(&mut generate_code_section(&equations));

    let entry_point_offset = assembly.len() as u64;

    assembly.append(&mut entry_point_code(0x281a0));

    let message_buffer_offset = assembly.len() as u64;

    assembly.append(&mut message_buffer());

    let string_table_offset = assembly.len() as u64;

    assembly.append(&mut assemble_string_table());

    let symtab_table_offset = assembly.len() as u64;

    assembly.append(&mut assemble_symtab_table(0, 10, 20));

    let strtab_table_offset = assembly.len() as u64;

    assembly.append(&mut assemble_strtab_table());

    let file_size = assembly.len() as u64;

    assembly.clear();

    // need a second pass here to update the binary with calculated values
    assembly.append(&mut assemble_elf_header(entry_point_offset));
    assembly.append(&mut assemble_program_header(string_table_offset));
    assembly.append(&mut assemble_section_header(file_size, string_table_offset, symtab_table_offset - string_table_offset));
    assembly.append(&mut assemble_symtab_section_header(symtab_table_offset, strtab_table_offset - symtab_table_offset));
    assembly.append(&mut assemble_strtab_section_header(strtab_table_offset, file_size - strtab_table_offset));
    assembly.append(&mut generate_code_section(&equations));
    assembly.append(&mut entry_point_code(FILE_LOAD_VA + message_buffer_offset));
    assembly.append(&mut message_buffer());
    assembly.append(&mut assemble_string_table());
    assembly.append(&mut assemble_symtab_table(entry_point_offset, entry_point_offset - 0x48, entry_point_offset - 0x2f));
    assembly.append(&mut assemble_strtab_table());
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
        number_of_section_header_entries: 5,
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

pub fn assemble_section_header(file_size: u64, string_table_offset: u64, string_table_size: u64) -> Vec<u8> {
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
        aligment: 0,
        entry_size: 0,
        string_table: 7, //calculate string_table_name - string_table
        string_table_index: 3,
        loadable: 0,
        string_table_address: FILE_LOAD_VA + string_table_offset, //calculate file_load_va + string_table
        string_table_offset: string_table_offset, //calculate string_table
        string_table_size: string_table_size, //calculate string_table_end - string_table
        reserved1: 0,
        reserved2: 0,
        reserved3: 1,
        reserved4: 0,
    };

    let res = encode(&sh);
    res
}

pub fn assemble_symtab_section_header(symtab_offset: u64, symtab_table_size: u64) -> Vec<u8> {
    let sh = SectionHeader {
        name: 17, // address of the .symtab text 
        bits: 2,
        flags: 0,
        addr: 0,
        offset: symtab_offset,
        size: symtab_table_size,
        link: 4,
        info: 0,
        addralign: 0,
        entsize: 0x18,
    };

    let res = encode(&sh);
    res
}

pub fn assemble_strtab_section_header(strtab_offset: u64, strtab_table_size: u64) -> Vec<u8> {
    let sh = SectionHeader {
        name: 25, // address of the .strtab text 
        bits: 3,
        flags: 0,
        addr: 0,
        offset: strtab_offset,
        size: strtab_table_size,
        link: 0,
        info: 0,
        addralign: 0,
        entsize: 0,
    };

    let res = encode(&sh);
    res
}

pub fn assemble_string_table() -> Vec<u8> {
    b"\x00.text\x00.shstrtab\x00.symtab\x00.strtab\x00".to_vec()
}

pub fn assemble_symtab_table(entry_point_offset: u64, avg_offset: u64, quad_offset: u64) -> Vec<u8> {
    let mut vec = Vec::new();
    //NULL entry
    vec.append(&mut encode(SymabEntry {
        name: 0,
        info: 0,
        other: 0,
        shndx: 0,
        value: 0,
        size: 0,
    }));
    // entry point entry
    vec.append(&mut encode(SymabEntry {
        name: 1, // address of entry name
        info: 0x10,
        other: 0,
        shndx: 1,
        value: FILE_LOAD_VA + entry_point_offset,
        size: 0,
    }));
    // avg entry
    vec.append(&mut encode(SymabEntry {
        name: 0x0d, // address of entry name
        info: 0,
        other: 0,
        shndx: 1,
        value: FILE_LOAD_VA + avg_offset,
        size: 0,
    }));
    // quad entry
    vec.append(&mut encode(SymabEntry {
        name: 0x11, // address of entry name
        info: 0,
        other: 0,
        shndx: 1,
        value: FILE_LOAD_VA + quad_offset,
        size: 0,
    }));
    vec
}

pub fn assemble_strtab_table() -> Vec<u8> {
    b"\x00entry_point\x00avg\x00quad\x00".to_vec()
}

pub fn entry_point_code(message_buffer_offset: u64) -> Vec<u8> {
    let mut message_buffer_address = message_buffer_offset.to_le_bytes().to_vec();
    let mut message_buffer_address_u32 = (message_buffer_offset as u32).to_le_bytes().to_vec();
    let mut vec = Vec::new();
    // call the functions here and print the results
    // avg(x,y) = (100 + 80) / 2 = 90 results in Z ASCII character
    // quad(x, a, b, c) = (2*2*1 + 30*2 + 4) = 68 results in D ASCII character
    // execute ./miniout.elf in the console
    // Print results will look as follows:
    // Z <- result  
    // D <- result
    vec.append(&mut b"\x6a\x64\
                    \x6a\x50\
                    \xe8\xaf\xff\xff\xff\
                    \x88\x04\x25\
                    ".to_vec());
    vec.append(&mut message_buffer_address_u32);
    vec.append(&mut b"\xb8\x01\x00\x00\x00\
                    \xbf\x01\x00\x00\x00\
                    \x48\xbe\
                    ".to_vec());
    vec.append(&mut message_buffer_address);
    vec.append(&mut b"\xba\x0e\x00\x00\x00\
                \x0f\x05\
                ".to_vec());
    vec.append(&mut b"\x6a\x04\
                    \x6a\x1e\
                    \x6a\x01\
                    \x6a\x02\
                    \xe8\x99\xff\xff\xff\
                    \x88\x04\x25\
                    ".to_vec());
    vec.append(&mut (message_buffer_offset as u32).to_le_bytes().to_vec());
    vec.append(&mut b"\xb8\x01\x00\x00\x00\
                    \xba\x0e\x00\x00\x00\
                    \x0f\x05\
                    ".to_vec());
    vec.append(&mut b"\xb8\x3c\x00\x00\x00\
                \xbf\x00\x00\x00\x00\
                \x0f\x05\
                ".to_vec());
    vec 
}

pub fn message_buffer() -> Vec<u8> {
    let message = b"\x48\x20\x3c\x2d\x20\x72\x65\x73\x75\x6c\x74\x20\x0a\x00";
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
        let arguments = &arr[0][start_pos + 1..end_pos]
                                                    .split(',').map(|c| c.trim().chars().next().unwrap())
                                                    .collect::<Vec<char>>();
        let equation = arr[1];
        let f = parse(&equation.to_owned());
        // println!("{}", formula_parser::print(&f.unwrap()));
        let tree = &f.unwrap();
        equations.push(Equation {function_name: function_name, tree: tree.clone(), arguments: arguments.clone()});
        // println!("{:?}", tree);
    }

    equations
}

fn get_arguments(tree: &ParseNode) -> (&GrammarItem, &GrammarItem) {
    let lhs_type = &tree.children.get(0).unwrap().entry;
    let rhs_type = &tree.children.get(1).unwrap().entry;
    (lhs_type, rhs_type)
}

fn set_arguments_to_different_regs(tree: &ParseNode, rhs: &Vec<u8>, lhs: &Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    let res = match get_arguments(&tree) {
        (GrammarItem::Arg(_), GrammarItem::Arg(_)) => {
            let mut r = Vec::new();
            r.append(&mut b"\x48\x8b\x45".to_vec());
            r.append(&mut rhs.clone());
            let mut l = Vec::new();
            l.append(&mut b"\x48\x8b\x4d".to_vec());
            l.append(&mut lhs.clone());
            (r, l)
        },
        (_, GrammarItem::Arg(_)) => {
            let mut r = Vec::new();
            r.append(&mut b"\x48\x8b\x45".to_vec());
            r.append(&mut rhs.clone());
            (lhs.clone(), r)
        },
        (GrammarItem::Arg(_), _) => {
            let mut l = Vec::new();
            l.append(&mut b"\x48\x8b\x45".to_vec());
            l.append(&mut lhs.clone());
            (l, rhs.clone())
        },
        _ => (rhs.clone(), lhs.clone())
    };
    res
}

fn combine(tree: &ParseNode, args: &Vec<char>) -> Vec<u8> {
    match tree.entry {
        GrammarItem::Paren => {
            combine(tree.children.get(0).expect("parens need one child"), args)
        }
        GrammarItem::Sum => {
            let mut lhs = combine(tree.children.get(0).expect("sums need two children"), args);
            let mut rhs = combine(tree.children.get(1).expect("sums need two children"), args);
            let both_sides_operators= match get_arguments(&tree) {
                (_ , GrammarItem::Arg(_) | GrammarItem::Number(_))
                | (GrammarItem::Arg(_) | GrammarItem::Number(_) , _)  => false,
                _ => true
            };
            // println!("Product_Type: lhs_type={:#04x?} rhs_type={:#04x?}", lhs_type, rhs_type);
            let mut v = Vec::new();
            if rhs[..3] == lhs[..3] {
                rhs[2] = 0x45; //change register to rcx
            }
            // (rhs, lhs) = set_arguments_to_different_regs(tree, &rhs, &lhs);
            v.append(&mut lhs);
            v.append(&mut rhs);
            if both_sides_operators {
                v.append(&mut b"\x59".to_vec());
            }
            v.append(&mut b"\x48\x01\xc8".to_vec());
            // v.append(&mut b"\x50".to_vec());
            v
        }
        GrammarItem::Product => {
            let mut lhs = combine(tree.children.get(0).expect("products need two children"), args);
            let mut rhs = combine(tree.children.get(1).expect("products need two children"), args);
            let one_side_operator= match get_arguments(&tree) {
                (GrammarItem::Arg(_) | GrammarItem::Number(_), GrammarItem::Arg(_) | GrammarItem::Number(_)) => false,
                _ => true
            };
            // println!("Product_Type: lhs_type={:#04x?} rhs_type={:#04x?}", lhs_type, rhs_type);
            let mut v = Vec::new();
            // (rhs, lhs) = set_arguments_to_different_regs(tree, &rhs, &lhs);
            if rhs[..3] == lhs[..3] {
                rhs[2] = 0x45;
            }
            // println!("Product: lhs={:#04x?} rhs={:#04x?}", lhs, rhs);
            v.append(&mut rhs);
            v.append(&mut lhs);
            // v.append(&mut b"\x48\x8b\x04\x24".to_vec()); //if
            v.append(&mut b"\x48\xf7\xe1".to_vec());
            if one_side_operator {
                v.append(&mut b"\x50".to_vec()); //if
            }
            v
        }
        GrammarItem::Div => {
            let mut lhs = combine(tree.children.get(0).expect("divider need two children"), args);
            let mut rhs = combine(tree.children.get(1).expect("divider need two children"), args);
            let mut v = Vec::new();
            if rhs[..3] == lhs[..3] {
                rhs[2] = 0x45;
            }
            // (rhs, lhs) = set_arguments_to_different_regs(tree, &rhs, &lhs);
            v.append(&mut lhs);
            v.append(&mut rhs);
            v.append(&mut b"\x48\xf7\xf1".to_vec());
            v
        }
        GrammarItem::Number(n) => {
            let mut v = b"\xb9".to_vec();
            v.append(&mut (n as u32).to_le_bytes().to_vec());
            v
        },
        GrammarItem::Arg(n) => {
            let offset = (args.iter()
                                    .position(|&x| x == n)
                                    .unwrap())*8 + 0x10;
            let mut v = b"\x48\x8b\x4d".to_vec();
            v.append(&mut [offset as u8].to_vec()); 
            v
            // [offset as u8].to_vec()
        },
    }
}

fn equation_to_code(eq: &Equation) -> Vec<u8> {
    let mut res = Vec::new(); 

    res.append(&mut b"\x55".to_vec());
    res.append(&mut b"\x48\x89\xe5".to_vec());

    res.append(&mut combine(&eq.tree, &eq.arguments));

    res.append(&mut b"\x5d".to_vec());
    res.append(&mut b"\xc3".to_vec());
    // println!("{:?}", res);

    res
}