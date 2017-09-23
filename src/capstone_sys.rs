#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

use std::convert::From;
use std::mem::transmute;

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
pub const CS_MODE_MIPSGP64: cs_mode = cs_mode_CS_MODE_MIPSGP64;
pub const CS_MODE_V9: cs_mode = cs_mode_CS_MODE_V9;
pub const CS_MODE_BIG_ENDIAN: cs_mode = cs_mode_CS_MODE_BIG_ENDIAN;
pub const CS_MODE_MIPS32: cs_mode = cs_mode_CS_MODE_MIPS32;
pub const CS_MODE_MIPS64: cs_mode = cs_mode_CS_MODE_MIPS64;

// Operand enum getters.
impl cs_x86_op {
    pub fn reg(&self) -> x86_reg {
        return unsafe { self.__bindgen_anon_1.reg };
    }
    pub fn imm(&self) -> i64 {
        return unsafe { self.__bindgen_anon_1.imm };
    }
    pub fn fp(&self) -> f64 {
        return unsafe { self.__bindgen_anon_1.fp };
    }
    pub fn mem(&self) -> &x86_op_mem {
        return unsafe { &self.__bindgen_anon_1.mem };
    }
}

impl cs_arm64_op {
    pub fn reg(&self) -> arm64_reg {
        return unsafe { self.__bindgen_anon_1.reg.into() };
    }
    pub fn imm(&self) -> i64 {
        return unsafe { self.__bindgen_anon_1.imm };
    }
    pub fn fp(&self) -> f64 {
        return unsafe { self.__bindgen_anon_1.fp };
    }
    pub fn mem(&self) -> &arm64_op_mem {
        return unsafe { &self.__bindgen_anon_1.mem };
    }
    pub fn pstate(&self) -> &arm64_pstate {
        return unsafe { &self.__bindgen_anon_1.pstate };
    }
    pub fn sys(&self) -> u32 {
        return unsafe { self.__bindgen_anon_1.sys };
    }
    pub fn prefetch(&self) -> &arm64_prefetch_op {
        return unsafe { &self.__bindgen_anon_1.prefetch };
    }
    pub fn barrier(&self) -> &arm64_barrier_op {
        return unsafe { &self.__bindgen_anon_1.barrier };
    }

}

impl cs_arm_op {
    pub fn reg(&self) -> arm_reg {
        return unsafe { self.__bindgen_anon_1.reg.into() };
    }
    pub fn imm(&self) -> i32 {
        return unsafe { self.__bindgen_anon_1.imm };
    }
    pub fn fp(&self) -> f64 {
        return unsafe { self.__bindgen_anon_1.fp };
    }
    pub fn mem(&self) -> &arm_op_mem {
        return unsafe { &self.__bindgen_anon_1.mem };
    }
    pub fn setend(&self) -> &arm_setend_type {
        return unsafe { &self.__bindgen_anon_1.setend };
    }
}

impl cs_mips_op {
    pub fn reg(&self) -> mips_reg {
        return unsafe { self.__bindgen_anon_1.reg.into() };
    }
    pub fn imm(&self) -> i64 {
        return unsafe { self.__bindgen_anon_1.imm };
    }
    pub fn mem(&self) -> &mips_op_mem {
        return unsafe { &self.__bindgen_anon_1.mem };
    }
}

impl cs_ppc_op {
    pub fn reg(&self) -> ppc_reg {
        return unsafe { self.__bindgen_anon_1.reg.into() };
    }
    pub fn imm(&self) -> i32 {
        return unsafe { self.__bindgen_anon_1.imm };
    }
    pub fn mem(&self) -> &ppc_op_mem {
        return unsafe { &self.__bindgen_anon_1.mem };
    }
    pub fn crx(&self) -> &ppc_op_crx {
        return unsafe { &self.__bindgen_anon_1.crx };
    }
}

impl cs_sparc_op {
    pub fn reg(&self) -> sparc_reg {
        return unsafe { self.__bindgen_anon_1.reg.into() };
    }
    pub fn imm(&self) -> i32 {
        return unsafe { self.__bindgen_anon_1.imm };
    }
    pub fn mem(&self) -> &sparc_op_mem {
        return unsafe { &self.__bindgen_anon_1.mem };
    }
}

impl cs_sysz_op {
    pub fn reg(&self) -> sysz_reg {
        return unsafe { self.__bindgen_anon_1.reg.into() };
    }
    pub fn imm(&self) -> i64 {
        return unsafe { self.__bindgen_anon_1.imm };
    }
    pub fn mem(&self) -> &sysz_op_mem {
        return unsafe { &self.__bindgen_anon_1.mem };
    }
}

impl cs_xcore_op {
    pub fn reg(&self) -> xcore_reg {
        return unsafe { self.__bindgen_anon_1.reg.into() };
    }
    pub fn imm(&self) -> i32 {
        return unsafe { self.__bindgen_anon_1.imm };
    }
    pub fn mem(&self) -> &xcore_op_mem {
        return unsafe { &self.__bindgen_anon_1.mem };
    }
}

