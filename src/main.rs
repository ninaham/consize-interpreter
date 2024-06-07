use std::{env, rc::Rc};

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
        dictionary: Rc::new(Interpreter::init_dictionary()),
    };

    println!(
        "{} {}",
        "Consize returns:".yellow().bold(),
        print_stack(&call(int).datastack, false, false)
    )
}

fn call(int: Interpreter) -> Interpreter {
    let mut int1 = int.uncomment().tokenize().get_dict().func();

    int1.datastack.push(StackElement::SubStack(Vec::new()));
    int1.swap().apply()
}
