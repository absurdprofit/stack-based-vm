use crate::instruction::{Add, Instruction, InstructionSet, Print, Push};

mod instruction;

fn main() {
    let program: Vec<InstructionSet> = vec![
        Push::new(50).into(),
        Push::new(51).into(),
        Add.into(),
        Print.into(),
    ];
    let mut stack = Vec::new();

    for instruction in program {
        instruction.execute(&mut stack);
    }
}
