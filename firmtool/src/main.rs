use std::fs;
use std::env::args;
use std::iter;
use std::convert::{TryFrom, TryInto};
use firmtool::{Result, Builder, Section, CopyMethod};

fn main() {
    let firm_path = args().nth(1).expect("arg1: must be firm output path");
    let mut builder = Builder::new();

    {
        let path = args().nth(2).expect("arg2: must be arm9 elf path");
        let arm9 = fs::read(path).unwrap();
        let program = extract_elf_program(&arm9).unwrap();
        let section = Section::new(program.addr, CopyMethod::NDMA, program.data).unwrap();
        builder.arm9_entrypoint(program.entrypoint);
        builder.add_section(section);
    }

    if let Some(path) = args().nth(3) {
        let arm11 = fs::read(path).unwrap();
        let program = extract_elf_program(&arm11).unwrap();
        let section = Section::new(program.addr, CopyMethod::XDMA, program.data).unwrap();
        builder.arm11_entrypoint(program.entrypoint);
        builder.add_section(section);
    }

    let firm = builder.build().unwrap();
    fs::write(firm_path, firm).unwrap();
}

fn extract_elf_program(file: &[u8]) -> Result<ElfProgram> {
    use goblin::elf::{Elf, program_header::PT_LOAD};
    
    let elf = Elf::parse(file)?;
    let entrypoint = u32::try_from(elf.entry)?;
    let mut data = Vec::new();
    let mut program_start = None;
    let mut previous_end = None;

    for ph in &elf.program_headers {
        if ph.p_type != PT_LOAD || ph.p_memsz == 0 {
            continue;
        }

        if ph.p_paddr != ph.p_vaddr {
            Err("virtual addresses are not supported")?
        }

        if ph.p_filesz > ph.p_memsz {
            Err("file size cannot exceed mem size")?
        }

        program_start.get_or_insert(ph.p_paddr);

        if *previous_end.get_or_insert(ph.p_paddr) != ph.p_paddr {
            println!("skipping {:?}", ph);
            continue
        }

        previous_end = Some(ph.p_paddr + ph.p_memsz);

        // TODO: check indices
        let segment = &file[ph.file_range()];
        data.extend_from_slice(segment);

        // println!("segment = {:02x?}", segment);

        let zero_fill = ph.p_memsz - ph.p_filesz;
        data.extend(iter::repeat(0).take(zero_fill as usize));
    }

    Ok(ElfProgram {
        entrypoint,
        addr: program_start.ok_or("No suitable segments found")?
            .try_into().map_err(|_| "Program is too big")?,
        data,
    })
}

struct ElfProgram {
    entrypoint: u32,
    addr: u32,
    data: Vec<u8>,
}
