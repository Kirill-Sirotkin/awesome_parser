use std::fmt::Display;

#[derive(Debug)]
pub enum Statement {
    Seq(Box<Statement>, Box<Statement>),
    Fun(FunCode, Box<Expr>),
}
impl Statement {
    pub fn pretty_print(&self, num: usize) {
        print!("{}", "_".repeat(num));

        match self {
            Statement::Fun(fun, expr) => {
                println!("{}", fun);
                expr.pretty_print(num + 1)
            }
            Statement::Seq(lhs, rhs) => {
                lhs.pretty_print(num + 1);
                rhs.pretty_print(num + 1);
                println!("{}", "-".repeat(20));
            }
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Op(Box<Expr>, OpCode, Box<Expr>),
}
impl Expr {
    pub fn pretty_print(&self, num: usize) {
        print!("{}", "_".repeat(num));

        match self {
            Expr::Number(number) => println!("{}", number),
            Expr::Op(lhs, op, rhs) => {
                println!("{}", op);
                lhs.pretty_print(num + 1);
                rhs.pretty_print(num + 1);
            }
        }
    }
}

#[derive(Debug)]
pub enum FunCode {
    Print,
}
impl Display for FunCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FunCode::Print => write!(f, "Print"),
        }
    }
}

#[derive(Debug)]
pub enum OpCode {
    Mul,
    Div,
    Add,
    Sub,
    Exp,
}
impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::Mul => write!(f, "*"),
            OpCode::Div => write!(f, "/"),
            OpCode::Add => write!(f, "+"),
            OpCode::Sub => write!(f, "-"),
            OpCode::Exp => write!(f, "^"),
        }
    }
}
