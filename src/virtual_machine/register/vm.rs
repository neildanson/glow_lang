
#[derive(Debug, Clone, PartialEq)]
pub enum Values {
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    /*Array,
    Map,
    Function,
    Null,
    Undefined,*/
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
