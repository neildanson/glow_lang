use std::collections::{HashMap, BTreeMap};

use super::vm::*;
use crate::{interface::Compiler, *};

pub fn extract_value(expr:&Expr) -> Option<Values> {
    match expr {
        Expr::Int(i) => Some(Values::Int(*i)),
        Expr::Str(s) => Some(Values::String(s.clone())),
        _ => None,
    }
}

pub fn emit(expr: &Expr, locals : &mut HashMap<String, usize>, locals_rev:&mut BTreeMap<usize, String>) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    match expr {
        Expr::Ident(name, expr) => {
            //See if name already has an index
            let index = 
                if let Some(index) = locals.get(name) {
                    *index
                } else {
                    let i = 
                        match locals_rev.last_entry() { 
                            Some(x) => *x.key() + 1,
                            None => 0,
                        };
                    locals_rev.insert(i, name.clone());
                    locals.insert(name.clone(),i);
                    i
                };
            
            instructions.push(Instruction::StoreLocal(index, extract_value(&expr).unwrap()));
        },
        Expr::Add(l,r) => {
            let l = *l.clone();
            let r = *r.clone();
            match (l,r) {
                (Expr::Symbol(l), Expr::Symbol(r)) => {
                    let l = locals.get(&l).unwrap();
                    let r = locals.get(&r).unwrap();
                    instructions.push(Instruction::Add(*l, *r))

                },
                _ => {},
            }
        }
        _ => {},
    }
    instructions
}

pub fn emit_body(exprs: &[Expr]) -> Vec<Instruction> {
    let mut locals :HashMap<String, usize>= HashMap::new();
    let mut locals_rev :BTreeMap<usize, String>= BTreeMap::new();
    
    let mut instructions = Vec::new();
    let mut i = 0;
    for e in exprs {
        let mut instructions_to_add = emit(e, &mut locals, &mut locals_rev);
        instructions.append(&mut instructions_to_add);
    }
    instructions
}

mod tests {
    use super::*;

    #[test]
    fn emit_assign() {
        let expr = vec![Expr::Ident("x".to_string(), Box::new(Expr::Int(1)))];
        let result = emit_body(&expr);

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
