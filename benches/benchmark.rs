extern crate glow_lang;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;

use glow_lang::virtual_machine::*;
use glow_lang::*;

fn parse_success(c: &mut Criterion) {
    let any_number = any_of(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);
    let number_parser = pchar('-').optional().then(any_number.many1());

    let to_number = number_parser.map(move |(negate, value): (Option<char>, Vec<char>)| {
        let string: String = value.into_iter().collect();
        let number = string.parse::<i32>().unwrap();
        match negate {
            Some(_) => -number,
            None => number,
        }
    });

    c.bench_function("Parse Success", |b| {
        b.iter(|| {
            for _ in 0..100 {
                let _ = black_box(to_number.parse("-123456789".to_string()));
            }
        })
    });
}

fn parse_fail(c: &mut Criterion) {
    let any_number = any_of(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);
    let number_parser = pchar('-').optional().then(any_number.many1());

    let to_number = number_parser.map(move |(negate, value): (Option<char>, Vec<char>)| {
        let string: String = value.into_iter().collect();
        let number = string.parse::<i32>().unwrap();
        match negate {
            Some(_) => -number,
            None => number,
        }
    });

    c.bench_function("Parse Fail", |b| {
        b.iter(|| {
            for _ in 0..100 {
                let _ = black_box(to_number.parse("-12345678B".to_string()));
            }
        })
    });
}

fn stack_vm_addition(c: &mut Criterion) {
    let mut instructions = vec![stack::vm::Instruction::Push(stack::vm::Values::Int(0))];
    for i in 0..1000 {
        instructions.push(stack::vm::Instruction::Push(stack::vm::Values::Int(i)));
        instructions.push(stack::vm::Instruction::Add);
    }

    let function = vm::Function::new(Vec::new(), instructions);
    let program = stack::vm::Program::new(HashMap::new());

    c.bench_function("VM Addition", |b| {
        b.iter(|| {
            black_box(program.eval(&function, &Vec::new()));
        })
    });
}

fn stack_vm_loop(c: &mut Criterion) {
    let mut instructions = vec![];
    instructions.push(stack::vm::Instruction::Push(stack::vm::Values::Int(0)));
    instructions.push(stack::vm::Instruction::StoreLocal("Local".to_string())); // <- Load 0 into local
    instructions.push(stack::vm::Instruction::LoadLocal("Local".to_string()));
    instructions.push(stack::vm::Instruction::Push(stack::vm::Values::Int(1)));
    instructions.push(stack::vm::Instruction::Add); // <- Add 1 to local
    instructions.push(stack::vm::Instruction::StoreLocal("Local".to_string())); // <- Store local back into local (Store pops the stack)
    instructions.push(stack::vm::Instruction::LoadLocal("Local".to_string()));
    instructions.push(stack::vm::Instruction::Push(stack::vm::Values::Int(1000))); // <--Load 1000
    instructions.push(stack::vm::Instruction::JumpNotEqual(2)); // <-- Jump if local != 1000
    instructions.push(stack::vm::Instruction::LoadLocal("Local".to_string()));
    instructions.push(stack::vm::Instruction::Ret);
    let function = vm::Function::new(Vec::new(), instructions);
    let program = stack::vm::Program::new(HashMap::new());

    c.bench_function("VM Loop", |b| {
        b.iter(|| {
            black_box(program.eval(&function, &Vec::new()));
        })
    });
}

fn stack_vm_simple_add(c: &mut Criterion) {
    let instructions = vec![
        stack::vm::Instruction::Push(stack::vm::Values::Int(1)),
        stack::vm::Instruction::StoreLocal("x".to_string()),
        stack::vm::Instruction::Push(stack::vm::Values::Int(2)),
        stack::vm::Instruction::StoreLocal("y".to_string()),
        stack::vm::Instruction::LoadLocal("x".to_string()),
        stack::vm::Instruction::LoadLocal("y".to_string()),
        stack::vm::Instruction::Add,
    ];
    let function = vm::Function::new(Vec::new(), instructions);
    let program = stack::vm::Program::new(HashMap::new());

    c.bench_function("Stack Simple Add Loop", |b| {
        b.iter(|| {
            black_box(program.eval(&function, &Vec::new()));
        })
    });
}

fn register_vm_simple_add(c: &mut Criterion) {
    let instructions = vec![
        register::vm::Instruction::StoreLocal(0, register::vm::Values::Int(1)),
        register::vm::Instruction::StoreLocal(1, register::vm::Values::Int(2)),
        register::vm::Instruction::Add(0, 1),
    ];
    let function = register::vm::Function::new(Vec::new(), instructions);
    let program = register::vm::Program::new(HashMap::new());

    c.bench_function("Register Simple Add Loop", |b| {
        b.iter(|| {
            black_box(program.eval(&function, &Vec::new()));
        })
    });
}

criterion_group!(
    benches,
    parse_success,
    parse_fail,
    stack_vm_addition,
    stack_vm_loop,
    stack_vm_simple_add,
    register_vm_simple_add
);
criterion_main!(benches);
