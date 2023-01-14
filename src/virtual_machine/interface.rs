use crate::FunctionDef;

pub trait Compiler {
    type F;
    fn compile(f : FunctionDef) -> Self::F;
}

pub trait Runtime {
    type F;
    fn run(&self, f : &Self::F);
}