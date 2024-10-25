pub mod elf;
pub mod linker;

pub fn assemble() -> std::result::Result<Vec<u8>, Box<dyn std::error::Error>> {
    use iced_x86::code_asm::*;

    let mut a = CodeAssembler::new(64)?;

    /*
        endbr64
        push   rbp
        mov    rbp,rsp
        mov    DWORD PTR [rbp-0x4],0x0
        mov    eax,0x2
        pop    rbp
        ret
    */
    a.endbr64()?;
    a.push(rbp)?;
    a.mov(rbp, rsp)?;
    a.mov(dword_ptr(rbp - 0x4), 0)?;
    a.mov(eax, 2u32)?;
    a.pop(rbp)?;
    a.ret()?;

    let bytes = a.assemble(0x0)?;
    Ok(bytes)
}