// Register: enum <-> integer
impl From<u32> for x86_reg {
    fn from(i: u32) -> Self {
        return unsafe { transmute::<u32, Self>(i) }
    }
}
impl x86_reg {
    pub fn as_int(&self) -> u32 {
        return unsafe { transmute::<Self, u32>(*self) }
    }
}

impl From<u32> for arm64_reg {
    fn from(i: u32) -> Self {
        return unsafe { transmute::<u32, Self>(i) }
    }
}
impl arm64_reg {
    pub fn as_int(&self) -> u32 {
        return unsafe { transmute::<Self, u32>(*self) }
    }
}

impl From<u32> for arm_reg {
    fn from(i: u32) -> Self {
        return unsafe { transmute::<u32, Self>(i) }
    }
}
impl arm_reg {
    pub fn as_int(&self) -> u32 {
        return unsafe { transmute::<Self, u32>(*self) }
    }
}

impl From<u32> for mips_reg {
    fn from(i: u32) -> Self {
        return unsafe { transmute::<u32, Self>(i) }
    }
}
impl mips_reg {
    pub fn as_int(&self) -> u32 {
        return unsafe { transmute::<Self, u32>(*self) }
    }
}

impl From<u32> for ppc_reg {
    fn from(i: u32) -> Self {
        return unsafe { transmute::<u32, Self>(i) }
    }
}
impl ppc_reg {
    pub fn as_int(&self) -> u32 {
        return unsafe { transmute::<Self, u32>(*self) }
    }
}

impl From<u32> for sparc_reg {
    fn from(i: u32) -> Self {
        return unsafe { transmute::<u32, Self>(i) }
    }
}
impl sparc_reg {
    pub fn as_int(&self) -> u32 {
        return unsafe { transmute::<Self, u32>(*self) }
    }
}

impl From<u32> for sysz_reg {
    fn from(i: u32) -> Self {
        return unsafe { transmute::<u32, Self>(i) }
    }
}
impl sysz_reg {
    pub fn as_int(&self) -> u32 {
        return unsafe { transmute::<Self, u32>(*self) }
    }
}

impl From<u32> for xcore_reg {
    fn from(i: u32) -> Self {
        return unsafe { transmute::<u32, Self>(i) }
    }
}
impl xcore_reg {
    pub fn as_int(&self) -> u32 {
        return unsafe { transmute::<Self, u32>(*self) }
    }
}

// Groups: enum <-> integer.
impl From<u32> for x86_insn_group {
    fn from(i: u32) -> Self {
        return unsafe { transmute::<u32, Self>(i) }
    }
}
impl x86_insn_group {
    pub fn as_int(&self) -> u32 {
        return unsafe { transmute::<Self, u32>(*self) }
    }
}

impl From<u32> for arm64_insn_group {
    fn from(i: u32) -> Self {
        return unsafe { transmute::<u32, Self>(i) }
    }
}
impl arm64_insn_group {
    pub fn as_int(&self) -> u32 {
        return unsafe { transmute::<Self, u32>(*self) }
    }
}

impl From<u32> for arm_insn_group {
    fn from(i: u32) -> Self {
        return unsafe { transmute::<u32, Self>(i) }
    }
}
impl arm_insn_group {
    pub fn as_int(&self) -> u32 {
        return unsafe { transmute::<Self, u32>(*self) }
    }
}

impl From<u32> for mips_insn_group {
    fn from(i: u32) -> Self {
        return unsafe { transmute::<u32, Self>(i) }
    }
}
impl mips_insn_group {
    pub fn as_int(&self) -> u32 {
        return unsafe { transmute::<Self, u32>(*self) }
    }
}

impl From<u32> for ppc_insn_group {
    fn from(i: u32) -> Self {
        return unsafe { transmute::<u32, Self>(i) }
    }
}
impl ppc_insn_group {
    pub fn as_int(&self) -> u32 {
        return unsafe { transmute::<Self, u32>(*self) }
    }
}

impl From<u32> for sparc_insn_group {
    fn from(i: u32) -> Self {
        return unsafe { transmute::<u32, Self>(i) }
    }
}
impl sparc_insn_group {
    pub fn as_int(&self) -> u32 {
        return unsafe { transmute::<Self, u32>(*self) }
    }
}

impl From<u32> for sysz_insn_group {
    fn from(i: u32) -> Self {
        return unsafe { transmute::<u32, Self>(i) }
    }
}
impl sysz_insn_group {
    pub fn as_int(&self) -> u32 {
        return unsafe { transmute::<Self, u32>(*self) }
    }
}

impl From<u32> for xcore_insn_group {
    fn from(i: u32) -> Self {
        return unsafe { transmute::<u32, Self>(i) }
    }
}
impl xcore_insn_group {
    pub fn as_int(&self) -> u32 {
        return unsafe { transmute::<Self, u32>(*self) }
    }
}
