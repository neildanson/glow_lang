use std::{collections::HashMap, default};

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Values {
    Int(i32),
    Bool(bool),
    String(String),
    Register(usize), /*
                     Array,
                     Float(f32),
                     Map,
                     Function,
                     Null,
                     Undefined,*/
    #[default]
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Types {
    Int,
    Float,
    Bool,
    String,
    /*Array,
    Map,
    Function,
    Null,
    Undefined,*/
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Nop,
    StoreLocal(usize, Values),
    LoadLocal(usize),
    //Math
    Add(usize, usize),
    Sub(usize, usize),
    Mul(usize, usize),
    Div(usize, usize),
    Mod,
    //Comparison
    Equal,
    NotEqual,
    Gt,
    Gte,
    Lt,
    Lte,
    //Logical Operators
    And,
    Or,
    Ret,
    //Control Flow
    JumpEqual(usize),
    JumpNotEqual(usize),
    JumpUnconditional(usize),
    Call(String),
}

#[derive(Debug)]
pub struct Function {
    pub parameters: Vec<Types>,
    pub instructions: Vec<Instruction>,
}

impl Function {
    pub fn new(parameters: Vec<Types>, instructions: Vec<Instruction>) -> Function {
        Function {
            parameters,
            instructions,
        }
    }
}
pub struct Program {
    functions: HashMap<&'static str, Function>,
}

fn add(left: &Values, right: &Values) -> Values {
    match (left, right) {
        (Values::Int(left), Values::Int(right)) => Values::Int(left + right),
        _ => panic!("Addition not supported for Values"),
    }
}

impl Program {
    pub fn new(functions: HashMap<&'static str, Function>) -> Program {
        Program { functions }
    }
    pub fn eval(&self, function: &Function, params: &[Values]) -> Option<Values> {
        let mut registers = Vec::new();
        Vec::resize(&mut registers, 100, Values::Null);
        for instruction in function.instructions.iter() {
            match instruction {
                Instruction::StoreLocal(index, value) => {
                    registers[*index] = value.clone();
                },
                Instruction::Add(left, right) => {
                    let left = registers[*left].clone();
                    let right = registers[*right].clone();
                    add(&left, &right);
                },
                _ => {},
            }
        }

        None
    }
}
