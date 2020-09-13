#[cfg(has_asm)]
use ckb_vm::{
    machine::{
        aot::AotCompilingMachine,
        asm::{AsmCoreMachine, AsmMachine},
        VERSION1,
    },
    DefaultMachineBuilder, ISA_B, ISA_IMAC,
};
use ckb_vm::{run, SparseMemory};

use bytes::Bytes;
use std::fs::File;
use std::io::Read;

#[cfg(has_asm)]
#[test]
pub fn test_b_extension_asm() {
    let mut file = File::open("tests/programs/b_extension").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer = Bytes::from(buffer);

    let asm_core = AsmCoreMachine::new(ISA_IMAC | ISA_B, VERSION1, u64::max_value());
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core).build();
    let mut machine = AsmMachine::new(core, None);

    machine
        .load_program(&buffer, &["b_extension".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[cfg(has_asm)]
#[test]
pub fn test_b_extension_aot() {
    let mut file = File::open("tests/programs/b_extension").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let asm_core = AsmCoreMachine::new(ISA_IMAC | ISA_B, VERSION1, u64::max_value());
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core).build();
    let mut aot_machine =
        AotCompilingMachine::load(&buffer, None, ISA_IMAC | ISA_B, VERSION1).unwrap();
    let code = aot_machine.compile().unwrap();
    let mut machine = AsmMachine::new(core, Some(&code));

    machine
        .load_program(&buffer, &vec!["b_extension".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_b_extension_rust() {
    let mut file = File::open("tests/programs/b_extension").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer = Bytes::from(buffer);

    let result = run::<u64, SparseMemory<u64>>(&buffer, &vec!["b_extension".into()]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}
