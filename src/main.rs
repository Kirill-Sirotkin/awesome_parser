use std::fs;

use awesome_parser::ast::{Expr, FunCode, OpCode, Statement};

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub example, "/example.rs");

fn main() {
    let data = fs::read_to_string("./src/text_to_parse.txt").unwrap();
    let result = example::StmParser::new().parse(&data).unwrap();
    result.pretty_print(0);
    eval(result);
}

fn eval(parse_result: Box<Statement>) {
    match *parse_result {
        Statement::Seq(lhs, rhs) => {
            eval(lhs);
            eval(rhs);
        }
        Statement::Fun(fun, expr) => {
            eval_fun(fun, expr);
        }
    }
}

fn eval_math(expr: Box<Expr>) -> i32 {
    match *expr {
        Expr::Number(num) => num,
        Expr::Op(lhs, op, rhs) => match op {
            OpCode::Mul => eval_math(lhs) * eval_math(rhs),
            OpCode::Div => eval_math(lhs) / eval_math(rhs),
            OpCode::Add => eval_math(lhs) + eval_math(rhs),
            OpCode::Sub => eval_math(lhs) - eval_math(rhs),
            OpCode::Exp => f32::powi(eval_math(lhs) as f32, eval_math(rhs)) as i32,
        },
    }
}

fn eval_fun(fun: FunCode, expr: Box<Expr>) {
    match fun {
        FunCode::Print => println!("{}", eval_math(expr)),
    }
}
