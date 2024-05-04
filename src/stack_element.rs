use std::{collections::BTreeMap, fmt::Display};

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum StackElement {
    SubStack(Vec<StackElement>),
    Word(String),
    Map(BTreeMap<StackElement, StackElement>),
    Nil,
}

impl Display for StackElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackElement::SubStack(st) => write!(f, "{} ", print_stack(st, true, true)),
            StackElement::Word(s) => write!(f, "{} ", s),
            StackElement::Map(m) => write!(f, "{} ", print_map(m)),
            StackElement::Nil => write!(f, "nil"),
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

pub fn print_stack(stack: &Vec<StackElement>, print_brackets: bool, reverse: bool) -> String {
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
