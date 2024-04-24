pub mod interpreter;
pub mod stack_element;

use std::collections::VecDeque;

use colored::Colorize;
use rustyline::Editor;
use stack_element::StackElement;

use crate::stack_element::print_stack;


fn main() {
    let mut stack: Vec<StackElement> = Vec::new();
    let mut rl = Editor::<()>::new();
    println!("{}", "Welcome to Consize\n".yellow().bold());
    while let Ok(inp) = rl.readline("=> ".bright_cyan().to_string().as_str()) {
        rl.add_history_entry(inp.clone());

        let mut input = inp
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<VecDeque<String>>();
        let instructions = &mut parse(&mut input);
        instructions
            .iter()
            .for_each(|i| call(&mut stack, i).unwrap_or_else(|e| eprintln!("{e}")));

        println!("{}\n", print_stack(&stack, false, false));
    }
}


fn parse(input: &mut VecDeque<String>) -> Vec<StackElement> {
    let mut stack = Vec::new();
    while !input.is_empty() {
        let ins = input.pop_front().unwrap();
        stack.push(StackElement::Keyword(ins))
    }

    stack
}

fn call(stack: &mut Vec<StackElement>, input: &StackElement) -> Result<(), std::io::Error> {
    match input {
        StackElement::Keyword(k) => match k.as_str() {
            
            
            
            
            "clear" => {
                stack.clear();
                Ok(())
            }
            "call" => {
                if let StackElement::SubStack(s) = stack.pop().unwrap() {
                    return s.iter().try_for_each(|i| call(stack, i));
                }

                Ok(())
            }
            s => {
                stack.push(StackElement::Word(s.to_string()));
                Ok(())
            }
        },
        StackElement::SubStack(s) => {
            stack.push(StackElement::SubStack(s.clone()));
            Ok(())
        }
        StackElement::Word(n) => {
            stack.push(StackElement::Word(n.to_string()));
            Ok(())
        }
        StackElement::Map(m) => {
            stack.push(StackElement::Map(m.clone()));
            Ok(())
        }
    }
}
