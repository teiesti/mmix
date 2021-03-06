use machine::state::State;

/// octa difference immediate
pub fn odifi(state: &mut State, x: u8, y: u8, z: u8) {
    // Load first operand
    let op1: u64 = state.gpr[y].into();

    // Execute
    let result = op1.saturating_sub(z as u64);

    // Store result
    state.gpr[x] = result.into();
}
