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

    pub fn dup(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        let a = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))?;
        datastack.push(a.clone());
        datastack.push(a);
        Ok(Self {
            datastack,
            callstack: self.callstack.clone(),
            dictionary: self.dictionary.clone(),
            count: self.count,
        })
    }

    pub fn swap(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        let a = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))?;
        let b = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))?;
        datastack.push(a);
        datastack.push(b);

        Ok(Self {
            datastack,
            callstack: self.callstack.clone(),
            dictionary: self.dictionary.clone(),
            count: self.count,
        })
    }

    pub fn drop(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))?;

        Ok(Self {
            datastack,
            callstack: self.callstack.clone(),
            dictionary: self.dictionary.clone(),
            count: self.count,
        })
    }

    pub fn rot(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();

        let a = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))?;
        let b = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))?;
        let c = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))?;

        datastack.push(b);
        datastack.push(a);
        datastack.push(c);

        Ok(Self {
            datastack,
            callstack: self.callstack.clone(),
            dictionary: self.dictionary.clone(),
            count: self.count,
        })
    }

    pub fn emptystack(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        datastack.push(StackElement::SubStack(Vec::new()));
        Ok(Self {
            datastack,
            callstack: self.callstack.clone(),
            dictionary: self.dictionary.clone(),
            count: self.count,
        })
    }

    pub fn push(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        let e = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))?;

        if let StackElement::SubStack(mut ss) = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            ss.push(e);
            datastack.push(StackElement::SubStack(ss));
            return Ok(Self {
                datastack,
                callstack: self.callstack.clone(),
                dictionary: self.dictionary.clone(),
                count: self.count,
            });
        }
        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} I can only push to stacks", "Error:".red().bold()),
        ))
    }

    pub fn r#type(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        match datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            StackElement::SubStack(_) => datastack.push(StackElement::Word("stk".to_string())),
            StackElement::Word(_) => datastack.push(StackElement::Word("wrd".to_string())),
            StackElement::Map(_) => datastack.push(StackElement::Word("map".to_string())),
            StackElement::Nil => datastack.push(StackElement::Word("nil".to_string())),
            StackElement::Fun(_) => datastack.push(StackElement::Word("fct".to_string())),
        };

        Ok(Self {
            datastack,
            callstack: self.callstack.clone(),
            dictionary: self.dictionary.clone(),
            count: self.count,
        })
    }

    pub fn equal(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        let a = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))?;

        let b = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))?;

        if a == b {
            datastack.push(StackElement::Word("t".to_string()));
        } else {
            datastack.push(StackElement::Word("f".to_string()));
        }

        Ok(Interpreter {
            datastack,
            callstack: self.callstack.clone(),
            dictionary: self.dictionary.clone(),
            count: self.count,
        })
    }

    pub fn identical(&self) -> Result<Self, Error> {
        unimplemented!()
    }

    pub fn pop(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        let substack = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))?;

        match substack {
            StackElement::SubStack(mut ss) => {
                ss.pop().unwrap_or(StackElement::Nil);
                datastack.push(StackElement::SubStack(ss));
                Ok(Self {
                    datastack,
                    callstack: self.callstack.clone(),
                    dictionary: self.dictionary.clone(),
                    count: self.count,
                })
            }
            _ => Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} I can only pop from stacks", "Error:".red().bold()),
            )),
        }
    }

    pub fn top(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        let substack = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))?;
        match substack {
            StackElement::SubStack(mut ss) => {
                let el = ss.pop().unwrap_or(StackElement::Nil);
                datastack.push(el);
                Ok(Self {
                    datastack,
                    callstack: self.callstack.clone(),
                    dictionary: self.dictionary.clone(),
                    count: self.count,
                })
            }
            _ => Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} I can only pop from stacks", "Error:".red().bold()),
            )),
        }
    }

    pub fn concat(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::SubStack(mut ss1) = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            if let StackElement::SubStack(mut ss2) = datastack.pop().ok_or(Error::new(
                io::ErrorKind::InvalidData,
                format!("{} not enough operands", "Error:".red().bold()),
            ))? {
                ss1.append(&mut ss2);
                datastack.push(StackElement::SubStack(ss1))
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

        Ok(Self {
            datastack,
            callstack: self.callstack.clone(),
            dictionary: self.dictionary.clone(),
            count: self.count,
        })
    }

    pub fn reverse(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::SubStack(mut ss) = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            ss.reverse();
            datastack.push(StackElement::SubStack(ss))
        } else {
            return Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} I can only reverse stacks", "Error:".red().bold()),
            ));
        }
        Ok(Self {
            datastack,
            callstack: self.callstack.clone(),
            dictionary: self.dictionary.clone(),
            count: self.count,
        })
    }

    pub fn mapping(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::SubStack(ss) = datastack.pop().ok_or(Error::new(
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

            datastack.push(StackElement::Map(map));
            return Ok(Self {
                datastack,
                callstack: self.callstack.clone(),
                dictionary: self.dictionary.clone(),
                count: self.count,
            });
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need map to create mapping", "Error:".red().bold()),
        ))
    }

    pub fn unmap(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::Map(map) = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            let keys: Vec<StackElement> = map.iter().map(|(i, _)| i).cloned().collect();
            let values: Vec<StackElement> = map.iter().map(|(_, i)| i).cloned().collect();
            let mut st = Vec::new();
            for i in 0..keys.len() {
                st.push(keys[i].clone());
                st.push(values[i].clone());
            }

            datastack.push(StackElement::SubStack(st));

            return Ok(Self {
                datastack,
                callstack: self.callstack.clone(),
                dictionary: self.dictionary.clone(),
                count: self.count,
            });
        }
        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need map to unmap", "Error:".red().bold()),
        ))
    }

    pub fn keys(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::Map(map) = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            let keys = map
                .iter()
                .map(|(i, _)| i)
                .cloned()
                .collect::<Vec<StackElement>>();
            datastack.push(StackElement::SubStack(keys));

            return Ok(Self {
                datastack,
                callstack: self.callstack.clone(),
                dictionary: self.dictionary.clone(),
                count: self.count,
            });
        }
        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need map to list keys", "Error:".red().bold()),
        ))
    }

    pub fn assoc(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::Map(mut map) = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            let key = datastack.pop().ok_or(Error::new(
                io::ErrorKind::InvalidData,
                format!("{} not enough operands", "Error:".red().bold()),
            ))?;
            let value = datastack.pop().ok_or(Error::new(
                io::ErrorKind::InvalidData,
                format!("{} not enough operands", "Error:".red().bold()),
            ))?;

            if !map.iter().any(|(i, _)| *i == key) {
                map.push((key, value));
            }

            datastack.push(StackElement::Map(map));

            return Ok(Self {
                datastack,
                callstack: self.callstack.clone(),
                dictionary: self.dictionary.clone(),
                count: self.count,
            });
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need map to assoc value to map", "Error:".red().bold()),
        ))
    }

    pub fn dissoc(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::Map(mut map) = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            let key = datastack.pop().ok_or(Error::new(
                io::ErrorKind::InvalidData,
                format!("{} not enough operands", "Error:".red().bold()),
            ))?;
            map.retain(|(i, _)| *i == key);

            datastack.push(StackElement::Map(map));

            return Ok(Self {
                datastack,
                callstack: self.callstack.clone(),
                dictionary: self.dictionary.clone(),
                count: self.count,
            });
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "{} need map to dissoc value from map",
                "Error:".red().bold()
            ),
        ))
    }

    pub fn get(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        let default = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))?;
        if let StackElement::Map(m) = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            let key = datastack.pop().ok_or(Error::new(
                io::ErrorKind::InvalidData,
                format!("{} not enough operands", "Error:".red().bold()),
            ))?;

            datastack.push(match m.iter().find(|(i, _)| *i == key) {
                Some((_, j)) => j.clone(),
                None => default,
            });

            return Ok(Self {
                datastack,
                callstack: self.callstack.clone(),
                dictionary: self.dictionary.clone(),
                count: self.count,
            });
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need map to get value from map", "Error:".red().bold()),
        ))
    }

    pub fn merge(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::Map(mut m1) = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            if let StackElement::Map(m2) = datastack.pop().ok_or(Error::new(
                io::ErrorKind::InvalidData,
                format!("{} not enough operands", "Error:".red().bold()),
            ))? {
                m2.iter().for_each(|(k, v)| {
                    if !m1.iter().any(|(i, _)| *i == *k) {
                        m1.push((k.clone(), v.clone()));
                    }
                });

                datastack.push(StackElement::Map(m1));

                return Ok(Self {
                    datastack,
                    callstack: self.callstack.clone(),
                    dictionary: self.dictionary.clone(),
                    count: self.count,
                });
            }
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need maps to merge maps", "Error:".red().bold()),
        ))
    }

    pub fn word(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::SubStack(st) = datastack.pop().ok_or(Error::new(
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
                datastack.push(StackElement::Word(s));
                return Ok(Self {
                    datastack,
                    callstack: self.callstack.clone(),
                    dictionary: self.dictionary.clone(),
                    count: self.count,
                });
            }
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need stack to use 'word'", "Error:".red().bold()),
        ))
    }

    pub fn unword(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::Word(str) = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            datastack.push(StackElement::SubStack(
                str.chars()
                    .map(|c| StackElement::Word(c.to_string()))
                    .collect(),
            ));

            return Ok(Self {
                datastack,
                callstack: self.callstack.clone(),
                dictionary: self.dictionary.clone(),
                count: self.count,
            });
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need word to use 'unword'", "Error:".red().bold()),
        ))
    }

    pub fn char(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::Word(w) = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            datastack.push(StackElement::Word(match w.as_str() {
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

            return Ok(Interpreter {
                datastack,
                callstack: self.callstack.clone(),
                dictionary: self.dictionary.clone(),
                count: self.count,
            });
        }
        Err(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} need word to parse to utf", "Error:".red().bold()),
        ))
    }

    pub fn print(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        print!(
            "{}",
            datastack.pop().ok_or(Error::new(
                io::ErrorKind::InvalidData,
                format!("{} not enough operands", "Error:".red().bold()),
            ))?
        );

        Ok(Self {
            datastack,
            callstack: self.callstack.clone(),
            dictionary: self.dictionary.clone(),
            count: self.count,
        })
    }

    pub fn flush(&self) -> Result<Self, Error> {
        stdout().flush().unwrap();

        Ok(Self {
            datastack: self.datastack.clone(),
            callstack: self.callstack.clone(),
            dictionary: self.dictionary.clone(),
            count: self.count,
        })
    }

    pub fn read_line(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        let mut inp = "".to_string();
        stdin().read_line(&mut inp).unwrap();

        datastack.push(StackElement::Word(inp));
        Ok(Self {
            datastack,
            callstack: self.callstack.clone(),
            dictionary: self.dictionary.clone(),
            count: self.count,
        })
    }

    pub fn slurp(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::Word(src) = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            datastack.push(StackElement::Word(fs::read_to_string(src)?));
            return Ok(Self {
                datastack,
                callstack: self.callstack.clone(),
                dictionary: self.dictionary.clone(),
                count: self.count,
            });
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need word to read from file", "Error:".red().bold()),
        ))
    }

    pub fn spit(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::Word(data) = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            if let StackElement::Word(file) = datastack.pop().ok_or(Error::new(
                io::ErrorKind::InvalidData,
                format!("{} not enough operands", "Error:".red().bold()),
            ))? {
                fs::write(file, data)?;
                return Ok(Self {
                    datastack,
                    callstack: self.callstack.clone(),
                    dictionary: self.dictionary.clone(),
                    count: self.count,
                });
            }
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} could not write file", "Error:".red().bold()),
        ))
    }

    pub fn spit_on(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::Word(data) = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            if let StackElement::Word(path) = datastack.pop().ok_or(Error::new(
                io::ErrorKind::InvalidData,
                format!("{} not enough operands", "Error:".red().bold()),
            ))? {
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(path)
                    .unwrap();

                file.write_all(data.as_bytes()).unwrap();
                return Ok(Self {
                    datastack,
                    callstack: self.callstack.clone(),
                    dictionary: self.dictionary.clone(),
                    count: self.count,
                });
            }
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} could not write file", "Error:".red().bold()),
        ))
    }

    pub fn uncomment(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::Word(wrd) = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            datastack.push(StackElement::Word(
                wrd.lines()
                    .map(|l| l.split('%').next().unwrap())
                    .collect::<String>(),
            ));

            return Ok(Self {
                datastack,
                callstack: self.callstack.clone(),
                dictionary: self.dictionary.clone(),
                count: self.count,
            });
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} I can only uncomment words", "Error:".red().bold()),
        ))
    }

    pub fn tokenize(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::Word(w) = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            datastack.push(StackElement::SubStack(
                w.split_whitespace()
                    .map(|s| StackElement::Word(s.to_string()))
                    .rev()
                    .collect(),
            ));

            return Ok(Self {
                datastack,
                callstack: self.callstack.clone(),
                dictionary: self.dictionary.clone(),
                count: self.count,
            });
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need word to tokenize", "Error:".red().bold()),
        ))
    }

    pub fn undocument(&self) -> Result<Self, Error> {
        todo!()
    }

    pub fn current_time_millis(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        datastack.push(StackElement::Word(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis()
                .to_string(),
        ));

        Ok(Self {
            datastack,
            callstack: self.callstack.clone(),
            dictionary: self.dictionary.clone(),
            count: self.count,
        })
    }

    pub fn operating_system(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        datastack.push(StackElement::Word(env::consts::OS.to_string()));

        Ok(Self {
            datastack,
            callstack: self.callstack.clone(),
            dictionary: self.dictionary.clone(),
            count: self.count,
        })
    }

    pub fn call(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        let mut callstack = self.callstack.clone();
        match datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            StackElement::SubStack(mut st) => callstack.append(&mut st),
            StackElement::Word(w) => callstack.push(StackElement::Word(w)),
            _ => panic!("bitte passier nicht"),
        }

        Ok(Self {
            datastack,
            callstack,
            dictionary: self.dictionary.clone(),
            count: self.count,
        })
    }

    pub fn call_cc(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        let mut callstack = self.callstack.clone();

        let top = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))?;
        callstack.clear();
        match top.clone() {
            StackElement::SubStack(mut ss) => callstack.append(&mut ss),
            el => callstack.push(el),
        }

        datastack = [
            StackElement::SubStack(datastack.clone()),
            StackElement::SubStack(self.callstack.clone()),
        ]
        .to_vec();

        Ok(Self {
            datastack,
            callstack,
            dictionary: self.dictionary.clone(),
            count: self.count,
        })
    }

    pub fn r#continue(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();

        if let StackElement::SubStack(new_callstack) = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            if let StackElement::SubStack(new_datastack) = datastack.pop().ok_or(Error::new(
                io::ErrorKind::InvalidData,
                format!("{} not enough operands", "Error:".red().bold()),
            ))? {
                return Ok(Interpreter {
                    datastack: new_datastack,
                    callstack: new_callstack,
                    dictionary: self.dictionary.clone(),
                    count: self.count,
                });
            }
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need quotation for continue", "Error:".red().bold()),
        ))
    }

    pub fn get_dict(&self) -> Result<Self, Error> {
        let mut map = Vec::new();
        let mut datastack = self.datastack.clone();
        self.dictionary.iter().for_each(|(k, v)| {
            let key = StackElement::Word(k.clone());
            let value = StackElement::Fun(v.clone());
            if !map.iter().map(|(i, _)| i).any(|se| key == *se) {
                map.push((key, value));
            }
        });

        datastack.push(StackElement::Map(map));

        Ok(Self {
            datastack,
            callstack: self.callstack.clone(),
            dictionary: self.dictionary.clone(),
            count: self.count,
        })
    }

    pub fn set_dict(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::Map(dict) = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            let dict = map_to_dict(&dict)?;
            return Ok(Self {
                datastack,
                callstack: self.callstack.clone(),
                dictionary: dict,
                count: self.count,
            });
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need map for set-dict", "Error:".red().bold()),
        ))
    }

    pub fn stepcc(&self) -> Result<Self, Error> {
        let mut callstack = self.callstack.clone();
        let mut datastack = self.datastack.clone();
        let mut count = self.count;
        let e = callstack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))?;

        count += 1;

        //println!("{}; {}", count, e);

        match e {
            StackElement::SubStack(ss) => datastack.push(StackElement::SubStack(ss)),
            StackElement::Word(w) => {
                match self.dictionary.get(&w) {
                    Some(fun) => match fun.deref() {
                        Funct::BuiltIn(fct) => {
                            return fct(&Interpreter {
                                datastack,
                                callstack,
                                dictionary: self.dictionary.clone(),
                                count,
                            });
                        }
                        Funct::SelfDefined(stack) => match stack.clone() {
                            StackElement::SubStack(mut ss) => {
                                callstack.append(&mut ss);
                            }
                            _ => panic!("bitte bitte trete nicht ein"),
                        },
                    },
                    None => {
                        datastack.push(StackElement::Word(w));
                        callstack.push(StackElement::Word("read-word".to_string()))
                    }
                };
            }
            StackElement::Map(m) => {
                datastack.push(StackElement::Map(m));
                callstack.push(StackElement::Word("read-mapping".to_string()))
            }
            StackElement::Nil => datastack.push(StackElement::Nil),
            StackElement::Fun(f) => match f.deref() {
                Funct::BuiltIn(bi) => {
                    return bi(&Interpreter {
                        datastack: datastack.clone(),
                        callstack: callstack.clone(),
                        dictionary: self.dictionary.clone(),
                        count,
                    });
                }
                Funct::SelfDefined(sd) => match sd {
                    StackElement::SubStack(ss) => callstack.append(&mut ss.clone()),
                    _ => todo!("bitte bitte trete niemals ein"),
                },
            },
        };

        Ok(Self {
            datastack,
            callstack,
            dictionary: self.dictionary.clone(),
            count,
        })
    }

    pub fn apply(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::SubStack(stk) = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} apply: not enough operands", "Error:".red().bold()),
        ))? {
            if let StackElement::Word(wrd) = datastack.pop().ok_or(Error::new(
                io::ErrorKind::InvalidData,
                format!("{} apply: not enough operands", "Error:".red().bold()),
            ))? {
                if let Some(fct) = self.dictionary.get(wrd.as_str()) {
                    let mut inner = Self {
                        datastack: stk,
                        callstack: Vec::new(),
                        dictionary: self.dictionary.clone(),
                        count: self.count,
                    };
                    let next = match fct.deref() {
                        Funct::BuiltIn(fun) => fun(&inner),
                        Funct::SelfDefined(st) => {
                            inner.callstack.push(st.clone());
                            inner.stepcc()
                        }
                    }?;

                    datastack.push(StackElement::SubStack(next.datastack));

                    return Ok(Self {
                        datastack,
                        callstack: self.callstack.clone(),
                        dictionary: self.dictionary.clone(),
                        count: self.count,
                    });
                }
            }
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need stack and word for apply", "Error:".red().bold()),
        ))
    }

    pub fn compose(&self) -> Result<Self, Error> {
        todo!()
    }

    pub fn func(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::Map(dict) = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            let dict = map_to_dict(&dict)?;
            if let StackElement::SubStack(qt) = datastack.pop().unwrap() {
                fn runcc(inner: &Interpreter) -> Result<Vec<StackElement>, Error> {
                    if inner.callstack.is_empty() {
                        Ok(inner.datastack.clone())
                    } else {
                        match inner.stepcc() {
                            Ok(i) => runcc(&i),
                            Err(e) => Err(e),
                        }
                    }
                }
                let mut ds_dash = runcc(&Self {
                    datastack: Vec::new(),
                    callstack: qt,
                    dictionary: dict,
                    count: self.count,
                })?;

                datastack.append(&mut ds_dash);
            }
        }

        Ok(Self {
            datastack,
            callstack: self.callstack.clone(),
            dictionary: self.dictionary.clone(),
            count: self.count,
        })
    }

    pub fn integer(&self) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        let e = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))?;
        datastack.push(match e {
            StackElement::Word(w) => match w.parse::<usize>() {
                Ok(_) => StackElement::Word("t".to_string()),
                Err(_) => StackElement::Word("f".to_string()),
            },
            _ => StackElement::Word("f".to_string()),
        });

        Ok(Self {
            datastack,
            callstack: self.callstack.clone(),
            dictionary: self.dictionary.clone(),
            count: self.count,
        })
    }

    pub fn addition(&self) -> Result<Self, Error> {
        self.binary("+")
    }

    pub fn subtraction(&self) -> Result<Self, Error> {
        self.binary("-")
    }

    pub fn multiplication(&self) -> Result<Self, Error> {
        self.binary("*")
    }

    pub fn division(&self) -> Result<Self, Error> {
        self.binary("div")
    }

    pub fn modulo(&self) -> Result<Self, Error> {
        self.binary("mod")
    }

    pub fn greater_than(&self) -> Result<Self, Error> {
        self.binary(">")
    }

    pub fn less_than(&self) -> Result<Self, Error> {
        self.binary("<")
    }

    pub fn equals(&self) -> Result<Self, Error> {
        self.binary("==")
    }

    pub fn less_equals(&self) -> Result<Self, Error> {
        self.binary("<=")
    }

    pub fn greater_equals(&self) -> Result<Self, Error> {
        self.binary(">=")
    }

    fn binary(&self, op: &str) -> Result<Self, Error> {
        let mut datastack = self.datastack.clone();
        if self.datastack.len() < 2 {
            return Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} not enough operands", "Error:".red().bold()),
            ));
        }
        if let StackElement::Word(w1) = datastack.pop().ok_or(Error::new(
            io::ErrorKind::InvalidData,
            format!("{} not enough operands", "Error:".red().bold()),
        ))? {
            if let StackElement::Word(w2) = datastack.pop().ok_or(Error::new(
                io::ErrorKind::InvalidData,
                format!("{} not enough operands", "Error:".red().bold()),
            ))? {
                if w1.parse::<usize>().is_ok() && w2.parse::<usize>().is_ok() {
                    let x: usize = w1.parse().map_err(|_| {
                        Error::new(
                            io::ErrorKind::InvalidData,
                            format!("{} not enough operands", "Error:".red().bold()),
                        )
                    })?;
                    let y: usize = w2.parse().map_err(|_| {
                        Error::new(
                            io::ErrorKind::InvalidData,
                            format!("{} not enough operands", "Error:".red().bold()),
                        )
                    })?;
                    datastack.push(StackElement::Word(match op {
                        "+" => (x + y).to_string(),
                        "-" => (x - y).to_string(),
                        "*" => (x * y).to_string(),
                        "div" => (x / y).to_string(),
                        "mod" => (x % y).to_string(),
                        ">=" => match x >= y {
                            true => "t".to_string(),
                            false => "f".to_string(),
                        },
                        "==" => match x == y {
                            true => "t".to_string(),
                            false => "f".to_string(),
                        },
                        "<" => match x < y {
                            true => "t".to_string(),
                            false => "f".to_string(),
                        },
                        ">" => match x > y {
                            true => "t".to_string(),
                            false => "f".to_string(),
                        },
                        "<=" => match x <= y {
                            true => "t".to_string(),
                            false => "f".to_string(),
                        },
                        _ => panic!("unknown operator"),
                    }));
                    return Ok(Self {
                        datastack,
                        callstack: self.callstack.clone(),
                        dictionary: self.dictionary.clone(),
                        count: self.count,
                    });
                }
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

    pub fn comment(&self) -> Result<Self, Error> {
        let mut callstack = self.callstack.clone();
        let mut datastack = self.datastack.clone();
        datastack.push(StackElement::Fun(Rc::new(Funct::SelfDefined(
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
        ))));
        callstack.push(StackElement::Word("call/cc".to_string()));

        Ok(Self {
            datastack,
            callstack,
            dictionary: self.dictionary.clone(),
            count: self.count,
        })
    }

    pub fn load(&self) -> Result<Self, Error> {
        let mut callstack = self.callstack.clone();
        callstack.append(
            &mut [
                StackElement::Word("tokenize".to_string()),
                StackElement::Word("uncomment".to_string()),
                StackElement::Word("slurp".to_string()),
            ]
            .to_vec(),
        );

        Ok(Self {
            datastack: self.datastack.clone(),
            callstack,
            dictionary: self.dictionary.clone(),
            count: self.count,
        })
    }

    pub fn run(&self) -> Result<Self, Error> {
        let mut callstack = self.callstack.clone();
        callstack.append(
            &mut [
                StackElement::Word("call".to_string()),
                StackElement::Word("load".to_string()),
            ]
            .to_vec(),
        );

        Ok(Self {
            datastack: self.datastack.clone(),
            callstack,
            dictionary: self.dictionary.clone(),
            count: self.count,
        })
    }

    pub fn start(&self) -> Result<Self, Error> {
        let mut callstack = self.callstack.clone();
        callstack.append(
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

        Ok(Self {
            datastack: self.datastack.clone(),
            callstack,
            dictionary: self.dictionary.clone(),
            count: self.count,
        })
    }
}
