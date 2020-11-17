#![cfg(has_asm)]

use bytes::Bytes;
use ckb_vm::machine::asm::AsmMachine;
use ckb_vm::{
    machine::{asm::AsmCoreMachine, VERSION1},
    DefaultMachineBuilder, ISA_B, ISA_IMC, ISA_MOP,
};
use std::fs;

#[test]
pub fn test_mop_wide_multiply() {
    let program_path = "tests/programs/mop_wide_multiply";
    let program: Bytes = fs::read(program_path).unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_MOP | ISA_IMC, VERSION1, u64::max_value());
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core).build();
    let mut machine = AsmMachine::new(core, None);
    machine
        .load_program(&program, &[program_path.into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_mop_wide_divide() {
    let program_path = "tests/programs/mop_wide_divide";
    let program: Bytes = fs::read(program_path).unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_MOP | ISA_IMC, VERSION1, u64::max_value());
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core).build();
    let mut machine = AsmMachine::new(core, None);
    machine
        .load_program(&program, &[program_path.into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_mop_far_jump() {
    let program_path = "tests/programs/mop_far_jump";
    let program: Bytes = fs::read(program_path).unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_MOP | ISA_IMC, VERSION1, u64::max_value());
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core).build();
    let mut machine = AsmMachine::new(core, None);
    machine
        .load_program(&program, &[program_path.into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_mop_ld32() {
    let program_path = "tests/programs/mop_ld32";
    let program: Bytes = fs::read(program_path).unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_MOP | ISA_IMC | ISA_B, VERSION1, u64::max_value());
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core).build();
    let mut machine = AsmMachine::new(core, None);
    machine
        .load_program(&program, &[program_path.into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}
