use crate::Result;
use std::io::Write;
use byteorder::{WriteBytesExt, LE};

pub const SIZE: u32 = 0x200;

pub struct Header {
    pub boot_priority: u32,
    pub arm11_entrypoint: u32,
    pub arm9_entrypoint: u32,
    pub reserved: [u8; 0x30],
    pub section_headers: [SectionHeader; 4],
    pub rsa_signature: [u8; 0x100]
}

impl Default for Header {
    fn default() -> Self {
        Self {
            boot_priority: 0,
            arm11_entrypoint: 0,
            arm9_entrypoint: 0,
            reserved: [0; 0x30],
            section_headers: <_>::default(),
            rsa_signature: [0; 0x100],
        }
    }
}

impl Header {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_writer<W: Write>(&self, w: &mut W) -> Result {
        w.write_all(b"FIRM")?;
        w.write_u32::<LE>(self.boot_priority)?;
        w.write_u32::<LE>(self.arm11_entrypoint)?;
        w.write_u32::<LE>(self.arm9_entrypoint)?;
        w.write_all(&self.reserved)?;

        for section_header in &self.section_headers {
            section_header.to_writer(w)?;
        }

        w.write_all(&self.rsa_signature)?;

        Ok(())
    }
}

#[derive(Default)]
pub struct SectionHeader {
    pub offset: u32,
    pub addr: u32,
    pub size: u32,
    pub copy_method: CopyMethod,
    pub sha256: [u8; 32],
}

impl SectionHeader {
    fn to_writer<W: Write>(&self, w: &mut W) -> Result {
        w.write_u32::<LE>(self.offset)?;
        w.write_u32::<LE>(self.addr)?;
        w.write_u32::<LE>(self.size)?;
        w.write_u32::<LE>(self.copy_method as u32)?;
        w.write_all(&self.sha256)?;

        Ok(())
    }
}

#[derive(FromPrimitive,Copy,Clone)]
pub enum CopyMethod {
    NDMA = 0,
    XDMA = 1,
    CPU = 2,
}

impl Default for CopyMethod {
    fn default() -> Self {
        CopyMethod::NDMA
    }
}
