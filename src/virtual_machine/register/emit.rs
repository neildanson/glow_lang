use super::vm::*;
use crate::{interface::Compiler, *};

pub fn emit(expr: &Expr) -> Vec<Instruction> {
    vec![]
}

pub fn emit_body(expr: &[Expr]) -> Vec<Instruction> {
    vec![]
}

mod tests {
    use super::*;

    #[test]
    fn emit_assign() {
        let expr = Expr::Ident("x".to_string(), Box::new(Expr::Int(1)));
        let result = emit(&expr);

        let expected = vec![Instruction::StoreLocal(0, Values::Int(1))];

        assert_eq!(result, expected);
    }

    #[test]
    fn emit_add() {
        let expr = vec![
            Expr::Ident("x".to_string(), Box::new(Expr::Int(1))),
            Expr::Ident("y".to_string(), Box::new(Expr::Int(2))),
            Expr::Add(
                Box::new(Expr::Symbol("x".to_string())),
                Box::new(Expr::Symbol("y".to_string())),
            ),
        ];
        let result = emit_body(&expr);

        let expected = vec![
            Instruction::StoreLocal(0, Values::Int(1)),
            Instruction::StoreLocal(1, Values::Int(2)),
            Instruction::Add(0, 1),
        ];

        assert_eq!(result, expected);
    }
}
