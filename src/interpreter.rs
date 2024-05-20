use std::{
    collections::BTreeMap,
    env,
    fs::{self, OpenOptions},
    io::{self, stdin, stdout, Error, Write},
    ops::Deref,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use colored::Colorize;

use crate::stack_element::{map_to_dict, Funct, StackElement};

#[derive(Clone)]
pub struct Interpreter {
    pub datastack: Vec<StackElement>,
    pub callstack: Vec<StackElement>,
    pub dictionary: BTreeMap<String, Rc<Funct>>,
    pub count: usize,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            datastack: Vec::new(),
            callstack: Vec::new(),
            dictionary: Self::init_dictionary(),
            count: 0,
        }
    }

    pub fn init_dictionary() -> BTreeMap<String, Rc<Funct>> {
        let mut dict: BTreeMap<String, Rc<Funct>> = BTreeMap::new();

        dict.insert(
            "swap".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::swap))),
        );
        dict.insert(
            "dup".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::dup))),
        );
        dict.insert(
            "drop".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::drop))),
        );
        dict.insert(
            "rot".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::rot))),
        );
        dict.insert(
            "type".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::r#type))),
        );
        dict.insert(
            "equal?".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::equal))),
        );
        dict.insert(
            "identical?".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::identical))),
        );
        dict.insert(
            "emptystack".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::emptystack))),
        );
        dict.insert(
            "push".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::push))),
        );
        dict.insert(
            "top".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::top))),
        );
        dict.insert(
            "pop".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::pop))),
        );
        dict.insert(
            "concat".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::concat))),
        );
        dict.insert(
            "reverse".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::reverse))),
        );
        dict.insert(
            "mapping".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::mapping))),
        );
        dict.insert(
            "unmap".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::unmap))),
        );
        dict.insert(
            "keys".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::keys))),
        );
        dict.insert(
            "assoc".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::assoc))),
        );
        dict.insert(
            "dissoc".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::dissoc))),
        );
        dict.insert(
            "get".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::get))),
        );
        dict.insert(
            "merge".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::merge))),
        );
        dict.insert(
            "word".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::word))),
        );
        dict.insert(
            "unword".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::unword))),
        );
        dict.insert(
            "char".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::char))),
        );
        dict.insert(
            "print".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::print))),
        );
        dict.insert(
            "flush".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::flush))),
        );
        dict.insert(
            "read-line".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::read_line))),
        );
        dict.insert(
            "slurp".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::slurp))),
        );
        dict.insert(
            "spit".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::spit))),
        );
        dict.insert(
            "spit-on".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::spit_on))),
        );
        dict.insert(
            "uncomment".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::uncomment))),
        );
        dict.insert(
            "tokenize".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::tokenize))),
        );
        dict.insert(
            "undocument".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::undocument))),
        ); //TODO
        dict.insert(
            "current-time-millis".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::current_time_millis))),
        );
        dict.insert(
            "operating-system".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::operating_system))),
        );
        dict.insert(
            "call".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::call))),
        );
        dict.insert(
            "call/cc".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::call_cc))),
        );
        dict.insert(
            "continue".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::r#continue))),
        );
        dict.insert(
            "get-dict".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::get_dict))),
        );
        dict.insert(
            "set-dict".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::set_dict))),
        );
        dict.insert(
            "stepcc".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::stepcc))),
        );
        dict.insert(
            "apply".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::apply))),
        );
        dict.insert(
            "compose".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::compose))),
        ); //TODO
        dict.insert(
            "func".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::func))),
        );
        dict.insert(
            "integer?".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::integer))),
        );
        dict.insert(
            "+".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::addition))),
        );
        dict.insert(
            "-".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::subtraction))),
        );
        dict.insert(
            "*".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::multiplication))),
        );
        dict.insert(
            "div".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::division))),
        );
        dict.insert(
            "mod".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::modulo))),
        );
        dict.insert(
            "<".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::less_than))),
        );
        dict.insert(
            ">".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::greater_than))),
        );
        dict.insert(
            "==".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::equals))),
        );
        dict.insert(
            "<=".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::less_equals))),
        );
        dict.insert(
            ">=".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::greater_equals))),
        );
        dict.insert(
            "\\".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::comment))),
        );
        dict.insert(
            "load".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::load))),
        );
        dict.insert(
            "run".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::run))),
        );
        dict.insert(
            "start".to_string(),
            Rc::new(Funct::BuiltIn(Rc::new(Self::start))),
        );

        dict
    }

    pub fn dup(mut self) -> Result<Box<Self>, Error> {
        let a = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))?;
        self.datastack.push(a.clone());
        self.datastack.push(a);
        Ok(Box::new(Self {
            datastack: self.datastack,
            callstack: self.callstack,
            dictionary: self.dictionary,
            count: self.count,
        }))
    }

    pub fn swap(mut self) -> Result<Box<Self>, Error> {
        let a = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))?;
        let b = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))?;
        self.datastack.push(a);
        self.datastack.push(b);

        Ok(Box::new(Self {
            datastack: self.datastack,
            callstack: self.callstack,
            dictionary: self.dictionary,
            count: self.count,
        }))
    }

    pub fn drop(mut self) -> Result<Box<Self>, Error> {
        self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))?;

        Ok(Box::new(Self {
            datastack: self.datastack,
            callstack: self.callstack,
            dictionary: self.dictionary,
            count: self.count,
        }))
    }

    pub fn rot(mut self) -> Result<Box<Self>, Error> {
        let a = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))?;
        let b = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))?;
        let c = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))?;

        self.datastack.push(b);
        self.datastack.push(a);
        self.datastack.push(c);

        Ok(Box::new(Self {
            datastack: self.datastack,
            callstack: self.callstack,
            dictionary: self.dictionary,
            count: self.count,
        }))
    }

    pub fn emptystack(mut self) -> Result<Box<Self>, Error> {
        self.datastack.push(StackElement::SubStack(Vec::new()));
        Ok(Box::new(Self {
            datastack: self.datastack,
            callstack: self.callstack,
            dictionary: self.dictionary,
            count: self.count,
        }))
    }

    pub fn push(mut self) -> Result<Box<Self>, Error> {
        let e = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))?;

        if let StackElement::SubStack(mut ss) = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            ss.push(e);
            self.datastack.push(StackElement::SubStack(ss));
            return Ok(Box::new(Self {
                datastack: self.datastack,
                callstack: self.callstack,
                dictionary: self.dictionary,
                count: self.count,
            }));
        }
        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} I can only push to stacks", "Error:".red().bold()),
        ))
    }

    pub fn r#type(mut self) -> Result<Box<Self>, Error> {
        match self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            StackElement::SubStack(_) => self.datastack.push(StackElement::Word("stk".to_string())),
            StackElement::Word(_) => self.datastack.push(StackElement::Word("wrd".to_string())),
            StackElement::Map(_) => self.datastack.push(StackElement::Word("map".to_string())),
            StackElement::Nil => self.datastack.push(StackElement::Word("nil".to_string())),
            StackElement::Fun(_) => self.datastack.push(StackElement::Word("fct".to_string())),
        };

        Ok(Box::new(Self {
            datastack: self.datastack,
            callstack: self.callstack,
            dictionary: self.dictionary,
            count: self.count,
        }))
    }

    pub fn equal(mut self) -> Result<Box<Self>, Error> {
        let a = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))?;

        let b = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))?;

        if a == b {
            self.datastack.push(StackElement::Word("t".to_string()));
        } else {
            self.datastack.push(StackElement::Word("f".to_string()));
        }

        Ok(Box::new(Interpreter {
            datastack: self.datastack,
            callstack: self.callstack,
            dictionary: self.dictionary,
            count: self.count,
        }))
    }

    pub fn identical(self) -> Result<Box<Self>, Error> {
        unimplemented!()
    }

    pub fn pop(mut self) -> Result<Box<Self>, Error> {
        let substack = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))?;

        match substack {
            StackElement::SubStack(mut ss) => {
                ss.pop().unwrap_or(StackElement::Nil);
                self.datastack.push(StackElement::SubStack(ss));
                Ok(Box::new(Self {
                    datastack: self.datastack,
                    callstack: self.callstack,
                    dictionary: self.dictionary,
                    count: self.count,
                }))
            }
            _ => Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} I can only pop from stacks", "Error:".red().bold()),
            )),
        }
    }

    pub fn top(mut self) -> Result<Box<Self>, Error> {
        let substack = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))?;
        match substack {
            StackElement::SubStack(mut ss) => {
                let el = ss.pop().unwrap_or(StackElement::Nil);
                self.datastack.push(el);
                Ok(Box::new(Self {
                    datastack: self.datastack,
                    callstack: self.callstack,
                    dictionary: self.dictionary,
                    count: self.count,
                }))
            }
            _ => Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} I can only pop from stacks", "Error:".red().bold()),
            )),
        }
    }

    pub fn concat(mut self) -> Result<Box<Self>, Error> {
        if let StackElement::SubStack(mut ss1) = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            if let StackElement::SubStack(mut ss2) = self.datastack.pop().ok_or(Error::new(
                io::ErrorKind::InvalidData,
                format!("{} not enough operands", "Error:".red().bold()),
            ))? {
                ss1.append(&mut ss2);
                self.datastack.push(StackElement::SubStack(ss1))
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

        Ok(Box::new(Self {
            datastack: self.datastack,
            callstack: self.callstack,
            dictionary: self.dictionary,
            count: self.count,
        }))
    }

    pub fn reverse(mut self) -> Result<Box<Self>, Error> {
        if let StackElement::SubStack(ss) = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            let ss_rev = ss.iter().rev().cloned().collect();
            self.datastack.push(StackElement::SubStack(ss_rev))
        } else {
            return Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} I can only reverse stacks", "Error:".red().bold()),
            ));
        }
        Ok(Box::new(Self {
            datastack: self.datastack,
            callstack: self.callstack,
            dictionary: self.dictionary,
            count: self.count,
        }))
    }

    pub fn mapping(mut self) -> Result<Box<Self>, Error> {
        if let StackElement::SubStack(ss) = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
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

            let mut map = Vec::new();

            for i in 0..keys.len() {
                map.push((keys[i].clone(), values[i].clone()));
            }

            self.datastack.push(StackElement::Map(map));
            return Ok(Box::new(Self {
                datastack: self.datastack,
                callstack: self.callstack,
                dictionary: self.dictionary,
                count: self.count,
            }));
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need map to create mapping", "Error:".red().bold()),
        ))
    }

    pub fn unmap(mut self) -> Result<Box<Self>, Error> {
        if let StackElement::Map(map) = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            let keys: Vec<StackElement> = map.iter().map(|(i, _)| i).cloned().collect();
            let values: Vec<StackElement> = map.iter().map(|(_, i)| i).cloned().collect();
            let mut st = Vec::new();
            for i in 0..keys.len() {
                st.push(values[i].clone());
                st.push(keys[i].clone());
            }

            self.datastack.push(StackElement::SubStack(st));

            return Ok(Box::new(Self {
                datastack: self.datastack,
                callstack: self.callstack,
                dictionary: self.dictionary,
                count: self.count,
            }));
        }
        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need map to unmap", "Error:".red().bold()),
        ))
    }

    pub fn keys(mut self) -> Result<Box<Self>, Error> {
        if let StackElement::Map(map) = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            let keys = map
                .iter()
                .map(|(i, _)| i)
                .cloned()
                .collect::<Vec<StackElement>>();
            self.datastack.push(StackElement::SubStack(keys));

            return Ok(Box::new(Self {
                datastack: self.datastack,
                callstack: self.callstack,
                dictionary: self.dictionary,
                count: self.count,
            }));
        }
        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need map to list keys", "Error:".red().bold()),
        ))
    }

    pub fn assoc(mut self) -> Result<Box<Self>, Error> {
        if let StackElement::Map(mut map) = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            let key = self.datastack.pop().ok_or(Error::new(
                io::ErrorKind::InvalidData,
                format!("{} not enough operands", "Error:".red().bold()),
            ))?;
            let value = self.datastack.pop().ok_or(Error::new(
                io::ErrorKind::InvalidData,
                format!("{} not enough operands", "Error:".red().bold()),
            ))?;

            if !map.iter().any(|(i, _)| *i == key) {
                map.push((key, value));
            }

            self.datastack.push(StackElement::Map(map));

            return Ok(Box::new(Self {
                datastack: self.datastack,
                callstack: self.callstack,
                dictionary: self.dictionary,
                count: self.count,
            }));
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need map to assoc value to map", "Error:".red().bold()),
        ))
    }

    pub fn dissoc(mut self) -> Result<Box<Self>, Error> {
        if let StackElement::Map(mut map) = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            let key = self.datastack.pop().ok_or(Error::new(
                io::ErrorKind::InvalidData,
                format!("{} not enough operands", "Error:".red().bold()),
            ))?;
            map.retain(|(i, _)| *i != key);

            self.datastack.push(StackElement::Map(map));

            return Ok(Box::new(Self {
                datastack: self.datastack,
                callstack: self.callstack,
                dictionary: self.dictionary,
                count: self.count,
            }));
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "{} need map to dissoc value from map",
                "Error:".red().bold()
            ),
        ))
    }

    pub fn get(mut self) -> Result<Box<Self>, Error> {
        let default = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))?;
        if let StackElement::Map(m) = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            let key = self.datastack.pop().ok_or(Error::new(
                io::ErrorKind::InvalidData,
                format!("{} not enough operands", "Error:".red().bold()),
            ))?;

            self.datastack
                .push(match m.iter().find(|(i, _)| *i == key) {
                    Some((_, j)) => match j {
                        StackElement::Fun(func) => match func.deref() {
                            Funct::BuiltIn(_) => j.clone(),
                            Funct::SelfDefined(st) => *st.clone(),
                        },
                        _ => j.clone(),
                    },
                    None => default,
                });

            return Ok(Box::new(Self {
                datastack: self.datastack,
                callstack: self.callstack,
                dictionary: self.dictionary,
                count: self.count,
            }));
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need map to get value from map", "Error:".red().bold()),
        ))
    }

    pub fn merge(mut self) -> Result<Box<Self>, Error> {
        if let StackElement::Map(mut m1) = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            if let StackElement::Map(m2) = self.datastack.pop().ok_or(Error::new(
                io::ErrorKind::InvalidData,
                format!("{} not enough operands", "Error:".red().bold()),
            ))? {
                m2.iter().for_each(|(k, v)| {
                    if !m1.iter().any(|(i, _)| *i == *k) {
                        m1.push((k.clone(), v.clone()));
                    }
                });

                self.datastack.push(StackElement::Map(m1));

                return Ok(Box::new(Self {
                    datastack: self.datastack,
                    callstack: self.callstack,
                    dictionary: self.dictionary,
                    count: self.count,
                }));
            }
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need maps to merge maps", "Error:".red().bold()),
        ))
    }

    pub fn word(mut self) -> Result<Box<Self>, Error> {
        if let StackElement::SubStack(st) = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            if let Ok(s) = st
                .iter()
                .map(|e| {
                    if let StackElement::Word(str) = e {
                        Ok(str.as_str())
                    } else {
                        Err(Error::new(
                            io::ErrorKind::InvalidInput,
                            format!("{} stack may only contain words", "Error:".red().bold()),
                        ))
                    }
                })
                .rev()
                .collect::<Result<String, Error>>()
            {
                self.datastack.push(StackElement::Word(s));
                return Ok(Box::new(Self {
                    datastack: self.datastack,
                    callstack: self.callstack,
                    dictionary: self.dictionary,
                    count: self.count,
                }));
            }
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need stack to use 'word'", "Error:".red().bold()),
        ))
    }

    pub fn unword(mut self) -> Result<Box<Self>, Error> {
        if let StackElement::Word(str) = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            self.datastack.push(StackElement::SubStack(
                str.chars()
                    .map(|c| StackElement::Word(c.to_string()))
                    .rev()
                    .collect(),
            ));

            return Ok(Box::new(Self {
                datastack: self.datastack,
                callstack: self.callstack,
                dictionary: self.dictionary,
                count: self.count,
            }));
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need word to use 'unword'", "Error:".red().bold()),
        ))
    }

    pub fn char(mut self) -> Result<Box<Self>, Error> {
        if let StackElement::Word(w) = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            self.datastack.push(StackElement::Word(match w.as_str() {
                "\\space" => " ".to_string(),
                "\\newline" => "\n".to_string(),
                "\\formfeed" => '\x0c'.to_string(),
                "\\return" => "\r".to_string(),
                "\\backspace" => '\x08'.to_string(),
                "\\tab" => "\t".to_string(),
                s => {
                    if s.starts_with("\\u") {
                        let ss: String = s.chars().skip(2).collect();
                        char::from_u32(u32::from_str_radix(&ss, 16).map_err(|_| {
                            Error::new(
                                io::ErrorKind::InvalidData,
                                format!("{} invalid utf string", "Error:".red().bold()),
                            )
                        })?)
                        .ok_or(Error::new(
                            io::ErrorKind::InvalidData,
                            format!("{} invalid utf string", "Error:".red().bold()),
                        ))?
                        .to_string()
                    } else {
                        return Err(Error::new(
                            io::ErrorKind::InvalidData,
                            format!("{} invalid utf string", "Error:".red().bold()),
                        ));
                    }
                }
            }));

            return Ok(Box::new(Self {
                datastack: self.datastack,
                callstack: self.callstack,
                dictionary: self.dictionary,
                count: self.count,
            }));
        }
        Err(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} need word to parse to utf", "Error:".red().bold()),
        ))
    }

    pub fn print(mut self) -> Result<Box<Self>, Error> {
        print!(
            "{}",
            self.datastack.pop().ok_or(Error::new(
                io::ErrorKind::InvalidData,
                format!("{} not enough operands", "Error:".red().bold()),
            ))?
        );

        Ok(Box::new(Self {
            datastack: self.datastack,
            callstack: self.callstack,
            dictionary: self.dictionary,
            count: self.count,
        }))
    }

    pub fn flush(self) -> Result<Box<Self>, Error> {
        stdout().flush().unwrap();

        Ok(Box::new(Self {
            datastack: self.datastack,
            callstack: self.callstack,
            dictionary: self.dictionary,
            count: self.count,
        }))
    }

    pub fn read_line(mut self) -> Result<Box<Self>, Error> {
        let mut inp = "".to_string();
        stdin().read_line(&mut inp).unwrap();

        self.datastack.push(StackElement::Word(inp));
        Ok(Box::new(Self {
            datastack: self.datastack,
            callstack: self.callstack,
            dictionary: self.dictionary,
            count: self.count,
        }))
    }

    pub fn slurp(mut self) -> Result<Box<Self>, Error> {
        if let StackElement::Word(src) = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            self.datastack
                .push(StackElement::Word(fs::read_to_string(src)?));
            return Ok(Box::new(Self {
                datastack: self.datastack,
                callstack: self.callstack,
                dictionary: self.dictionary,
                count: self.count,
            }));
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need word to read from file", "Error:".red().bold()),
        ))
    }

    pub fn spit(mut self) -> Result<Box<Self>, Error> {
        if let StackElement::Word(data) = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            if let StackElement::Word(file) = self.datastack.pop().ok_or(Error::new(
                io::ErrorKind::InvalidData,
                format!("{} not enough operands", "Error:".red().bold()),
            ))? {
                fs::write(file, data)?;
                return Ok(Box::new(Self {
                    datastack: self.datastack,
                    callstack: self.callstack,
                    dictionary: self.dictionary,
                    count: self.count,
                }));
            }
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} could not write file", "Error:".red().bold()),
        ))
    }

    pub fn spit_on(mut self) -> Result<Box<Self>, Error> {
        if let StackElement::Word(data) = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            if let StackElement::Word(path) = self.datastack.pop().ok_or(Error::new(
                io::ErrorKind::InvalidData,
                format!("{} not enough operands", "Error:".red().bold()),
            ))? {
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(path)
                    .unwrap();

                file.write_all(data.as_bytes()).unwrap();
                return Ok(Box::new(Self {
                    datastack: self.datastack,
                    callstack: self.callstack,
                    dictionary: self.dictionary,
                    count: self.count,
                }));
            }
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} could not write file", "Error:".red().bold()),
        ))
    }

    pub fn uncomment(mut self) -> Result<Box<Self>, Error> {
        if let StackElement::Word(wrd) = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            self.datastack.push(StackElement::Word(
                wrd.lines()
                    .map(|l| l.split('%').next().unwrap())
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join(" "),
            ));

            return Ok(Box::new(Self {
                datastack: self.datastack,
                callstack: self.callstack,
                dictionary: self.dictionary,
                count: self.count,
            }));
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} I can only uncomment words", "Error:".red().bold()),
        ))
    }

    pub fn tokenize(mut self) -> Result<Box<Self>, Error> {
        if let StackElement::Word(w) = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            self.datastack.push(StackElement::SubStack(
                w.split_whitespace()
                    .map(|s| StackElement::Word(s.to_string()))
                    .rev()
                    .collect(),
            ));

            return Ok(Box::new(Self {
                datastack: self.datastack,
                callstack: self.callstack,
                dictionary: self.dictionary,
                count: self.count,
            }));
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need word to tokenize", "Error:".red().bold()),
        ))
    }

    pub fn undocument(self) -> Result<Box<Self>, Error> {
        todo!()
    }

    pub fn current_time_millis(mut self) -> Result<Box<Self>, Error> {
        self.datastack.push(StackElement::Word(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis()
                .to_string(),
        ));

        Ok(Box::new(Self {
            datastack: self.datastack,
            callstack: self.callstack,
            dictionary: self.dictionary,
            count: self.count,
        }))
    }

    pub fn operating_system(mut self) -> Result<Box<Self>, Error> {
        self.datastack
            .push(StackElement::Word(env::consts::OS.to_string()));

        Ok(Box::new(Self {
            datastack: self.datastack,
            callstack: self.callstack,
            dictionary: self.dictionary,
            count: self.count,
        }))
    }

    pub fn call(mut self) -> Result<Box<Self>, Error> {
        match self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            StackElement::SubStack(mut st) => self.callstack.append(&mut st),
            StackElement::Word(w) => self.callstack.push(StackElement::Word(w)),
            _ => panic!("bitte passier nicht"),
        }

        Ok(Box::new(Self {
            datastack: self.datastack,
            callstack: self.callstack,
            dictionary: self.dictionary,
            count: self.count,
        }))
    }

    pub fn call_cc(mut self) -> Result<Box<Self>, Error> {
        let top = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))?;
        let mut new_callstack = Vec::new();
        match top.clone() {
            StackElement::SubStack(mut ss) => new_callstack.append(&mut ss),
            el => new_callstack.push(el),
        }

        self.datastack = [
            StackElement::SubStack(self.datastack),
            StackElement::SubStack(self.callstack),
        ]
        .to_vec();

        Ok(Box::new(Self {
            datastack: self.datastack,
            callstack: new_callstack,
            dictionary: self.dictionary,
            count: self.count,
        }))
    }

    pub fn r#continue(mut self) -> Result<Box<Self>, Error> {
        if let StackElement::SubStack(new_callstack) = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            if let StackElement::SubStack(new_datastack) =
                self.datastack.pop().ok_or(Error::new(
                    io::ErrorKind::InvalidData,
                    format!("{} not enough operands", "Error:".red().bold()),
                ))?
            {
                return Ok(Box::new(Self {
                    datastack: new_datastack,
                    callstack: new_callstack,
                    dictionary: self.dictionary,
                    count: self.count,
                }));
            }
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need quotation for continue", "Error:".red().bold()),
        ))
    }

    pub fn get_dict(mut self) -> Result<Box<Self>, Error> {
        let mut map = Vec::new();
        self.dictionary.iter().for_each(|(k, v)| {
            let key = StackElement::Word(k.clone());
            let value = StackElement::Fun(v.clone());
            if !map.iter().map(|(i, _)| i).any(|se| key == *se) {
                map.push((key, value));
            }
        });

        self.datastack.push(StackElement::Map(map));

        Ok(Box::new(Self {
            datastack: self.datastack,
            callstack: self.callstack,
            dictionary: self.dictionary,
            count: self.count,
        }))
    }

    pub fn set_dict(mut self) -> Result<Box<Self>, Error> {
        if let StackElement::Map(dict) = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            let dict = map_to_dict(&dict)?;
            return Ok(Box::new(Self {
                datastack: self.datastack,
                callstack: self.callstack,
                dictionary: dict,
                count: self.count,
            }));
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need map for set-dict", "Error:".red().bold()),
        ))
    }

    pub fn stepcc(mut self) -> Result<Box<Self>, Error> {
        let mut count = self.count;
        let e = self.callstack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))?;

        count += 1;

        /*if count > 15231 {
            println!("{count}: {e}");
            println!("datastack: {}", print_stack(&self.datastack, true, true));
            println!("callstack: {}", print_stack(&self.callstack, true, true));
        }*/
        match e {
            StackElement::SubStack(ss) => self.datastack.push(StackElement::SubStack(ss)),
            StackElement::Word(w) => {
                match self.dictionary.get(&w) {
                    Some(fun) => match fun.deref() {
                        Funct::BuiltIn(fct) => {
                            return fct(Interpreter {
                                datastack: self.datastack,
                                callstack: self.callstack,
                                dictionary: self.dictionary.clone(),
                                count,
                            });
                        }
                        Funct::SelfDefined(stack) => match *stack.clone() {
                            StackElement::SubStack(mut ss) => {
                                self.callstack.append(&mut ss);
                            }
                            _ => panic!("bitte bitte trete nicht ein"),
                        },
                    },
                    None => {
                        self.datastack.push(StackElement::Word(w));
                        self.callstack
                            .push(StackElement::Word("read-word".to_string()))
                    }
                };
            }
            StackElement::Map(m) => {
                self.datastack.push(StackElement::Map(m));
                self.callstack
                    .push(StackElement::Word("read-mapping".to_string()))
            }
            StackElement::Nil => self.datastack.push(StackElement::Nil),
            StackElement::Fun(f) => match f.deref() {
                Funct::BuiltIn(bi) => {
                    return bi(Interpreter {
                        datastack: self.datastack,
                        callstack: self.callstack,
                        dictionary: self.dictionary,
                        count,
                    });
                }
                Funct::SelfDefined(sd) => match *sd.clone() {
                    StackElement::SubStack(ss) => self.callstack.append(&mut ss.clone()),
                    _ => todo!("bitte bitte trete niemals ein"),
                },
            },
        };

        Ok(Box::new(Self {
            datastack: self.datastack,
            callstack: self.callstack,
            dictionary: self.dictionary,
            count,
        }))
    }

    pub fn apply(mut self) -> Result<Box<Self>, Error> {
        if let StackElement::Fun(fun) = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            if let StackElement::SubStack(stack) = self.datastack.pop().ok_or(Error::new(
                io::ErrorKind::InvalidData,
                format!("{} not enough operands", "Error:".red().bold()),
            ))? {
                self.datastack
                    .push(StackElement::SubStack(match fun.deref() {
                        Funct::BuiltIn(bi) => {
                            bi(Interpreter {
                                datastack: stack,
                                callstack: self.callstack.clone(),
                                dictionary: self.dictionary.clone(),
                                count: self.count,
                            })?
                            .datastack
                        }
                        Funct::SelfDefined(_) => todo!(),
                    }))
            }
        }

        Ok(Box::new(Self {
            datastack: self.datastack,
            callstack: self.callstack,
            dictionary: self.dictionary,
            count: self.count,
        }))
    }

    pub fn compose(self) -> Result<Box<Self>, Error> {
        todo!()
    }

    pub fn func(mut self) -> Result<Box<Self>, Error> {
        let callstack = self.callstack.clone();
        let dictionary = self.dictionary.clone();
        if let StackElement::Map(dict) = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            let dict = map_to_dict(&dict)?;
            if let StackElement::SubStack(qt) = self.datastack.pop().unwrap() {
                fn runcc(inner: Interpreter) -> Result<Vec<StackElement>, Error> {
                    let mut int = inner;
                    while !int.callstack.is_empty() {
                        int = match int.stepcc() {
                            Ok(i) => *i,
                            Err(e) => return Err(e),
                        };
                    }

                    Ok(int.datastack)
                }

                let f = move |interpreter: Interpreter| {
                    let qt = qt.clone().to_owned();
                    Ok(Box::new(Interpreter {
                        datastack: runcc(Interpreter {
                            datastack: interpreter.datastack,
                            callstack: qt,
                            dictionary: dict.clone(),
                            count: self.count,
                        })?,
                        callstack: Vec::new(),
                        dictionary: BTreeMap::new(),
                        count: self.count,
                    }))
                };

                self.datastack
                    .push(StackElement::Fun(Rc::new(Funct::BuiltIn(Rc::new(f)))));
            }
        }

        Ok(Box::new(Self {
            datastack: self.datastack,
            callstack,
            dictionary,
            count: self.count,
        }))
    }

    pub fn integer(mut self) -> Result<Box<Self>, Error> {
        let e = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))?;
        self.datastack.push(match e {
            StackElement::Word(w) => match w.parse::<usize>() {
                Ok(_) => StackElement::Word("t".to_string()),
                Err(_) => StackElement::Word("f".to_string()),
            },
            _ => StackElement::Word("f".to_string()),
        });

        Ok(Box::new(Self {
            datastack: self.datastack,
            callstack: self.callstack,
            dictionary: self.dictionary,
            count: self.count,
        }))
    }

    pub fn addition(self) -> Result<Box<Self>, Error> {
        self.binary("+")
    }

    pub fn subtraction(self) -> Result<Box<Self>, Error> {
        self.binary("-")
    }

    pub fn multiplication(self) -> Result<Box<Self>, Error> {
        self.binary("*")
    }

    pub fn division(self) -> Result<Box<Self>, Error> {
        self.binary("div")
    }

    pub fn modulo(self) -> Result<Box<Self>, Error> {
        self.binary("mod")
    }

    pub fn greater_than(self) -> Result<Box<Self>, Error> {
        self.binary(">")
    }

    pub fn less_than(self) -> Result<Box<Self>, Error> {
        self.binary("<")
    }

    pub fn equals(self) -> Result<Box<Self>, Error> {
        self.binary("==")
    }

    pub fn less_equals(self) -> Result<Box<Self>, Error> {
        self.binary("<=")
    }

    pub fn greater_equals(self) -> Result<Box<Self>, Error> {
        self.binary(">=")
    }

    fn binary(mut self, op: &str) -> Result<Box<Self>, Error> {
        if let StackElement::Word(w1) = self.datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            if let StackElement::Word(w2) = self.datastack.pop().ok_or(Error::new(
                io::ErrorKind::InvalidData,
                format!("{} not enough operands", "Error:".red().bold()),
            ))? {
                let x: isize = w1.parse().map_err(|_| {
                    Error::new(
                        io::ErrorKind::InvalidData,
                        format!("{} {} is not an integer", "Error:".red().bold(), w1,),
                    )
                })?;
                let y: isize = w2.parse().map_err(|_| {
                    Error::new(
                        io::ErrorKind::InvalidData,
                        format!("{} {} is not an integer", "Error:".red().bold(), w2),
                    )
                })?;
                self.datastack.push(StackElement::Word(match op {
                    "+" => (x + y).to_string(),
                    "-" => (y - x).to_string(),
                    "*" => (x * y).to_string(),
                    "div" => (y / x).to_string(),
                    "mod" => (y % x).to_string(),
                    ">=" => match y >= x {
                        true => "t".to_string(),
                        false => "f".to_string(),
                    },
                    "==" => match y == x {
                        true => "t".to_string(),
                        false => "f".to_string(),
                    },
                    "<" => match y < x {
                        true => "t".to_string(),
                        false => "f".to_string(),
                    },
                    ">" => match y > x {
                        true => "t".to_string(),
                        false => "f".to_string(),
                    },
                    "<=" => match y <= x {
                        true => "t".to_string(),
                        false => "f".to_string(),
                    },
                    _ => panic!("unknown operator"),
                }));
                return Ok(Box::new(Self {
                    datastack: self.datastack,
                    callstack: self.callstack,
                    dictionary: self.dictionary,
                    count: self.count,
                }));
            }
        }
        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "{} need integers for arithmetic and logic operations",
                "Error:".red().bold()
            ),
        ))
    }

    pub fn comment(mut self) -> Result<Box<Self>, Error> {
        self.datastack
            .push(StackElement::Fun(Rc::new(Funct::SelfDefined(Box::new(
                StackElement::SubStack(
                    [
                        StackElement::Word("continue".to_string()),
                        StackElement::Word("pop".to_string()),
                        StackElement::Word("swap".to_string()),
                        StackElement::Word("push".to_string()),
                        StackElement::Word("swap".to_string()),
                        StackElement::Word("rot".to_string()),
                        StackElement::Word("top".to_string()),
                        StackElement::Word("dup".to_string()),
                    ]
                    .to_vec(),
                ),
            )))));
        self.callstack
            .push(StackElement::Word("call/cc".to_string()));

        Ok(Box::new(Self {
            datastack: self.datastack,
            callstack: self.callstack,
            dictionary: self.dictionary,
            count: self.count,
        }))
    }

    pub fn load(mut self) -> Result<Box<Self>, Error> {
        self.callstack.append(
            &mut [
                StackElement::Word("tokenize".to_string()),
                StackElement::Word("uncomment".to_string()),
                StackElement::Word("slurp".to_string()),
            ]
            .to_vec(),
        );

        Ok(Box::new(Self {
            datastack: self.datastack,
            callstack: self.callstack,
            dictionary: self.dictionary,
            count: self.count,
        }))
    }

    pub fn run(mut self) -> Result<Box<Self>, Error> {
        self.callstack.append(
            &mut [
                StackElement::Word("call".to_string()),
                StackElement::Word("load".to_string()),
            ]
            .to_vec(),
        );

        Ok(Box::new(Self {
            datastack: self.datastack,
            callstack: self.callstack,
            dictionary: self.dictionary,
            count: self.count,
        }))
    }

    pub fn start(mut self) -> Result<Box<Self>, Error> {
        self.callstack.append(
            &mut [
                StackElement::Word("slurp".to_string()),
                StackElement::Word("uncomment".to_string()),
                StackElement::Word("tokenize".to_string()),
                StackElement::Word("get-dict".to_string()),
                StackElement::Word("func".to_string()),
                StackElement::Word("emptystack".to_string()),
                StackElement::Word("swap".to_string()),
                StackElement::Word("apply".to_string()),
            ]
            .to_vec(),
        );

        Ok(Box::new(Self {
            datastack: self.datastack,
            callstack: self.callstack,
            dictionary: self.dictionary,
            count: self.count,
        }))
    }
}
