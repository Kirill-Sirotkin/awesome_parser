use std::fs;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub example, "/example.rs");

fn main() {
    let data = fs::read_to_string("./src/text_to_parse.txt").unwrap();
    let result = example::ExprParser::new().parse(&data).unwrap();
    result.pretty_print(0);
}
