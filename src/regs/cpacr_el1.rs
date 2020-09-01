//! Architectural Feature Access Control Register - EL1
//! 
//! Controls access to trace, SVE, Advanced SIMD and floating-point functionality.

use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u64,
    pub CPACR_EL1 [
        TTA  OFFSET(28) NUMBITS(1) [
            None = 0,
            All = 1
        ],

        FPEN OFFSET(20) NUMBITS(2) [
            All = 0b00,
            El0 = 0b01,
            None = 0b11
        ],

        ZEN  OFFSET(16) NUMBITS(2) [
            All = 0b00,
            El0 = 0b01,
            None = 0b11
        ]
    ]
}

pub struct Reg;

impl RegisterReadWrite<u64, CPACR_EL1::Register> for Reg {
    sys_coproc_read_raw!(u64, "CPACR_EL1");
    sys_coproc_write_raw!(u64, "CPACR_EL1");
}

pub static CPACR_EL1: Reg = Reg {};
