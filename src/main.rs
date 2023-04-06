use std::fs;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub example);

fn main() {
    let data = fs::read_to_string("./src/text_to_parse.txt").unwrap();
    example::ProgramExpressionSequenceParser::new()
        .parse(&data)
        .unwrap();
    // println!("parse result: {}", result.unwrap());
}
