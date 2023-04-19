use {
    solana_sdk::instruction::InstructionError,
    solana_program_runtime::invoke_context::InvokeContext,
};

pub fn process_instruction(
    _first_instruction_account: usize,
    _invoke_context: &mut InvokeContext,
) -> Result<(), InstructionError> {
    // Do nothing, compute budget instructions handled by the runtime
    Ok(())
}
