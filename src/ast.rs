use std::fmt::Display;

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
