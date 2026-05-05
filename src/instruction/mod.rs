use enum_dispatch::enum_dispatch;

pub struct Push {
    operand: i32,
}

impl Push {
    pub fn new(operand: i32) -> Self {
        Self { operand }
    }
}

pub struct Add;

pub struct Print;

impl Instruction for Print {
    fn execute(&self, stack: &mut Vec<i32>) -> () {
        let value = stack.pop();
        assert!(value.is_some(), "Stack is empty, value is None.");
        println!("{}\n", value.unwrap());
    }
}

impl Instruction for Push {
    fn execute(&self, stack: &mut Vec<i32>) -> () {
        stack.push(self.operand);
    }
}

impl Instruction for Add {
    fn execute(&self, stack: &mut Vec<i32>) -> () {
        let a = stack.pop();
        let b = stack.pop();
        assert!(a.is_some(), "Stack is empty, a is None.");
        assert!(b.is_some(), "Stack is empty, b is None.");
        stack.push(a.unwrap() + b.unwrap());
    }
}

#[enum_dispatch]
pub trait Instruction {
    fn execute(&self, stack: &mut Vec<i32>) -> ();
}

#[enum_dispatch(Instruction)]
pub enum InstructionSet {
    Push,
    Add,
    Print,
}
