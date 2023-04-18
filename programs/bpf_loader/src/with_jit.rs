miraland_sdk::declare_builtin!(
    miraland_sdk::bpf_loader::ID,
    solana_bpf_loader_program_with_jit,
    solana_bpf_loader_program::process_instruction_jit
);
