use std::{fs::File, io::BufWriter};

use object::{build, elf, write::StreamingBuffer};

pub fn build_object(program: &[u8]) {
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
    section.data = build::elf::SectionData::Data(program.into());
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
    symbol.st_size = program.len() as u64;
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

    use crate::{assembler::assemble, linker::link_binary};

    use super::*;

    #[test]
    fn it_works() {
        let program = assemble().unwrap();
        build_object(&program);
        link_binary(&["out.o".into()], Path::new("out")).unwrap();
    }
}
