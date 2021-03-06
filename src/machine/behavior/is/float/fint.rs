use machine::state::State;

pub fn fint(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let op1: f64 = state.gpr[z].into();

    // Execute
    let res: f64 = match y {
        1 => op1.round(),    // round off
        2 => op1.ceil(),     // round up
        3 => op1.floor(),    // round down
        4 => (op1 / 10.0).ceil() * 10.0,    // round to nearest ten
        _ => panic!("no rounding mode"),
    };

    // Store result
    state.gpr[x] = res.into();
}
