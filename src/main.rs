use std::{env, io::Error};

use colored::Colorize;
use interpreter::Interpreter;
use stack_element::StackElement;

use crate::stack_element::print_stack;

pub mod interpreter;
pub mod stack_element;

fn main() {
    let args = env::args();
    let int = Interpreter {
        datastack: vec![StackElement::Word(
            args.skip(1).collect::<Vec<String>>().join(" "),
        )],
        callstack: Vec::new(),
        dictionary: Interpreter::init_dictionary(),
        count: 0,
    };

    match call(int) {
        Ok(new_int) => println!(
            "{} {}",
            "Consize returns:".yellow().bold(),
            print_stack(&new_int.datastack, false, false)
        ),
        Err(e) => println!("{}", e),
    }
}

fn call(int: Interpreter) -> Result<Interpreter, Error> {
    int.uncomment()?.tokenize()?.get_dict()?.func()
}
