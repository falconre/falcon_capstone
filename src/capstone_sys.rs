#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

use std::{convert::From, mem::transmute};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const CS_MODE_LITTLE_ENDIAN: cs_mode = cs_mode_CS_MODE_LITTLE_ENDIAN;
pub const CS_MODE_ARM: cs_mode = cs_mode_CS_MODE_ARM;
pub const CS_MODE_16: cs_mode = cs_mode_CS_MODE_16;
pub const CS_MODE_32: cs_mode = cs_mode_CS_MODE_32;
pub const CS_MODE_64: cs_mode = cs_mode_CS_MODE_64;
pub const CS_MODE_THUMB: cs_mode = cs_mode_CS_MODE_THUMB;
pub const CS_MODE_MCLASS: cs_mode = cs_mode_CS_MODE_MCLASS;
pub const CS_MODE_V8: cs_mode = cs_mode_CS_MODE_V8;
pub const CS_MODE_MICRO: cs_mode = cs_mode_CS_MODE_MICRO;
pub const CS_MODE_MIPS3: cs_mode = cs_mode_CS_MODE_MIPS3;
pub const CS_MODE_MIPS32R6: cs_mode = cs_mode_CS_MODE_MIPS32R6;
pub const CS_MODE_V9: cs_mode = cs_mode_CS_MODE_V9;
pub const CS_MODE_BIG_ENDIAN: cs_mode = cs_mode_CS_MODE_BIG_ENDIAN;
pub const CS_MODE_MIPS32: cs_mode = cs_mode_CS_MODE_MIPS32;
pub const CS_MODE_MIPS64: cs_mode = cs_mode_CS_MODE_MIPS64;

#[cfg(not(feature = "capstone4"))]
pub const CS_MODE_MIPSGP64: cs_mode = cs_mode_CS_MODE_MIPSGP64;
#[cfg(feature = "capstone4")]
pub const CS_MODE_MPIS2: cs_mode = cs_mode_CS_MODE_MIPS2;
#[cfg(feature = "capstone4")]
pub const CS_MODE_QPX: cs_mode = cs_mode_CS_MODE_QPX;
#[cfg(feature = "capstone4")]
pub const CS_MODE_M68K_000: cs_mode = cs_mode_CS_MODE_M68K_000;
#[cfg(feature = "capstone4")]
pub const CS_MODE_M68K_010: cs_mode = cs_mode_CS_MODE_M68K_010;
#[cfg(feature = "capstone4")]
pub const CS_MODE_M68K_020: cs_mode = cs_mode_CS_MODE_M68K_020;
#[cfg(feature = "capstone4")]
pub const CS_MODE_M68K_030: cs_mode = cs_mode_CS_MODE_M68K_030;
#[cfg(feature = "capstone4")]
pub const CS_MODE_M68K_040: cs_mode = cs_mode_CS_MODE_M68K_040;
#[cfg(feature = "capstone4")]
pub const CS_MODE_M68K_060: cs_mode = cs_mode_CS_MODE_M68K_060;
#[cfg(feature = "capstone4")]
pub const CS_MODE_M680X_6301: cs_mode = cs_mode_CS_MODE_M680X_6301;
#[cfg(feature = "capstone4")]
pub const CS_MODE_M680X_6309: cs_mode = cs_mode_CS_MODE_M680X_6309;
#[cfg(feature = "capstone4")]
pub const CS_MODE_M680X_6800: cs_mode = cs_mode_CS_MODE_M680X_6800;
#[cfg(feature = "capstone4")]
pub const CS_MODE_M680X_6801: cs_mode = cs_mode_CS_MODE_M680X_6801;
#[cfg(feature = "capstone4")]
pub const CS_MODE_M680X_6805: cs_mode = cs_mode_CS_MODE_M680X_6805;
#[cfg(feature = "capstone4")]
pub const CS_MODE_M680X_6808: cs_mode = cs_mode_CS_MODE_M680X_6808;
#[cfg(feature = "capstone4")]
pub const CS_MODE_M680X_6809: cs_mode = cs_mode_CS_MODE_M680X_6809;
#[cfg(feature = "capstone4")]
pub const CS_MODE_M680X_6811: cs_mode = cs_mode_CS_MODE_M680X_6811;
#[cfg(feature = "capstone4")]
pub const CS_MODE_M680X_CPU12: cs_mode = cs_mode_CS_MODE_M680X_CPU12;
#[cfg(feature = "capstone4")]
pub const CS_MODE_M680X_HCS08: cs_mode = cs_mode_CS_MODE_M680X_HCS08;

