
pub type Range = core::ops::Range<usize>;
pub type RangeI = core::ops::RangeInclusive<usize>;

pub mod arm11 {
    use super::{Range, RangeI, common};
    pub use super::n3ds::arm11 as n3ds;
    pub use common::*;

    pub const         BOOTROM: Range  = 0x00000000 ..  0x00010000;
    pub const BOOTROM_MIRROR1: Range  = 0x00010000 ..  0x00020000;
    pub const     PRIVATE_MEM: Range  = 0x17E00000 ..  0x17E02000;
    pub const BOOTROM_MIRROR2: RangeI = 0xFFFF0000 ..= 0xFFFFFFFF;
}

pub mod arm9 {
    use super::{Range, RangeI, common};
    pub use super::n3ds::arm9 as n3ds;
    pub use common::*;

    pub const INSTRUCTION_TCM1: Range  = 0x00000000 ..  0x08000000;
    pub const INSTRUCTION_TCM2: Range  = 0x01FF8000 ..  0x02000000;
    pub const INSTRUCTION_TCM3: Range  = 0x07FF8000 ..  0x08000000;
    pub const      PRIVATE_MEM: Range  = 0x08000000 ..  0x08100000;
    pub const         DATA_TCM: Range  = 0xFFF00000 ..  0xFFF04000;
    pub const          BOOTROM: RangeI = 0xFFFF0000 ..= 0xFFFFFFFF;
}

mod common {
    use super::Range;

    pub const     VRAM: Range = 0x18000000 .. 0x18600000;
    pub const  DSP_MEM: Range = 0x1FF00000 .. 0x1FF80000;
    pub const AXI_WRAM: Range = 0x1FF80000 .. 0x20000000;
    pub const    FCRAM: Range = 0x20000000 .. 0x28000000;
}

mod n3ds {
    use super::Range;

    mod common {
        use super::Range;

        pub const EXTRA_FCRAM: Range = 0x28000000 .. 0x30000000;
    }

    pub mod arm9 {
        use super::Range;
        pub use super::common::*;

        pub const EXTENSION: Range = 0x08100000 .. 0x08180000;
    }

    pub mod arm11 {
        use super::Range;
        pub use super::common::*;

        pub const L2_CACHE_CONTROLLER: Range = 0x17E10000 .. 0x17E11000;
        pub const           EXTRA_MEM: Range = 0x1F000000 .. 0x1F400000;
    }
}
