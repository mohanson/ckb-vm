// Macro-Operation Fusion (also Macro-Op Fusion, MOP Fusion, or Macrofusion) is a hardware optimization technique found
// in many modern microarchitectures whereby a series of adjacent macro-operations are merged into a single
// macro-operation prior or during decoding. Those instructions are later decoded into fused-µOPs.
//
// - https://riscv.org/wp-content/uploads/2016/07/Tue1130celio-fusion-finalV2.pdf
// - https://en.wikichip.org/wiki/macro-operation_fusion#Proposed_fusion_operations
// - https://carrv.github.io/2017/papers/clark-rv8-carrv2017.pdf

use super::super::decoder::Decoder as RawDecoder;
use super::super::instructions::{extract_opcode, instruction_length, Instruction, Register};
use super::super::memory::Memory;
use super::super::Error;
use super::{Itype, R4type, Rtype, Utype};
use ckb_vm_definitions::instructions::{self as insts};
use ckb_vm_definitions::registers::RA;

#[derive(Default)]
pub struct Decoder {
    raw_decoder: RawDecoder,
    open: bool,
}

impl Decoder {
    pub fn new(decoder: RawDecoder, open: bool) -> Decoder {
        Decoder {
            raw_decoder: decoder,
            open,
        }
    }

    #[allow(clippy::needless_return)]
    pub fn decode<R: Register, M: Memory<R>>(
        &self,
        memory: &mut M,
        pc: u64,
    ) -> Result<Instruction, Error> {
        let b1 = self.raw_decoder.decode(memory, pc)?;
        if !self.open {
            return Ok(b1);
        }
        let o1 = extract_opcode(b1);
        match o1 {
            insts::OP_LUI => {
                let i1 = Utype(b1);
                let s1 = instruction_length(b1);
                let b2 = self.raw_decoder.decode(memory, pc + s1 as u64)?;
                let o2 = extract_opcode(b2);
                match o2 {
                    insts::OP_JALR => {
                        let i2 = Itype(b2);
                        if i2.rs1() == i1.rd() && i2.rd() == RA {
                            let imm = i1.immediate_s() + i2.immediate_s();
                            let ia = Utype::new_s(insts::OP_FAR_JUMP_ABS, i1.rd(), imm);
                            let s2 = instruction_length(b2);
                            let sa = (s1 + s2) as u64 >> 1 << 24;
                            return Ok(ia.0 | sa);
                        }
                        return Ok(b1);
                    }
                    insts::OP_ADDIW => {
                        let i2 = Itype(b2);
                        if i2.rs1() == i2.rd() && i2.rd() == i1.rd() {
                            let imm = i1.immediate_s() + i2.immediate_s();
                            let ia =
                                Utype::new_s(insts::OP_LD_SIGN_EXTENDED_32_CONSTANT, i1.rd(), imm);
                            let s2 = instruction_length(b2);
                            let sa = (s1 + s2) as u64 >> 1 << 24;
                            return Ok(ia.0 | sa);
                        }
                        return Ok(b1);
                    }
                    _ => Ok(b1),
                }
            }
            insts::OP_AUIPC => {
                let i1 = Utype(b1);
                let s1 = instruction_length(b1);
                let b2 = self.raw_decoder.decode(memory, pc + s1 as u64)?;
                let o2 = extract_opcode(b2);
                match o2 {
                    insts::OP_JALR => {
                        let i2 = Itype(b2);
                        if i2.rs1() == i1.rd() && i2.rd() == RA {
                            let imm = i1.immediate_s() + i2.immediate_s();
                            let ia = Utype::new_s(insts::OP_FAR_JUMP_REL, i1.rd(), imm);
                            let s2 = instruction_length(b2);
                            let sa = (s1 + s2) as u64 >> 1 << 24;
                            return Ok(ia.0 | sa);
                        }
                        return Ok(b1);
                    }
                    _ => Ok(b1),
                }
            }
            insts::OP_MULH => {
                let i1 = Rtype(b1);
                if i1.rd() == i1.rs1() || i1.rd() == i1.rs2() {
                    return Ok(b1);
                }
                let s1 = instruction_length(b1);
                let b2 = self.raw_decoder.decode(memory, pc + s1 as u64)?;
                let o2 = extract_opcode(b2);
                match o2 {
                    insts::OP_MUL => {
                        let i2 = Rtype(b2);
                        if i1.rs1() != i2.rs1() || i1.rs2() != i2.rs2() {
                            return Ok(b1);
                        }
                        let s2 = instruction_length(b2);
                        let ia =
                            R4type::new(insts::OP_WIDE_MUL, i1.rd(), i1.rs1(), i1.rs2(), i2.rd());
                        let sa = (s1 + s2) as u64 >> 1 << 24;
                        return Ok(ia.0 | sa);
                    }
                    _ => Ok(b1),
                }
            }
            insts::OP_MULHU => {
                let i1 = Rtype(b1);
                if i1.rd() == i1.rs1() || i1.rd() == i1.rs2() {
                    return Ok(b1);
                }
                let s1 = instruction_length(b1);
                let b2 = self.raw_decoder.decode(memory, pc + s1 as u64)?;
                let o2 = extract_opcode(b2);
                match o2 {
                    insts::OP_MUL => {
                        let i2 = Rtype(b2);
                        if i1.rs1() != i2.rs1() || i1.rs2() != i2.rs2() {
                            return Ok(b1);
                        }
                        let s2 = instruction_length(b2);
                        let ia =
                            R4type::new(insts::OP_WIDE_MULU, i1.rd(), i1.rs1(), i1.rs2(), i2.rd());
                        let sa = (s1 + s2) as u64 >> 1 << 24;
                        return Ok(ia.0 | sa);
                    }
                    _ => Ok(b1),
                }
            }
            insts::OP_DIV => {
                let i1 = Rtype(b1);
                if i1.rd() == i1.rs1() || i1.rd() == i1.rs2() {
                    return Ok(b1);
                }
                let s1 = instruction_length(b1);
                let b2 = self.raw_decoder.decode(memory, pc + s1 as u64)?;
                let o2 = extract_opcode(b2);
                match o2 {
                    insts::OP_REM => {
                        let i2 = Rtype(b2);
                        if i1.rs1() != i2.rs1() || i1.rs2() != i2.rs2() {
                            return Ok(b1);
                        }
                        let s2 = instruction_length(b2);
                        let ia =
                            R4type::new(insts::OP_WIDE_DIV, i1.rd(), i1.rs1(), i1.rs2(), i2.rd());
                        let sa = (s1 + s2) as u64 >> 1 << 24;
                        return Ok(ia.0 | sa);
                    }
                    _ => Ok(b1),
                }
            }
            insts::OP_DIVU => {
                let i1 = Rtype(b1);
                if i1.rd() == i1.rs1() || i1.rd() == i1.rs2() {
                    return Ok(b1);
                }
                let s1 = instruction_length(b1);
                let b2 = self.raw_decoder.decode(memory, pc + s1 as u64)?;
                let o2 = extract_opcode(b2);
                match o2 {
                    insts::OP_REMU => {
                        let i2 = Rtype(b2);
                        if i1.rs1() != i2.rs1() || i1.rs2() != i2.rs2() {
                            return Ok(b1);
                        }
                        let s2 = instruction_length(b2);
                        let ia =
                            R4type::new(insts::OP_WIDE_DIVU, i1.rd(), i1.rs1(), i1.rs2(), i2.rd());
                        let sa = (s1 + s2) as u64 >> 1 << 24;
                        return Ok(ia.0 | sa);
                    }
                    _ => Ok(b1),
                }
            }
            _ => Ok(b1),
        }
    }
}
