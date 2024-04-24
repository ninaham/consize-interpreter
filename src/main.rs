use std::{
    collections::{BTreeMap, VecDeque},
    fmt::Display,
    io::{self, stdin, stdout, Error, Write},
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
            .for_each(|i| call(&mut stack, i).unwrap_or_else(|e| eprintln!("{e}")));

        println!("{}\n", print_stack(&stack, false, false));
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
enum StackElement {
    SubStack(Vec<StackElement>),
    Word(String),
    Keyword(String),
    Map(BTreeMap<StackElement, StackElement>),
}

impl Display for StackElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackElement::SubStack(st) => write!(f, "{} ", print_stack(st, true, true)),
            StackElement::Word(s) => write!(f, "{} ", s),
            StackElement::Keyword(s) => write!(f, "{} ", s),
            StackElement::Map(m) => write!(f, "{} ", print_map(m)),
        }
    }
}

fn print_map(map: &BTreeMap<StackElement, StackElement>) -> String {
    let mut str = "".to_string();
    str.push_str("{ ");
    for i in map {
        str.push_str(format!("{}{}", *i.0, *i.1).as_str())
    }
    str.push('}');

    str
}

fn print_stack(stack: &Vec<StackElement>, print_brackets: bool, reverse: bool) -> String {
    let mut str = "".to_string();
    if print_brackets && !reverse {
        str.push_str("[ ");
    }
    for i in stack {
        if reverse {
            str = i.to_string() + str.as_str();
        } else {
            str.push_str(format!("{}", *i).as_str())
        }
    }
    if reverse {
        str = "[ ".to_string() + str.as_str();
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
                if let StackElement::Word(w1) = stack.pop().unwrap() {
                    if let StackElement::Word(w2) = stack.pop().unwrap() {
                        if w1.parse::<usize>().is_ok() && w2.parse::<usize>().is_ok() {
                            stack.push(StackElement::Word(
                                (w1.parse::<usize>().unwrap() + w2.parse::<usize>().unwrap())
                                    .to_string(),
                            ));
                            return Ok(());
                        }
                    }
                }

                Err(Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("{} I can only add Integers", "Error:".red().bold()),
                ))
            }
            "-" => {
                if stack.len() < 2 {
                    return Err(Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("{} not enough operands", "Error:".red().bold()),
                    ));
                }
                if let StackElement::Word(w1) = stack.pop().unwrap() {
                    if let StackElement::Word(w2) = stack.pop().unwrap() {
                        if w1.parse::<usize>().is_ok() && w2.parse::<usize>().is_ok() {
                            stack.push(StackElement::Word(
                                (w1.parse::<usize>().unwrap() - w2.parse::<usize>().unwrap())
                                    .to_string(),
                            ));
                            return Ok(());
                        }
                    }
                }

                Err(Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("{} I can only subtract Integers", "Error:".red().bold()),
                ))
            }
            "*" => {
                if stack.len() < 2 {
                    return Err(Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("{} not enough operands", "Error:".red().bold()),
                    ));
                }
                if let StackElement::Word(w1) = stack.pop().unwrap() {
                    if let StackElement::Word(w2) = stack.pop().unwrap() {
                        if w1.parse::<usize>().is_ok() && w2.parse::<usize>().is_ok() {
                            stack.push(StackElement::Word(
                                (w1.parse::<usize>().unwrap() * w2.parse::<usize>().unwrap())
                                    .to_string(),
                            ));
                            return Ok(());
                        }
                    }
                }
                Err(Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("{} I can only multiply Integers", "Error:".red().bold()),
                ))
            }
            "div" => {
                if stack.len() < 2 {
                    return Err(Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("{} not enough operands", "Error:".red().bold()),
                    ));
                }
                if let StackElement::Word(w1) = stack.pop().unwrap() {
                    if let StackElement::Word(w2) = stack.pop().unwrap() {
                        if w1.parse::<usize>().is_ok() && w2.parse::<usize>().is_ok() {
                            stack.push(StackElement::Word(
                                (w1.parse::<usize>().unwrap() / w2.parse::<usize>().unwrap())
                                    .to_string(),
                            ));
                            return Ok(());
                        }
                    }
                }
                Err(Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("{} I can only divide Integers", "Error:".red().bold()),
                ))
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
                if stack.len() > 2 {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(a.clone());
                    stack.push(b.clone());
                }

                Err(Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("{} not enough operands", "Error:".red().bold()),
                ))
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

                if let StackElement::SubStack(ss) = stack.pop().unwrap() {
                    let mut new_substack = ss.clone();
                    new_substack.push(e);
                    stack.push(StackElement::SubStack(new_substack));
                    return Ok(());
                }
                Err(Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("{} I can only push to stacks", "Error:".red().bold()),
                ))
            }
            "type" => {
                match stack.pop().unwrap() {
                    StackElement::SubStack(_) => stack.push(StackElement::Word("stk".to_string())),
                    StackElement::Word(_) => stack.push(StackElement::Word("wrd".to_string())),
                    StackElement::Keyword(_) => stack.push(StackElement::Word("fct".to_string())),
                    StackElement::Map(_) => stack.push(StackElement::Word("map".to_string())),
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
            "mapping" => {
                if let StackElement::SubStack(ss) = stack.pop().unwrap() {
                    let keys = ss
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| i % 2 == 1)
                        .map(|(_, e)| e)
                        .collect::<Vec<&StackElement>>();

                    let values = ss
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| i % 2 == 0)
                        .map(|(_, e)| e)
                        .collect::<Vec<&StackElement>>();

                    if values.len() != keys.len() {
                        return Err(Error::new(
                            io::ErrorKind::InvalidInput,
                            format!("{} not enough values for every key", "Error:".red().bold()),
                        ));
                    }

                    let mut map = BTreeMap::new();

                    for i in 0..keys.len() {
                        map.insert(keys[i].clone(), values[i].clone());
                    }

                    stack.push(StackElement::Map(map));
                    return Ok(());
                }

                Err(Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("{} need map to create mapping", "Error:".red().bold()),
                ))
            }
            "unmap" => {
                if let StackElement::Map(map) = stack.pop().unwrap() {
                    let keys = map.keys().cloned().collect::<Vec<StackElement>>();
                    let values = map.values().cloned().collect::<Vec<StackElement>>();
                    let mut st = Vec::new();
                    for i in 0..keys.len() {
                        st.push(keys[i].clone());
                        st.push(values[i].clone());
                    }

                    stack.push(StackElement::SubStack(st));

                    return Ok(());
                }
                Err(Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("{} need map to unmap", "Error:".red().bold()),
                ))
            }
            "keys" => {
                if let StackElement::Map(map) = stack.pop().unwrap() {
                    let keys = map.keys().cloned().collect::<Vec<StackElement>>();
                    stack.push(StackElement::SubStack(keys));

                    return Ok(());
                }
                Err(Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("{} need map to list keys", "Error:".red().bold()),
                ))
            }
            "assoc" => {
                if let StackElement::Map(mut map) = stack.pop().unwrap() {
                    let key = stack.pop().unwrap();
                    let value = stack.pop().unwrap();
                    map.insert(key, value);

                    stack.push(StackElement::Map(map));

                    return Ok(());
                }

                Err(Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("{} need map to assoc value to map", "Error:".red().bold()),
                ))
            }
            "dissoc" => {
                if let StackElement::Map(mut map) = stack.pop().unwrap() {
                    let key = stack.pop().unwrap();
                    map.remove(&key);

                    stack.push(StackElement::Map(map))
                }

                Err(Error::new(
                    io::ErrorKind::InvalidInput,
                    format!(
                        "{} need map to dissoc value from map",
                        "Error:".red().bold()
                    ),
                ))
            }
            "get" => {
                let key = stack.pop().unwrap();
                if let StackElement::Map(m) = stack.pop().unwrap() {
                    let default = stack.pop().unwrap();

                    stack.push(m.clone().get(&key).unwrap_or(&default).clone());

                    return Ok(());
                }

                Err(Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("{} need map to get value from map", "Error:".red().bold()),
                ))
            }
            "merge" => {
                if let StackElement::Map(mut m1) = stack.pop().unwrap() {
                    if let StackElement::Map(m2) = stack.pop().unwrap() {
                        m2.iter().for_each(|(k, v)| {
                            m1.insert(k.clone(), v.clone()).unwrap();
                        });

                        stack.push(StackElement::Map(m1));

                        return Ok(());
                    }
                }

                Err(Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("{} need maps to merge maps", "Error:".red().bold()),
                ))
            }
            "word" => {
                if let StackElement::SubStack(st) = stack.pop().unwrap() {
                    if let Ok(s) = st
                        .iter()
                        .map(|e| {
                            if let StackElement::Word(str) = e {
                                Ok(str.as_str())
                            } else {
                                Err(Error::new(
                                    io::ErrorKind::InvalidInput,
                                    format!(
                                        "{} stack may only contain words",
                                        "Error:".red().bold()
                                    ),
                                ))
                            }
                        })
                        .rev()
                        .collect::<Result<String, Error>>()
                    {
                        stack.push(StackElement::Word(s));
                        return Ok(());
                    }
                }

                Err(Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("{} need stack to use 'word'", "Error:".red().bold()),
                ))
            }
            "unword" => {
                if let StackElement::Word(str) = stack.pop().unwrap() {
                    stack.push(StackElement::SubStack(
                        str.chars()
                            .map(|c| StackElement::Word(c.to_string()))
                            .collect(),
                    ))
                }

                Err(Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("{} need word to use 'unword'", "Error:".red().bold()),
                ))
            }
            "print" => {
                print!("{}", stack.pop().unwrap());

                Ok(())
            }
            "flush" => {
                stdout().flush().unwrap();

                Ok(())
            }
            "read-line" => {
                let mut inp = "".to_string();
                stdin().read_line(&mut inp).unwrap();

                stack.push(StackElement::Word(inp));
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