// Union field getters.
impl cs_x86_op {
    pub fn reg(&self) -> x86_reg {
        unsafe { self.__bindgen_anon_1.reg }
    }
    pub fn imm(&self) -> i64 {
        unsafe { self.__bindgen_anon_1.imm }
    }
    #[cfg(not(feature = "capstone4"))]
    pub fn fp(&self) -> f64 {
        unsafe { self.__bindgen_anon_1.fp }
    }
    pub fn mem(&self) -> &x86_op_mem {
        unsafe { &self.__bindgen_anon_1.mem }
    }
}

#[cfg(feature = "capstone4")]
impl cs_x86 {
    pub fn eflags(&self) -> u64 {
        unsafe { self.__bindgen_anon_1.eflags }
    }
    pub fn fpu_flags(&self) -> u64 {
        unsafe { self.__bindgen_anon_1.fpu_flags }
    }
}

impl cs_arm64_op {
    pub fn reg(&self) -> arm64_reg {
        unsafe { self.__bindgen_anon_1.reg }
    }
    pub fn imm(&self) -> i64 {
        unsafe { self.__bindgen_anon_1.imm }
    }
    pub fn fp(&self) -> f64 {
        unsafe { self.__bindgen_anon_1.fp }
    }
    pub fn mem(&self) -> &arm64_op_mem {
        unsafe { &self.__bindgen_anon_1.mem }
    }
    pub fn pstate(&self) -> &arm64_pstate {
        unsafe { &self.__bindgen_anon_1.pstate }
    }
    pub fn sys(&self) -> u32 {
        unsafe { self.__bindgen_anon_1.sys }
    }
    pub fn prefetch(&self) -> &arm64_prefetch_op {
        return unsafe { &self.__bindgen_anon_1.prefetch };
    }
    pub fn barrier(&self) -> &arm64_barrier_op {
        unsafe { &self.__bindgen_anon_1.barrier }
    }
}

impl cs_arm_op {
    pub fn reg(&self) -> arm_reg {
        #[cfg(not(feature = "capstone4"))]
        unsafe {
            self.__bindgen_anon_1.reg
        }

        // capstone/arm.h uses a plain int for the reg field rather than an arm_reg for some
        // reason, so we have to do some weird `as` business to get this to typecheck
        #[cfg(feature = "capstone4")]
        unsafe {
            (self.__bindgen_anon_1.reg as u32).into()
        }
    }
    pub fn imm(&self) -> i32 {
        unsafe { self.__bindgen_anon_1.imm }
    }
    pub fn fp(&self) -> f64 {
        unsafe { self.__bindgen_anon_1.fp }
    }
    pub fn mem(&self) -> &arm_op_mem {
        unsafe { &self.__bindgen_anon_1.mem }
    }
    pub fn setend(&self) -> &arm_setend_type {
        unsafe { &self.__bindgen_anon_1.setend }
    }
}

impl cs_mips_op {
    pub fn reg(&self) -> mips_reg {
        unsafe { self.__bindgen_anon_1.reg }
    }
    pub fn imm(&self) -> i64 {
        unsafe { self.__bindgen_anon_1.imm }
    }
    pub fn mem(&self) -> &mips_op_mem {
        unsafe { &self.__bindgen_anon_1.mem }
    }
}

impl cs_ppc_op {
    pub fn reg(&self) -> ppc_reg {
        unsafe { self.__bindgen_anon_1.reg }
    }
    #[cfg(not(feature = "capstone4"))]
    pub fn imm(&self) -> i32 {
        unsafe { self.__bindgen_anon_1.imm }
    }
    #[cfg(feature = "capstone4")]
    pub fn imm(&self) -> i64 {
        unsafe { self.__bindgen_anon_1.imm }
    }
    pub fn mem(&self) -> &ppc_op_mem {
        unsafe { &self.__bindgen_anon_1.mem }
    }
    pub fn crx(&self) -> &ppc_op_crx {
        unsafe { &self.__bindgen_anon_1.crx }
    }
}

impl cs_sparc_op {
    pub fn reg(&self) -> sparc_reg {
        unsafe { self.__bindgen_anon_1.reg }
    }
    #[cfg(not(feature = "capstone4"))]
    pub fn imm(&self) -> i32 {
        unsafe { self.__bindgen_anon_1.imm }
    }
    #[cfg(feature = "capstone4")]
    pub fn imm(&self) -> i64 {
        unsafe { self.__bindgen_anon_1.imm }
    }
    pub fn mem(&self) -> &sparc_op_mem {
        unsafe { &self.__bindgen_anon_1.mem }
    }
}

impl cs_sysz_op {
    pub fn reg(&self) -> sysz_reg {
        unsafe { self.__bindgen_anon_1.reg }
    }
    pub fn imm(&self) -> i64 {
        unsafe { self.__bindgen_anon_1.imm }
    }
    pub fn mem(&self) -> &sysz_op_mem {
        unsafe { &self.__bindgen_anon_1.mem }
    }
}

