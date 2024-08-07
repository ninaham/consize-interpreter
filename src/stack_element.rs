#![allow(clippy::match_like_matches_macro)]
use std::{
    collections::BTreeMap,
    fmt::{Debug, Display},
    io::Error,
    ops::Deref,
    rc::Rc,
};

use crate::interpreter::Interpreter;

pub type BuiltIn = Rc<dyn Fn(Interpreter) -> Interpreter>;

#[derive(Clone, Debug)]
pub enum StackElement {
    SubStack(Vec<StackElement>),
    Word(String),
    Map(Vec<(StackElement, StackElement)>),
    Fun(Rc<Funct>),
    Nil,
}

pub enum Funct {
    BuiltIn(BuiltIn),
    SelfDefined(StackElement),
}

impl Debug for Funct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BuiltIn(_arg0) => f.debug_tuple("BuiltIn").finish(),
            Self::SelfDefined(arg0) => f.debug_tuple("SelfDefined").field(arg0).finish(),
        }
    }
}

impl Display for StackElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SubStack(st) => write!(f, "{}", print_stack(st, true, true)),
            Self::Word(s) => write!(f, "{s}"),
            Self::Map(m) => write!(f, "{}", print_map(m)),
            Self::Nil => write!(f, "nil"),
            Self::Fun(fct) => match fct.deref() {
                Funct::BuiltIn(bi) => write!(f, "{:p}", bi),
                Funct::SelfDefined(bi) => write!(f, "{}", bi),
            },
        }
    }
}

pub fn print_map(map: &Vec<(StackElement, StackElement)>) -> String {
    let mut str = String::new();
    str.push_str("{ ");
    for i in map {
        str.push_str(format!("{}, {} ", i.0, i.1).as_str());
    }
    str.push('}');

    str
}

impl PartialEq for StackElement {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::SubStack(s) => match other {
                Self::SubStack(sk) => sk == s,
                _ => false,
            },
            Self::Word(w) => match other {
                Self::Word(wk) => w == wk,
                _ => false,
            },
            Self::Map(m) => match other {
                Self::Map(mk) => m == mk,
                _ => false,
            },
            Self::Fun(f) => match other {
                Self::Fun(fk) => match f.deref() {
                    Funct::BuiltIn(bi) => match fk.deref() {
                        Funct::BuiltIn(bik) => std::ptr::eq(bi, bik),
                        Funct::SelfDefined(_) => false,
                    },
                    Funct::SelfDefined(sd) => match fk.deref() {
                        Funct::BuiltIn(_) => false,
                        Funct::SelfDefined(sdk) => sd == sdk,
                    },
                },
                _ => false,
            },
            Self::Nil => match other {
                StackElement::Nil => true,
                _ => false,
            },
        }
    }
}

pub fn map_to_dict(
    map: &Vec<(StackElement, StackElement)>,
) -> Result<BTreeMap<String, Rc<Funct>>, Error> {
    let mut dict = BTreeMap::new();
    for tuple in map {
        if let StackElement::Word(w) = tuple.0.to_owned() {
            match tuple.1.to_owned() {
                StackElement::Fun(f) => {
                    dict.insert(w, f);
                }
                other => {
                    dict.insert(w, Rc::new(Funct::SelfDefined(other)));
                }
            }
        } else {
            panic!("need word as key for dict entry");
        }
    }

    Ok(dict)
}

pub fn print_stack(stack: &Vec<StackElement>, print_brackets: bool, reverse: bool) -> String {
    let mut str = String::new();
    if print_brackets && !reverse {
        str.push_str("[ ");
    }
    for i in stack {
        if reverse {
            str = i.to_string() + " " + str.as_str();
        } else {
            str.push_str(format!("{} ", *i).as_str());
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
