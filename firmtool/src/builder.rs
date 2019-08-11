use std::io::Write;
use std::convert::TryFrom;
use sha2::{Sha256, Digest};
use crate::{Result, CopyMethod, signature};
use crate::header::{self, Header, SectionHeader};

#[derive(Default)]
pub struct Builder {
    arm9_entrypoint: Option<u32>,
    arm11_entrypoint: Option<u32>,
    sections: Vec<Section>,
}

impl Builder {
    pub fn new() -> Self {
        Builder::default()
    }

    pub fn arm9_entrypoint(&mut self, entry: impl Into<Option<u32>>) -> &mut Self {
        self.arm9_entrypoint = entry.into();
        self
    }
    
    pub fn arm11_entrypoint(&mut self, entry: impl Into<Option<u32>>) -> &mut Self {
        self.arm11_entrypoint = entry.into();
        self
    }

    pub fn add_section(&mut self, section: Section) -> &mut Self {
        self.sections.push(section);
        self
    }

    pub fn build(&mut self) -> Result<Vec<u8>> {
        let mut firm = Vec::new();

        self.write_to(&mut firm)?;

        Ok(firm)
    }

    pub fn write_to<W: Write>(&mut self, w: &mut W) -> Result {
        if self.sections.len() > 4 {
            Err("Firm cannot contain more than fore sections")?
        }

        let arm9_entrypoint = self.arm9_entrypoint.ok_or("Missing arm9 entrypoint")?;
        let mut arm9_entrypoint_found = false;

        for section in &self.sections {
            arm9_entrypoint_found |= section.contains_addr(arm9_entrypoint);
        }

        if !arm9_entrypoint_found {
            Err("arm9 entry does not point into any section")?
        }

        let mut offset = header::SIZE;
        let mut section_headers = <[SectionHeader; 4]>::default();

        for (header, section) in section_headers.iter_mut().zip(&self.sections) {            
            *header = SectionHeader {
                offset,
                addr: section.addr,
                size: section.size(),
                copy_method: section.copy_method,
                sha256: Sha256::digest(&section.data).into(),
            };

            offset += section.size();
        }

        let header = Header {
            boot_priority: 0,
            arm9_entrypoint,
            arm11_entrypoint: self.arm11_entrypoint.unwrap_or(0),
            section_headers,
            reserved: [0; 0x30],
            rsa_signature: signature::NAND_RETAIL,
        };

        header.to_writer(w)?;

        for section in &self.sections {
            w.write_all(&section.data)?;
        }

        Ok(())
    }
}

pub struct Section {
    addr: u32,
    copy_method: CopyMethod,
    data: Vec<u8>,
}

impl Section {
    pub fn new(addr: u32, copy_method: CopyMethod, mut data: Vec<u8>) -> Result<Section> {
        // Align size to 512
        while data.len() % 512 != 0 {
            data.push(0xFF);
        }        

        let size = u32::try_from(data.len())?;

        addr.checked_add(size).ok_or("Section lies outside of address space")?;

        Ok(Self { addr, copy_method, data })
    }

    pub fn addr(&self) -> u32 {
        self.addr
    }

    pub fn contains_addr(&self, addr: u32) ->bool {
        let len = self.data.len() as u32;

        (self.addr .. self.addr + len).contains(&addr)
    }

    pub fn size(&self) -> u32 {
        self.data.len() as u32
    }
}