//! Rust wrappers around RISC-V routines
// All referenced from xv6-riscv/kernel/riscv.h

use core::arch::asm;

// MPP := Machine previous protection mode.
pub const MSTATUS_MPP_MASK: u64 = 3 << 11; // Mask for bit tricks
pub const MSTATUS_MPP_M: u64 = 3 << 11; // Machine
pub const MSTATUS_MPP_S: u64 = 1 << 11; // Supervisor
pub const MSTATUS_MPP_U: u64 = 0 << 11; // User
pub const MSTATUS_MIE: u64 = 1 << 3; // machine-mode interrupt enable.

// sstatus := Supervisor status reg.
pub const SSTATUS_SPP: u64 = 1 << 8;  // Previous mode, 1=Supervisor, 0=User
pub const SSTATUS_SPIE: u64 = 1 << 5; // Supervisor Previous Interrupt Enable
pub const SSTATUS_UPIE: u64 = 1 << 4; // User Previous Interrupt Enable
pub const SSTATUS_SIE: u64 = 1 << 1;  // Supervisor Interrupt Enable
pub const SSTATUS_UIE: u64 = 1 << 0;  // User Interrupt Enable

// Machine-mode Interrupt Enable
pub const MIE_MEIE: u64 = 1 << 11; // external
pub const MIE_MTIE: u64 = 1 << 7; // timer
pub const MIE_MSIE: u64 = 1 << 3;  // software

// Supervisor Interrupt Enable
pub const SIE_SEIE: u64 = 1 << 9; // external
pub const SIE_STIE: u64 = 1 << 5; // timer
pub const SIE_SSIE: u64 = 1 << 1; // software

// CLINT := Core local interruptor (where the timer is).
// CLINT_BASE: usize = 0x2000000; // clint is at this location in memlayout.
// xv6-riscv C code:
// #define CLINT_MTIMECMP(hartid) (CLINT + 0x4000 + 8*(hartid))
// #define  CLINT_MTIME (CLINT + 0xBFF8) // cycles since boot.
// int interval = 1000000; // cycles; about 1/10th second in qemu.
// *(uint64*)CLINT_MTIMECMP(id) = *(uint64*)CLINT_MTIME + interval;

// Need to write a value to the CLINT memory location.
// This is mmio, as such there are safety concerns:
//      https://doc.rust-lang.org/std/ptr/fn.write_volatile.html
// 
// Generate a machine lvl interrupt by setting mtime to be >= mtimecmp.
pub fn write_clint(hartid: u64, base: usize, interval: u64) {
    // Ok, treat base addr as a pointer we can write to.
    let base = (base + 0x4000 + 8 * (hartid as usize)) as *mut u64;
    unsafe {
        base.write_volatile(base as u64 + 0xBFF8 + interval);
    }
}



// Return id of current hart.
// the "m" in "mstatus" means machine mode.
// Note mstatus -> sstatus reg for supervisor mode.
pub fn read_mhartid() -> u64 {
    let id: u64;
    // Volatile by default?
    unsafe {
        asm!("csrr {}, mhartid", out(reg) id);
    }
    id
}

// Read CSR := Control and Status Register mstatus.
// Refer to chap 9 of riscv isa manual for info on CSRs.
pub fn read_mstatus() -> u64 {
    let status: u64;
    unsafe {
        asm!("csrr {}, mstatus", out(reg) status);
    }
    status
}

// Write to mstatus.
pub fn write_mstatus(status: u64) {
    unsafe {
        asm!("csrw mstatus, {}", in(reg) status);
    }
}

// Set mepc := machine exception program counter.
// a.k.a. what instr (address) to go to from exception.
pub fn write_mepc(addr: *const ()) {
    unsafe {
        asm!("csrw mepc, {}", in(reg) addr);
    }
}

pub fn read_sstatus() -> u64 {
    let status: u64;
    unsafe {
        asm!("csrr {}, sstatus", out(reg) status);
    }
    status
}

pub fn write_status(status: u64) {
    unsafe {
        asm!("csrw sstatus, {}", in(reg) status);
    }
}

// Enable sup mode interrupt and exception. 
pub fn read_sip() -> u64 {
    let x: u64;
    unsafe {
        asm!("csrr {}, sip", out(reg) x);
    }
    x
}

pub fn write_sip(ire: u64) {
    unsafe {
        asm!("csrw sip, {}", in(reg) ire);
    }
}

pub fn read_sie() -> u64 {
    let x: u64;
    unsafe {
        asm!("csrr {}, sie", out(reg) x);
    }
    x
}

pub fn write_sie(ire: u64) {
    unsafe {
        asm!("csrw sie, {}", in(reg) ire);
    }
}

pub fn read_mie() -> u64 {
    let x: u64;
    unsafe {
        asm!("csrr {}, mie", out(reg) x);
    }
    x
}

pub fn write_mie(x: u64) {
    unsafe {
        asm!("csrw mie, {}", in(reg) x);
    }
}


// SATP := supervisor address translation and protection.
// This is where we hold the page table address.
// use riscv's sv39 page table scheme.
//
// For reference:
// #define SATP_SV39 (8L << 60)
// #define MAKE_SATP(pagetable) (SATP_SV39 | (((uint64)pagetable) >> 12))
pub fn read_satp() -> u64 {
    let pt: u64;
    unsafe {
        asm!("csrr {}, satp", out(reg) pt);
    }
    pt
}

 pub fn write_satp(pt: u64) {
     unsafe {
         asm!("csrw satp, {}", in(reg) pt);
     }
 }

// medeleg := machine exception delegation (to supervisor mode)
// mideleg := machine interrupt delegation (to supervisor mode)
pub fn read_medeleg() -> u64 {
    let med: u64;
    unsafe {
        asm!("csrr {}, medeleg", out(reg) med);
    }
    med
}

pub fn write_medeleg(med: u64) {
    unsafe {
        asm!("csrw medeleg, {}", in(reg) med);
    }
}

pub fn read_mideleg() -> u64 {
    let mid: u64;
    unsafe {
        asm!("csrr {}, mideleg", out(reg) mid);
    }
    mid
}

pub fn write_mideleg(mid: u64) {
    unsafe {
        asm!("csrw mideleg, {}", in(reg) mid);
    }
}

// pmpaddr := phys mem protection addr. 
// Configure to give supervisor mode access to
// certain parts of memory.
pub fn write_pmpaddr0(addr: u64) {
    unsafe {
        asm!("csrw pmpaddr0, {}", in(reg) addr);
    }
}

pub fn write_pmpcfg0(addr: u64) {
    unsafe {
        asm!("csrw pmpcfg0, {}", in(reg) addr);
    }
}

// Just for curiosity's sake:
// https://github.com/rust-lang/rust/issues/82753
// 
// tp := thread pointer?
// This way we can query a hart's hartid and store it in tp reg.
pub fn write_tp(id: u64) {
    unsafe {
        asm!("mv tp, {}", in(reg) id);
    }
}

pub fn read_tp() -> u64 {
    let tp: u64;
    unsafe {
        asm!("mv {}, tp", out(reg) tp);
    }
    tp
}

// Make sure mret has an addr to go to!
pub fn call_mret() {
    unsafe {
        asm!("mret");
    }
}

pub fn write_mscratch(scratch: usize) {
    unsafe {
        asm!("csrw mscratch, {}", in(reg) scratch);
    }
}

// Give address of timervec address.
pub fn write_mtvec(addr: *const ()) {
    unsafe {
        asm!("csrw mtvec, {}", in(reg) addr);
    }
}

























