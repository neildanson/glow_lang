use crate::Function;

pub trait Compiler {
    type F;
    fn compile(f : Function) -> Self::F;
}

pub trait Runtime {
    type F;
    fn run(&self, f : &Self::F);
}