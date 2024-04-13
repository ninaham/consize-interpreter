use std::{collections::VecDeque, io::{self, BufRead}};

fn main() {
    let mut stack: Vec<StackElement> = Vec::new();
    println!("Welcome to Consize \n");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let _line = match line {
            Ok(line) => {
                let mut input = line.split_whitespace().map(|s| s.to_string()).collect::<VecDeque<String>>();
                let instructions = &mut build_stack(&mut input);
                instructions.iter().for_each(|i| call_stack(&mut stack, i));
            },
            Err(_) => panic!("Failed to read line"),
            
        };
        println!("{:?}", stack);
    }
}

#[derive(Clone, Debug)]
enum StackElement {
    Integer(usize),
    Keyword(String),
    SubStack(Vec<StackElement>)
}

fn build_stack(input: &mut VecDeque<String>) -> Vec<StackElement>{
    let mut stack = Vec::new();
    while !input.is_empty() {
        let ins = input.pop_front().unwrap();
        match ins.parse::<usize>() {
            Ok(n) => {
                stack.push(StackElement::Integer(n));
                continue;
            },
            Err(_) => {}
        }
        if ins == "[" {
            let s = build_stack(input);
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

fn call_stack(stack: &mut Vec<StackElement>, input: &StackElement) {
    match input {
        StackElement::Keyword(k) => {
            match k.as_str() {
                "+" => {
                    match stack.pop().unwrap() {
                        StackElement::Integer(x) => {
                            match stack.pop().unwrap() {
                                StackElement::Integer(y) => stack.push(StackElement::Integer(x + y)),
                                _ => panic!("I can only add integers")
                                
                            }
                        }
                        _ => panic!("I can only add integers")
                    }
                },
                "-" => {
                    match stack.pop().unwrap() {
                        StackElement::Integer(x) => {
                            match stack.pop().unwrap() {
                                StackElement::Integer(y) => stack.push(StackElement::Integer(x - y)),
                                _ => panic!("I can only subtract integers")
                                
                            }
                        }
                        _ => panic!("I can only subtract integers")
                    }
                },
                "*" => {
                    match stack.pop().unwrap() {
                        StackElement::Integer(x) => {
                            match stack.pop().unwrap() {
                                StackElement::Integer(y) => stack.push(StackElement::Integer(x * y)),
                                _ => panic!("I can only multiply integers")
                                
                            }
                        }
                        _ => panic!("I can only multiply integers")
                    }
                },
                "/" => {
                    match stack.pop().unwrap() {
                        StackElement::Integer(x) => {
                            match stack.pop().unwrap() {
                                StackElement::Integer(y) => stack.push(StackElement::Integer(x / y)),
                                _ => panic!("I can only divide integers")
                                
                            }
                        }
                        _ => panic!("I can only divide integers")
                    }
                },
                "clear" => stack.clear(),
                "dup" => {
                    let a = stack.pop().unwrap();
                    stack.push(a.clone());
                    stack.push(a);
                },
                "swap" => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(a.clone());
                    stack.push(b.clone());
                },
                "drop" => {
                    stack.pop().unwrap();
                },
                "rot" => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    let c = stack.pop().unwrap();

                    stack.push(b);
                    stack.push(a);
                    stack.push(c);
                }
                "call" => {
                    match stack.pop().unwrap() {
                        StackElement::SubStack(s) => s.iter().for_each(|i| call_stack(stack, i)),
                        _ => {}
                    }
                }
                _ => panic!("Invalid input"),
            }
        }
        StackElement::Integer(n) => stack.push(StackElement::Integer(*n)),
        StackElement::SubStack(s) => stack.push(StackElement::SubStack(s.clone())),
    }
    
}