use std::{fs::File, io::BufWriter};

use object::{build, elf, write::StreamingBuffer};

pub fn build_object() {
    let mut builder = build::elf::Builder::new(object::Endianness::Little, true);
    builder.header.e_type = elf::ET_REL;
    builder.header.e_machine = elf::EM_X86_64;
    builder.header.e_phoff = 0x40;

    let section = builder.sections.add();
    section.name = b".shstrtab"[..].into();
    section.sh_type = elf::SHT_STRTAB;
    section.data = build::elf::SectionData::SectionString;

    let section = builder.sections.add();
    section.name = b".strtab"[..].into();
    section.sh_type = elf::SHT_STRTAB;
    section.data = build::elf::SectionData::String;

    let section = builder.sections.add();
    section.name = b".text"[..].into();
    section.sh_type = elf::SHT_PROGBITS;
    section.sh_flags = (elf::SHF_ALLOC | elf::SHF_EXECINSTR) as u64;
    section.sh_addralign = 16;
    // program data goes here.
    let my_func_data: Vec<u8> = vec![
        0xf3, 0x0f, 0x1e, 0xfa, // endbr64
        0x55, // push   %rbp
        0x48, 0x89, 0xe5, // mov    %rsp,%rbp
        0xc7, 0x45, 0xfc, 0x00, 0x00, 0x00, 0x00, //  movl   $0x0,-0x4(%rbp)
        0xb8, 0x02, 0x00, 0x00, 0x00, // mov    $0x2,%eax
        0x5d, // pop    %rbp
        0xc3, // ret
    ];
    let my_func_data_size = my_func_data.len();
    section.data = build::elf::SectionData::Data(my_func_data.into());
    let text_id = section.id();

    let section = builder.sections.add();
    section.name = b".symtab"[..].into();
    section.sh_type = elf::SHT_SYMTAB;
    section.sh_flags = elf::SHF_ALLOC as u64;
    section.sh_addralign = 8;
    section.data = build::elf::SectionData::Symbol;

    let symbol = builder.symbols.add();
    symbol.name = b".text"[..].into();
    symbol.set_st_info(elf::STB_LOCAL, elf::STT_SECTION);
    symbol.section = Some(text_id);

    // Add symbols
    let symbol = builder.symbols.add();
    symbol.name = b"main"[..].into();
    symbol.set_st_info(elf::STB_GLOBAL, elf::STT_FUNC);
    symbol.st_size = my_func_data_size as u64;
    symbol.section = Some(text_id);
    // offset to the function within .text
    // Note: i think it needs to be 16 byte aligned
    symbol.st_value = 0;

    builder.set_section_sizes();

    let mut file = StreamingBuffer::new(BufWriter::new(File::create("out.o").unwrap()));
    builder.write(&mut file).unwrap();
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::linker::link_binary;

    use super::*;

    #[test]
    fn it_works() {
        build_object();
        link_binary(&["out.o".into()], Path::new("out")).unwrap();
    }
}
