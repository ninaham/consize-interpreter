use std::{
    collections::VecDeque,
    fmt::Display,
    io::{self, Error},
};

use colored::Colorize;
use rustyline::Editor;

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
            .for_each(|i| call(&mut stack, i).unwrap_or_else(|e| println!("{e}")));

        println!("{}\n", print_stack(&stack, false));
    }
}

#[derive(Clone, Debug)]
enum StackElement {
    Integer(usize),
    SubStack(Vec<StackElement>),
    Word(String),
    Keyword(String),
}

impl Display for StackElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackElement::SubStack(st) => write!(f, "{} ", print_stack(st, true)),
            StackElement::Word(s) => write!(f, "{} ", s),
            StackElement::Keyword(s) => write!(f, "{} ", s),
            StackElement::Integer(i) => write!(f, "{} ", i),
        }
    }
}

fn print_stack(stack: &Vec<StackElement>, print_brackets: bool) -> String {
    let mut str = "".to_string();
    if print_brackets {
        str.push_str("[ ");
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
            "div" => {
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
            "emptystack" => {
                stack.push(StackElement::SubStack(Vec::new()));
                Ok(())
            }
            "push" => {
                let e = stack.pop().unwrap();
                let substack = stack.pop().unwrap();

                match substack {
                    StackElement::SubStack(ss) => {
                        let mut new_substack = ss.clone();
                        new_substack.push(e);
                        stack.push(StackElement::SubStack(new_substack));
                        Ok(())
                    }
                    _ => Err(Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("{} I can only push to stacks", "Error:".red().bold()),
                    )),
                }
            }
            "type" => {
                match stack.pop().unwrap() {
                    StackElement::Integer(_) => stack.push(StackElement::Word("wrd".to_string())),
                    StackElement::SubStack(_) => stack.push(StackElement::Word("stk".to_string())),
                    StackElement::Word(_) => stack.push(StackElement::Word("wrd".to_string())),
                    StackElement::Keyword(_) => stack.push(StackElement::Word("fct".to_string())),
                };

                Ok(())
            }
            "pop" => {
                let substack = stack.pop().unwrap();
                match substack {
                    StackElement::SubStack(ss) => {
                        let mut new_ss = ss.clone();
                        new_ss.pop().unwrap();
                        stack.push(StackElement::SubStack(new_ss));
                        Ok(())
                    }
                    _ => Err(Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("{} I can only pop from stacks", "Error:".red().bold()),
                    )),
                }
            }
            "top" => {
                let substack = stack.pop().unwrap();
                match substack {
                    StackElement::SubStack(ss) => {
                        let mut new_ss = ss.clone();
                        let el = new_ss.pop().unwrap();
                        stack.push(el);
                        Ok(())
                    }
                    _ => Err(Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("{} I can only pop from stacks", "Error:".red().bold()),
                    )),
                }
            }
            "concat" => {
                if let StackElement::SubStack(mut ss1) = stack.pop().unwrap() {
                    if let StackElement::SubStack(mut ss2) = stack.pop().unwrap() {
                        ss2.append(&mut ss1);
                        stack.push(StackElement::SubStack(ss2))
                    } else {
                        return Err(Error::new(
                            io::ErrorKind::InvalidInput,
                            format!("{} I can only concat stacks", "Error:".red().bold()),
                        ));
                    }
                } else {
                    return Err(Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("{} I can only concat stacks", "Error:".red().bold()),
                    ));
                }

                Ok(())
            }
            "reverse" => {
                if let StackElement::SubStack(mut ss) = stack.pop().unwrap() {
                    ss.reverse();
                    stack.push(StackElement::SubStack(ss))
                } else {
                    return Err(Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("{} I can only reverse stacks", "Error:".red().bold()),
                    ));
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
        StackElement::Word(n) => {
            stack.push(StackElement::Word(n.to_string()));
            Ok(())
        }
    }
}
