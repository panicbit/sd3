#[macro_use] extern crate num_derive;

pub type Result<T = (), E = Box<std::error::Error>> = std::result::Result<T, E>;

pub mod header;
pub use header::CopyMethod;

mod builder;
pub use builder::{Builder, Section};

pub mod signature {
    pub const NAND_RETAIL: [u8; 0x100] = *include_bytes!("../signatures/nand_retail");
}
