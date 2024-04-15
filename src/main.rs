use std::{
    collections::VecDeque,
    fmt::Display,
    io::{self, stdout, Error, Write},
};

use colored::Colorize;

fn main() {
    let mut stack: Vec<StackElement> = Vec::new();
    println!("{}", "Welcome to Consize\n".yellow().bold());
    let stdin = io::stdin();
    loop {
        print!("{}", "=> ".bright_cyan());
        stdout().flush().unwrap_or_else(|_| {
            panic!("{} an error occured flushing stdout", "Error:".red().bold())
        });
        let mut inp = String::new();
        stdin.read_line(&mut inp).unwrap_or_else(|_| {
            panic!(
                "{} something went wrong reading from stdin",
                "Error: ".red().bold()
            )
        });

        let mut input = inp
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<VecDeque<String>>();
        let instructions = &mut parse(&mut input);
        instructions
            .iter()
            .for_each(|i| call(&mut stack, i).unwrap_or_else(|e| println!("{e}")));

        println!("{}\n", print_stack(&stack, false));
    }
}

#[derive(Clone, Debug)]
enum StackElement {
    Integer(usize),
    Keyword(String),
    SubStack(Vec<StackElement>),
    Word(String),
}

impl Display for StackElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackElement::Integer(i) => write!(f, "{} ", i),
            StackElement::Keyword(s) => write!(f, "{} ", s),
            StackElement::SubStack(st) => write!(f, "{} ", print_stack(st, true)),
            StackElement::Word(s) => write!(f, "{} ", s),
        }
    }
}

fn print_stack(stack: &Vec<StackElement>, print_brackets: bool) -> String {
    let mut str = "".to_string();
    if print_brackets {
        str.push('[');
    }
    for i in stack {
        str.push_str(format!("{}", *i).as_str())
    }
    if print_brackets {
        str.push(']');
    }

    str
}

fn parse(input: &mut VecDeque<String>) -> Vec<StackElement> {
    let mut stack = Vec::new();
    while !input.is_empty() {
        let ins = input.pop_front().unwrap();
        if let Ok(n) = ins.parse::<usize>() {
            stack.push(StackElement::Integer(n));
            continue;
        }
        if ins == "[ " {
            let s = parse(input);
            stack.push(StackElement::SubStack(s));
            continue;
        }
        if ins == "]" {
            return stack;
        }
        stack.push(StackElement::Keyword(ins))
    }

    stack
}

fn call(stack: &mut Vec<StackElement>, input: &StackElement) -> Result<(), std::io::Error> {
    match input {
        StackElement::Keyword(k) => match k.as_str() {
            "+" => {
                if stack.len() < 2 {
                    return Err(Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("{} not enough operands", "Error:".red().bold()),
                    ));
                }
                match stack.pop().unwrap() {
                    StackElement::Integer(x) => match stack.pop().unwrap() {
                        StackElement::Integer(y) => {
                            stack.push(StackElement::Integer(x + y));
                            Ok(())
                        }
                        _ => Err(Error::new(
                            io::ErrorKind::InvalidInput,
                            format!("{} I can only add Integers", "Error:".red().bold()),
                        )),
                    },
                    _ => Err(Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("{} I can only add Integers", "Error:".red().bold()),
                    )),
                }
            }
            "-" => {
                if stack.len() < 2 {
                    return Err(Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("{} not enough operands", "Error:".red().bold()),
                    ));
                }
                match stack.pop().unwrap() {
                    StackElement::Integer(x) => match stack.pop().unwrap() {
                        StackElement::Integer(y) => {
                            stack.push(StackElement::Integer(x - y));
                            Ok(())
                        }
                        _ => Err(Error::new(
                            io::ErrorKind::InvalidInput,
                            format!("{} I can only subtract Integers", "Error:".red().bold()),
                        )),
                    },
                    _ => Err(Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("{} I can only subtract Integers", "Error:".red().bold()),
                    )),
                }
            }
            "*" => {
                if stack.len() < 2 {
                    return Err(Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("{} not enough operands", "Error:".red().bold()),
                    ));
                }
                match stack.pop().unwrap() {
                    StackElement::Integer(x) => match stack.pop().unwrap() {
                        StackElement::Integer(y) => {
                            stack.push(StackElement::Integer(x * y));
                            Ok(())
                        }
                        _ => Err(Error::new(
                            io::ErrorKind::InvalidInput,
                            format!("{} I can only multiply Integers", "Error:".red().bold()),
                        )),
                    },
                    _ => Err(Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("{} I can only multiply Integers", "Error:".red().bold()),
                    )),
                }
            }
            "/" => {
                if stack.len() < 2 {
                    return Err(Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("{} not enough operands", "Error:".red().bold()),
                    ));
                }
                match stack.pop().unwrap() {
                    StackElement::Integer(x) => match stack.pop().unwrap() {
                        StackElement::Integer(y) => {
                            stack.push(StackElement::Integer(x / y));
                            Ok(())
                        }
                        _ => Err(Error::new(
                            io::ErrorKind::InvalidInput,
                            format!("{} I can only divide Integers", "Error:".red().bold()),
                        )),
                    },
                    _ => Err(Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("{} I can only divide Integers", "Error:".red().bold()),
                    )),
                }
            }
            "clear" => {
                stack.clear();
                Ok(())
            }
            "dup" => {
                if stack.is_empty() {
                    return Err(Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("{} not enough operands", "Error:".red().bold()),
                    ));
                }
                let a = stack.pop().unwrap();
                stack.push(a.clone());
                stack.push(a);
                Ok(())
            }
            "swap" => {
                if stack.len() < 2 {
                    return Err(Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("{} not enough operands", "Error:".red().bold()),
                    ));
                }
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a.clone());
                stack.push(b.clone());

                Ok(())
            }
            "drop" => {
                if stack.is_empty() {
                    return Err(Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("{} not enough operands", "Error:".red().bold()),
                    ));
                }
                stack.pop().unwrap();

                Ok(())
            }
            "rot" => {
                if stack.len() < 3 {
                    return Err(Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("{} not enough operands", "Error:".red().bold()),
                    ));
                }

                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                let c = stack.pop().unwrap();

                stack.push(b);
                stack.push(a);
                stack.push(c);

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
        StackElement::Integer(n) => {
            stack.push(StackElement::Integer(*n));
            Ok(())
        }
        StackElement::SubStack(s) => {
            stack.push(StackElement::SubStack(s.clone()));
            Ok(())
        }
        StackElement::Word(s) => {
            stack.push(StackElement::Word(s.clone()));
            Ok(())
        }
    }
}
