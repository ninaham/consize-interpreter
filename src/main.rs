use std::{
    env,
    io::{stdout, Error, Write},
};

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
            //"\\ prelude.txt run say-hi".to_string(),
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
        Err(e) => {
            stdout().flush().unwrap();
            eprintln!("{}", e)
        }
    }
}

fn call(int: Interpreter) -> Result<Box<Interpreter>, Error> {
    let mut int1 = Box::new(int.uncomment()?.tokenize()?.get_dict()?.func()?);

    int1.datastack.push(StackElement::SubStack(Vec::new()));
    int1.swap()?.apply()
}
