#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Code {
    /// Add Register
    ADDR,
    /// Add Immediate
    ADDI,
    /// Multiply Register
    MULR,
    /// Multiply Immediate
    MULI,
    /// Bitwise AND Register
    BANR,
    /// Bitwise AND Immediate
    BANI,
    /// Bitwise OR Register
    BORR,
    /// Bitwise OR Immediate
    BORI,
    /// Set Register
    SETR,
    /// Set Immediate
    SETI,
    /// Greater Than Immediate/Register
    GTIR,
    /// Greater Than Register/Immediate
    GTRI,
    /// Greater Than Register/Register
    GTRR,
    /// Equal Immediate/Register
    EQIR,
    /// Equal Register/Immediate
    EQRI,
    /// Equal Register/Register
    EQRR,
}

impl Code {
    pub fn all() -> impl Iterator<Item = &'static Code> {
        [
            Code::ADDI,
            Code::ADDR,
            Code::MULR,
            Code::MULI,
            Code::BANR,
            Code::BANI,
            Code::BORR,
            Code::BORI,
            Code::SETR,
            Code::SETI,
            Code::GTIR,
            Code::GTRI,
            Code::GTRR,
            Code::EQIR,
            Code::EQRI,
            Code::EQRR,
        ].into_iter()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Op {
    /// Code
    pub c: usize,

    /// Left operand
    pub l: usize,

    /// Right operand
    pub r: usize,

    /// Destination register
    pub d: usize,
}

impl Op {
    pub fn execute(self, c: Code, reg: &mut [usize]) {
        match c {
        | Code::ADDR => reg[self.d] = reg[self.l] + reg[self.r],
        | Code::ADDI => reg[self.d] = reg[self.l] + self.r,
        | Code::MULR => reg[self.d] = reg[self.l] * reg[self.r],
        | Code::MULI => reg[self.d] = reg[self.l] * self.r,
        | Code::BANR => reg[self.d] = reg[self.l] & reg[self.r],
        | Code::BANI => reg[self.d] = reg[self.l] & self.r,
        | Code::BORR => reg[self.d] = reg[self.l] | reg[self.r],
        | Code::BORI => reg[self.d] = reg[self.l] | self.r,
        | Code::SETR => reg[self.d] = reg[self.l],
        | Code::SETI => reg[self.d] = self.l,
        | Code::GTIR => reg[self.d] = if self.l > reg[self.r] { 1 } else { 0 },
        | Code::GTRI => reg[self.d] = if reg[self.l] > self.r { 1 } else { 0 },
        | Code::GTRR => reg[self.d] = if reg[self.l] > reg[self.r] { 1 } else { 0 },
        | Code::EQIR => reg[self.d] = if self.l == reg[self.r] { 1 } else { 0 },
        | Code::EQRI => reg[self.d] = if reg[self.l] == self.r { 1 } else { 0 },
        | Code::EQRR => reg[self.d] = if reg[self.l] == reg[self.r] { 1 } else { 0 },
        }
    }
}