impl cs_xcore_op {
    pub fn reg(&self) -> xcore_reg {
        unsafe { self.__bindgen_anon_1.reg }
    }
    pub fn imm(&self) -> i32 {
        unsafe { self.__bindgen_anon_1.imm }
    }
    pub fn mem(&self) -> &xcore_op_mem {
        unsafe { &self.__bindgen_anon_1.mem }
    }
}

#[cfg(feature = "capstone4")]
impl cs_tms320c64x_op {
    pub fn reg(&self) -> tms320c64x_reg {
        unsafe { self.__bindgen_anon_1.reg.into() }
    }
    pub fn imm(&self) -> i32 {
        unsafe { self.__bindgen_anon_1.imm }
    }
    pub fn mem(&self) -> &tms320c64x_op_mem {
        unsafe { &self.__bindgen_anon_1.mem }
    }
}

#[cfg(feature = "capstone4")]
impl cs_m680x_op {
    pub fn imm(&self) -> i32 {
        unsafe { self.__bindgen_anon_1.imm }
    }
    pub fn reg(&self) -> m680x_reg {
        unsafe { self.__bindgen_anon_1.reg }
    }
    pub fn idx(&self) -> m680x_op_idx {
        unsafe { self.__bindgen_anon_1.idx }
    }
    pub fn rel(&self) -> m680x_op_rel {
        unsafe { self.__bindgen_anon_1.rel }
    }
    pub fn ext(&self) -> m680x_op_ext {
        unsafe { self.__bindgen_anon_1.ext }
    }
    pub fn direct_addr(&self) -> u8 {
        unsafe { self.__bindgen_anon_1.direct_addr }
    }
    pub fn const_val(&self) -> u8 {
        unsafe { self.__bindgen_anon_1.const_val }
    }
}

#[cfg(feature = "capstone4")]
impl cs_m68k_op {
    pub fn imm(&self) -> u64 {
        unsafe { self.__bindgen_anon_1.imm }
    }
    pub fn dimm(&self) -> f64 {
        unsafe { self.__bindgen_anon_1.dimm }
    }
    pub fn simm(&self) -> f32 {
        unsafe { self.__bindgen_anon_1.simm }
    }
    pub fn reg(&self) -> m68k_reg {
        unsafe { self.__bindgen_anon_1.reg }
    }
    pub fn reg_pair(&self) -> (m68k_reg, m68k_reg) {
        unsafe {
            (
                self.__bindgen_anon_1.reg_pair.reg_0,
                self.__bindgen_anon_1.reg_pair.reg_1,
            )
        }
    }
}
#[cfg(feature = "capstone4")]
impl m68k_op_size {
    pub fn cpu_size(&self) -> m68k_cpu_size {
        unsafe { self.__bindgen_anon_1.cpu_size }
    }
    pub fn fpu_size(&self) -> m68k_fpu_size {
        unsafe { self.__bindgen_anon_1.fpu_size }
    }
}

macro_rules! impl_from_into_int {
    ($name:ident) => {
        impl From<u32> for $name {
            fn from(i: u32) -> Self {
                unsafe { transmute(i) }
            }
        }
        impl Into<u32> for $name {
            fn into(self) -> u32 {
                unsafe { transmute(self) }
            }
        }
        impl $name {
            pub fn as_int(&self) -> u32 {
                (*self).into()
            }
        }
    };
}

// Register: enum <-> integer
impl_from_into_int!(arm_reg);
impl_from_into_int!(arm64_reg);
impl_from_into_int!(mips_reg);
impl_from_into_int!(x86_reg);
impl_from_into_int!(ppc_reg);
impl_from_into_int!(sparc_reg);
impl_from_into_int!(sysz_reg);
impl_from_into_int!(xcore_reg);

#[cfg(feature = "capstone4")]
impl_from_into_int!(m68k_reg);
#[cfg(feature = "capstone4")]
impl_from_into_int!(tms320c64x_reg);
#[cfg(feature = "capstone4")]
impl_from_into_int!(m680x_reg);

// Groups: enum <-> integer.
impl_from_into_int!(arm_insn_group);
impl_from_into_int!(arm64_insn_group);
impl_from_into_int!(mips_insn_group);
impl_from_into_int!(x86_insn_group);
impl_from_into_int!(ppc_insn_group);
impl_from_into_int!(sparc_insn_group);
impl_from_into_int!(sysz_insn_group);
impl_from_into_int!(xcore_insn_group);

#[cfg(feature = "capstone4")]
impl_from_into_int!(tms320c64x_insn_group);
#[cfg(feature = "capstone4")]
impl_from_into_int!(evm_insn_group);
