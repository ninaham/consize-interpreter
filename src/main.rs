use std::{env, io::Error};

use colored::Colorize;
use interpreter::Interpreter;
use stack_element::StackElement;

use crate::stack_element::print_stack;

pub mod interpreter;
pub mod stack_element;

fn main() -> Result<(), Error> {
    let args = env::args();
    let int = Interpreter {
        datastack: vec![StackElement::Word(
            //args.skip(1).collect::<Vec<String>>().join(" "),
            "\\ prelude.txt run say-hi".to_string(),
        )],
        callstack: Vec::new(),
        dictionary: Interpreter::init_dictionary(),
        count: 0,
    };
    let new_int = int.uncomment()?.tokenize()?.get_dict()?.func()?;
    println!("{}", print_stack(&new_int.callstack, false, false));

    println!(
        "{} {}",
        "Consize returns:".yellow().bold(),
        print_stack(&new_int.datastack, false, false)
    );

    Ok(())
}
