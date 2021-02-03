mod machine_run;

#[test]
pub fn test_b_extension() {
    machine_run::int_v1_imcb("tests/programs/b_extension");
    #[cfg(has_asm)]
    machine_run::asm_v1_imcb("tests/programs/b_extension");
    #[cfg(has_asm)]
    machine_run::aot_v1_imcb("tests/programs/b_extension");
}

#[test]
pub fn test_clzw_bug() {
    machine_run::int_v1_imcb("tests/programs/clzw_bug");
    #[cfg(has_asm)]
    machine_run::asm_v1_imcb("tests/programs/clzw_bug");
    #[cfg(has_asm)]
    machine_run::aot_v1_imcb("tests/programs/clzw_bug");
}
