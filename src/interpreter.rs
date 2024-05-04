use std::{
    collections::BTreeMap,
    env,
    fs::{self, OpenOptions},
    io::{self, stdin, stdout, Error, Write},
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use colored::Colorize;

use crate::stack_element::StackElement;

pub enum Funct {
    BuiltIn(BuiltIn),
    SelfDefined(Vec<StackElement>),
}

type BuiltIn = Box<dyn Fn(&Interpreter) -> Result<Interpreter, Error>>;

pub struct Interpreter {
    pub datastack: Vec<StackElement>,
    pub callstack: Vec<StackElement>,
    pub dictionary: Rc<BTreeMap<String, Funct>>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            datastack: Vec::new(),
            callstack: Vec::new(),
            dictionary: Rc::new(Self::init_dictionary()),
        }
    }

    fn init_dictionary() -> BTreeMap<String, Funct> {
        let mut dict: BTreeMap<String, Funct> = BTreeMap::new();

        dict.insert("swap".to_string(), Funct::BuiltIn(Box::new(Self::swap)));
        dict.insert("dup".to_string(), Funct::BuiltIn(Box::new(Self::dup)));
        dict.insert("drop".to_string(), Funct::BuiltIn(Box::new(Self::drop)));
        dict.insert("rot".to_string(), Funct::BuiltIn(Box::new(Self::rot)));
        dict.insert("type".to_string(), Funct::BuiltIn(Box::new(Self::r#type)));
        dict.insert("equal?".to_string(), Funct::BuiltIn(Box::new(Self::equal))); //TODO
        dict.insert(
            "identical?".to_string(),
            Funct::BuiltIn(Box::new(Self::identical)),
        ); //TODO
        dict.insert(
            "emptystack".to_string(),
            Funct::BuiltIn(Box::new(Self::emptystack)),
        );
        dict.insert("push".to_string(), Funct::BuiltIn(Box::new(Self::push)));
        dict.insert("top".to_string(), Funct::BuiltIn(Box::new(Self::top)));
        dict.insert("pop".to_string(), Funct::BuiltIn(Box::new(Self::pop)));
        dict.insert("concat".to_string(), Funct::BuiltIn(Box::new(Self::concat)));
        dict.insert(
            "reverse".to_string(),
            Funct::BuiltIn(Box::new(Self::reverse)),
        );
        dict.insert(
            "mapping".to_string(),
            Funct::BuiltIn(Box::new(Self::mapping)),
        );
        dict.insert("unmap".to_string(), Funct::BuiltIn(Box::new(Self::unmap)));
        dict.insert("keys".to_string(), Funct::BuiltIn(Box::new(Self::keys)));
        dict.insert("assoc".to_string(), Funct::BuiltIn(Box::new(Self::assoc)));
        dict.insert("dissoc".to_string(), Funct::BuiltIn(Box::new(Self::dissoc)));
        dict.insert("get".to_string(), Funct::BuiltIn(Box::new(Self::get)));
        dict.insert("merge".to_string(), Funct::BuiltIn(Box::new(Self::merge)));
        dict.insert("word".to_string(), Funct::BuiltIn(Box::new(Self::word)));
        dict.insert("unword".to_string(), Funct::BuiltIn(Box::new(Self::unword)));
        dict.insert("char".to_string(), Funct::BuiltIn(Box::new(Self::char))); //TODO
        dict.insert("print".to_string(), Funct::BuiltIn(Box::new(Self::print)));
        dict.insert("flush".to_string(), Funct::BuiltIn(Box::new(Self::flush)));
        dict.insert(
            "read-line".to_string(),
            Funct::BuiltIn(Box::new(Self::read_line)),
        );
        dict.insert("slurp".to_string(), Funct::BuiltIn(Box::new(Self::slurp)));
        dict.insert("spit".to_string(), Funct::BuiltIn(Box::new(Self::spit)));
        dict.insert(
            "spit-on".to_string(),
            Funct::BuiltIn(Box::new(Self::spit_on)),
        );
        dict.insert(
            "uncomment".to_string(),
            Funct::BuiltIn(Box::new(Self::uncomment)),
        );
        dict.insert(
            "tokenize".to_string(),
            Funct::BuiltIn(Box::new(Self::tokenize)),
        );
        dict.insert(
            "undocument".to_string(),
            Funct::BuiltIn(Box::new(Self::undocument)),
        ); //TODO
        dict.insert(
            "current-time-millis".to_string(),
            Funct::BuiltIn(Box::new(Self::current_time_millis)),
        );
        dict.insert(
            "operating-system".to_string(),
            Funct::BuiltIn(Box::new(Self::operating_system)),
        );
        dict.insert("call".to_string(), Funct::BuiltIn(Box::new(Self::call)));
        dict.insert(
            "call/cc".to_string(),
            Funct::BuiltIn(Box::new(Self::call_cc)),
        ); //TODO
        dict.insert(
            "continue".to_string(),
            Funct::BuiltIn(Box::new(Self::r#continue)),
        ); //TODO
        dict.insert(
            "get-dict".to_string(),
            Funct::BuiltIn(Box::new(Self::get_dict)),
        );
        dict.insert(
            "set-dict".to_string(),
            Funct::BuiltIn(Box::new(Self::set_dict)),
        ); //TODO
        dict.insert("stepcc".to_string(), Funct::BuiltIn(Box::new(Self::stepcc))); //TODO
        dict.insert("apply".to_string(), Funct::BuiltIn(Box::new(Self::apply)));
        dict.insert(
            "compose".to_string(),
            Funct::BuiltIn(Box::new(Self::compose)),
        ); //TODO
        dict.insert("func".to_string(), Funct::BuiltIn(Box::new(Self::func))); //TODO
        dict.insert(
            "integer?".to_string(),
            Funct::BuiltIn(Box::new(Self::integer)),
        );
        dict.insert("+".to_string(), Funct::BuiltIn(Box::new(Self::addition)));
        dict.insert("-".to_string(), Funct::BuiltIn(Box::new(Self::subtraction)));
        dict.insert(
            "*".to_string(),
            Funct::BuiltIn(Box::new(Self::multiplication)),
        );
        dict.insert("div".to_string(), Funct::BuiltIn(Box::new(Self::division)));
        dict.insert("mod".to_string(), Funct::BuiltIn(Box::new(Self::modulo)));
        dict.insert("<".to_string(), Funct::BuiltIn(Box::new(Self::less_than)));
        dict.insert(
            ">".to_string(),
            Funct::BuiltIn(Box::new(Self::greater_than)),
        );
        dict.insert("==".to_string(), Funct::BuiltIn(Box::new(Self::equals)));
        dict.insert(
            "<=".to_string(),
            Funct::BuiltIn(Box::new(Self::less_equals)),
        );
        dict.insert(
            ">=".to_string(),
            Funct::BuiltIn(Box::new(Self::greater_equals)),
        );
        dict.insert("\\".to_string(), Funct::BuiltIn(Box::new(Self::comment)));
        dict.insert("load".to_string(), Funct::BuiltIn(Box::new(Self::load)));
        dict.insert("run".to_string(), Funct::BuiltIn(Box::new(Self::run)));
        dict.insert("start".to_string(), Funct::BuiltIn(Box::new(Self::start)));

        dict
    }

    fn dup(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        if self.datastack.is_empty() {
            return Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} not enough operands", "Error:".red().bold()),
            ));
        }
        let a = datastack.pop().unwrap();
        datastack.push(a.clone());
        datastack.push(a);
        Ok(Interpreter {
            datastack,
            callstack: self.callstack.clone(),
            dictionary: self.dictionary.clone(),
        })
    }

    fn swap(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        if self.datastack.len() > 2 {
            let a = datastack.pop().unwrap();
            let b = datastack.pop().unwrap();
            datastack.push(a);
            datastack.push(b);

            return Ok(Interpreter {
                datastack,
                callstack: self.callstack.clone(),
                dictionary: self.dictionary.clone(),
            });
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} not enough operands", "Error:".red().bold()),
        ))
    }

    fn drop(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        if self.datastack.is_empty() {
            return Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} not enough operands", "Error:".red().bold()),
            ));
        }
        datastack.pop().unwrap();

        Ok(Interpreter {
            datastack,
            callstack: self.callstack.clone(),
            dictionary: self.dictionary.clone(),
        })
    }

    fn rot(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        if self.datastack.len() < 3 {
            return Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} not enough operands", "Error:".red().bold()),
            ));
        }

        let a = datastack.pop().unwrap();
        let b = datastack.pop().unwrap();
        let c = datastack.pop().unwrap();

        datastack.push(b);
        datastack.push(a);
        datastack.push(c);

        Ok(Interpreter {
            datastack,
            callstack: self.callstack.clone(),
            dictionary: self.dictionary.clone(),
        })
    }

    fn emptystack(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        datastack.push(StackElement::SubStack(Vec::new()));
        Ok(Interpreter {
            datastack,
            callstack: self.callstack.clone(),
            dictionary: self.dictionary.clone(),
        })
    }

    fn push(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        let e = datastack.pop().unwrap();

        if let StackElement::SubStack(mut ss) = datastack.pop().unwrap() {
            ss.push(e);
            datastack.push(StackElement::SubStack(ss));
            return Ok(Interpreter {
                datastack,
                callstack: self.callstack.clone(),
                dictionary: self.dictionary.clone(),
            });
        }
        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} I can only push to stacks", "Error:".red().bold()),
        ))
    }

    fn r#type(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        match datastack.pop().unwrap() {
            StackElement::SubStack(_) => datastack.push(StackElement::Word("stk".to_string())),
            StackElement::Word(_) => datastack.push(StackElement::Word("wrd".to_string())),
            StackElement::Map(_) => datastack.push(StackElement::Word("map".to_string())),
            StackElement::Nil => datastack.push(StackElement::Word("nil".to_string())),
        };

        Ok(Interpreter {
            datastack,
            callstack: self.callstack.clone(),
            dictionary: self.dictionary.clone(),
        })
    }

    fn equal(&self) -> Result<Interpreter, Error> {
        todo!()
    }

    fn identical(&self) -> Result<Interpreter, Error> {
        todo!()
    }

    fn pop(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        let substack = datastack.pop().unwrap();
        match substack {
            StackElement::SubStack(mut ss) => {
                ss.pop();
                datastack.push(StackElement::SubStack(ss));
                Ok(Interpreter {
                    datastack,
                    callstack: self.callstack.clone(),
                    dictionary: self.dictionary.clone(),
                })
            }
            _ => Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} I can only pop from stacks", "Error:".red().bold()),
            )),
        }
    }

    fn top(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        let substack = datastack.pop().unwrap();
        match substack {
            StackElement::SubStack(mut ss) => {
                let el = ss.pop().unwrap();
                datastack.push(el);
                Ok(Interpreter {
                    datastack,
                    callstack: self.callstack.clone(),
                    dictionary: self.dictionary.clone(),
                })
            }
            _ => Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} I can only pop from stacks", "Error:".red().bold()),
            )),
        }
    }

    fn concat(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::SubStack(mut ss1) = datastack.pop().unwrap() {
            if let StackElement::SubStack(mut ss2) = datastack.pop().unwrap() {
                ss2.append(&mut ss1);
                datastack.push(StackElement::SubStack(ss2))
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

        Ok(Interpreter {
            datastack,
            callstack: self.callstack.clone(),
            dictionary: self.dictionary.clone(),
        })
    }

    fn reverse(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::SubStack(mut ss) = datastack.pop().unwrap() {
            ss.reverse();
            datastack.push(StackElement::SubStack(ss))
        } else {
            return Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} I can only reverse stacks", "Error:".red().bold()),
            ));
        }
        Ok(Interpreter {
            datastack,
            callstack: self.callstack.clone(),
            dictionary: self.dictionary.clone(),
        })
    }

    fn mapping(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::SubStack(ss) = datastack.pop().unwrap() {
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

            datastack.push(StackElement::Map(map));
            return Ok(Interpreter {
                datastack,
                callstack: self.callstack.clone(),
                dictionary: self.dictionary.clone(),
            });
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need map to create mapping", "Error:".red().bold()),
        ))
    }

    fn unmap(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::Map(map) = datastack.pop().unwrap() {
            let keys = map.keys().cloned().collect::<Vec<StackElement>>();
            let values = map.values().cloned().collect::<Vec<StackElement>>();
            let mut st = Vec::new();
            for i in 0..keys.len() {
                st.push(keys[i].clone());
                st.push(values[i].clone());
            }

            datastack.push(StackElement::SubStack(st));

            return Ok(Interpreter {
                datastack,
                callstack: self.callstack.clone(),
                dictionary: self.dictionary.clone(),
            });
        }
        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need map to unmap", "Error:".red().bold()),
        ))
    }

    fn keys(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::Map(map) = datastack.pop().unwrap() {
            let keys = map.keys().cloned().collect::<Vec<StackElement>>();
            datastack.push(StackElement::SubStack(keys));

            return Ok(Interpreter {
                datastack,
                callstack: self.callstack.clone(),
                dictionary: self.dictionary.clone(),
            });
        }
        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need map to list keys", "Error:".red().bold()),
        ))
    }

    fn assoc(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::Map(mut map) = datastack.pop().unwrap() {
            let key = datastack.pop().unwrap();
            let value = datastack.pop().unwrap();
            map.insert(key, value);

            datastack.push(StackElement::Map(map));

            return Ok(Interpreter {
                datastack,
                callstack: self.callstack.clone(),
                dictionary: self.dictionary.clone(),
            });
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need map to assoc value to map", "Error:".red().bold()),
        ))
    }

    fn dissoc(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::Map(mut map) = datastack.pop().unwrap() {
            let key = datastack.pop().unwrap();
            map.remove(&key);

            datastack.push(StackElement::Map(map));

            return Ok(Interpreter {
                datastack,
                callstack: self.callstack.clone(),
                dictionary: self.dictionary.clone(),
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

    fn get(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        let key = datastack.pop().unwrap();
        if let StackElement::Map(m) = datastack.pop().unwrap() {
            let default = datastack.pop().unwrap();

            datastack.push(m.get(&key).unwrap_or(&default).clone());

            return Ok(Interpreter {
                datastack,
                callstack: self.callstack.clone(),
                dictionary: self.dictionary.clone(),
            });
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need map to get value from map", "Error:".red().bold()),
        ))
    }

    fn merge(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::Map(mut m1) = datastack.pop().unwrap() {
            if let StackElement::Map(m2) = datastack.pop().unwrap() {
                m2.iter().for_each(|(k, v)| {
                    m1.insert(k.clone(), v.clone()).unwrap();
                });

                datastack.push(StackElement::Map(m1));

                return Ok(Interpreter {
                    datastack,
                    callstack: self.callstack.clone(),
                    dictionary: self.dictionary.clone(),
                });
            }
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need maps to merge maps", "Error:".red().bold()),
        ))
    }

    fn word(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::SubStack(st) = datastack.pop().unwrap() {
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
                return Ok(Interpreter {
                    datastack,
                    callstack: self.callstack.clone(),
                    dictionary: self.dictionary.clone(),
                });
            }
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need stack to use 'word'", "Error:".red().bold()),
        ))
    }

    fn unword(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::Word(str) = datastack.pop().unwrap() {
            datastack.push(StackElement::SubStack(
                str.chars()
                    .map(|c| StackElement::Word(c.to_string()))
                    .collect(),
            ));

            return Ok(Interpreter {
                datastack,
                callstack: self.callstack.clone(),
                dictionary: self.dictionary.clone(),
            });
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need word to use 'unword'", "Error:".red().bold()),
        ))
    }

    fn char(&self) -> Result<Interpreter, Error> {
        todo!()
    }

    fn print(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        print!("{}", datastack.pop().unwrap());

        Ok(Interpreter {
            datastack,
            callstack: self.callstack.clone(),
            dictionary: self.dictionary.clone(),
        })
    }

    fn flush(&self) -> Result<Interpreter, Error> {
        stdout().flush().unwrap();

        Ok(Interpreter {
            datastack: self.datastack.clone(),
            callstack: self.callstack.clone(),
            dictionary: self.dictionary.clone(),
        })
    }

    fn read_line(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        let mut inp = "".to_string();
        stdin().read_line(&mut inp).unwrap();

        datastack.push(StackElement::Word(inp));
        Ok(Interpreter {
            datastack,
            callstack: self.callstack.clone(),
            dictionary: self.dictionary.clone(),
        })
    }

    fn slurp(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::Word(src) = datastack.pop().unwrap() {
            datastack.push(StackElement::Word(fs::read_to_string(src)?));
            return Ok(Interpreter {
                datastack,
                callstack: self.callstack.clone(),
                dictionary: self.dictionary.clone(),
            });
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need word to read from file", "Error:".red().bold()),
        ))
    }

    fn spit(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::Word(data) = datastack.pop().unwrap() {
            if let StackElement::Word(file) = datastack.pop().unwrap() {
                fs::write(file, data)?;
                return Ok(Interpreter {
                    datastack,
                    callstack: self.callstack.clone(),
                    dictionary: self.dictionary.clone(),
                });
            }
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} could not write file", "Error:".red().bold()),
        ))
    }

    fn spit_on(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::Word(data) = datastack.pop().unwrap() {
            if let StackElement::Word(path) = datastack.pop().unwrap() {
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(path)
                    .unwrap();

                file.write_all(data.as_bytes()).unwrap();
                return Ok(Interpreter {
                    datastack,
                    callstack: self.callstack.clone(),
                    dictionary: self.dictionary.clone(),
                });
            }
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} could not write file", "Error:".red().bold()),
        ))
    }

    fn uncomment(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::Word(wrd) = datastack.pop().unwrap() {
            datastack.push(StackElement::Word(
                wrd.lines()
                    .map(|l| l.split(" %").next().unwrap())
                    .collect::<String>(),
            ));

            return Ok(Interpreter {
                datastack,
                callstack: self.callstack.clone(),
                dictionary: self.dictionary.clone(),
            });
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} I can only uncomment words", "Error:".red().bold()),
        ))
    }

    fn tokenize(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::Word(w) = datastack.pop().unwrap() {
            datastack.push(StackElement::SubStack(
                w.split_whitespace()
                    .map(|s| StackElement::Word(s.to_string()))
                    .collect(),
            ));

            return Ok(Interpreter {
                datastack,
                callstack: self.callstack.clone(),
                dictionary: self.dictionary.clone(),
            });
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need word to tokenize", "Error:".red().bold()),
        ))
    }

    fn undocument(&self) -> Result<Interpreter, Error> {
        todo!()
    }

    fn current_time_millis(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        datastack.push(StackElement::Word(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis()
                .to_string(),
        ));

        Ok(Interpreter {
            datastack,
            callstack: self.callstack.clone(),
            dictionary: self.dictionary.clone(),
        })
    }

    fn operating_system(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        datastack.push(StackElement::Word(env::consts::OS.to_string()));

        Ok(Interpreter {
            datastack,
            callstack: self.callstack.clone(),
            dictionary: self.dictionary.clone(),
        })
    }

    fn call(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        let mut callstack = self.callstack.clone();
        if let StackElement::SubStack(mut st) = datastack.pop().unwrap() {
            st.append(&mut callstack);
            callstack = st;
            return Ok(Interpreter {
                datastack,
                callstack,
                dictionary: self.dictionary.clone(),
            });
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "{} can only append stack to callstack",
                "Error:".red().bold()
            ),
        ))
    }

    fn call_cc(&self) -> Result<Interpreter, Error> {
        todo!()
    }

    fn r#continue(&self) -> Result<Interpreter, Error> {
        todo!()
    }

    fn get_dict(&self) -> Result<Interpreter, Error> {
        let mut map = BTreeMap::new();
        let mut datastack = self.datastack.clone();
        self.dictionary.iter().for_each(|(k, v)| {
            let key = StackElement::Word(k.clone());
            let value = match v {
                Funct::BuiltIn(_) => StackElement::Word("<fct>".to_string()),
                Funct::SelfDefined(st) => StackElement::SubStack(st.clone()),
            };

            map.insert(key, value);
        });

        datastack.push(StackElement::Map(map));

        Ok(Interpreter {
            datastack,
            callstack: self.callstack.clone(),
            dictionary: self.dictionary.clone(),
        })
    }

    fn set_dict(&self) -> Result<Interpreter, Error> {
        todo!()
    }

    fn stepcc(&self) -> Result<Interpreter, Error> {
        let mut callstack = self.callstack.clone();
        let e = callstack.pop().unwrap();

        Ok(Interpreter {
            datastack: self.datastack.clone(),
            callstack,
            dictionary: self.dictionary.clone(),
        })
    }

    fn apply(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        if let StackElement::SubStack(stk) = datastack.pop().unwrap() {
            if let StackElement::Word(wrd) = datastack.pop().unwrap() {
                if let Some(fct) = self.dictionary.get(wrd.as_str()) {
                    let mut inner = Interpreter {
                        datastack: stk,
                        callstack: Vec::new(),
                        dictionary: self.dictionary.clone(),
                    };
                    match fct {
                        Funct::BuiltIn(fun) => fun(&mut inner),
                        Funct::SelfDefined(st) => {
                            inner.callstack.append(&mut st.clone());
                            inner.stepcc()
                        }
                    }?;

                    datastack.push(StackElement::SubStack(inner.datastack));

                    return Ok(Interpreter {
                        datastack,
                        callstack: self.callstack.clone(),
                        dictionary: self.dictionary.clone(),
                    });
                }
            }
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need stack and word for apply", "Error:".red().bold()),
        ))
    }

    fn compose(&self) -> Result<Interpreter, Error> {
        todo!()
    }

    fn func(&self) -> Result<Interpreter, Error> {
        todo!()
    }

    fn integer(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        let e = datastack.pop().unwrap();
        datastack.push(match e {
            StackElement::Word(w) => match w.parse::<usize>() {
                Ok(_) => StackElement::Word("t".to_string()),
                Err(_) => StackElement::Word("f".to_string()),
            },
            _ => StackElement::Word("f".to_string()),
        });

        Ok(Interpreter {
            datastack,
            callstack: self.callstack.clone(),
            dictionary: self.dictionary.clone(),
        })
    }

    fn addition(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        if self.datastack.len() < 2 {
            return Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} not enough operands", "Error:".red().bold()),
            ));
        }
        if let StackElement::Word(w1) = datastack.pop().unwrap() {
            if let StackElement::Word(w2) = datastack.pop().unwrap() {
                if w1.parse::<usize>().is_ok() && w2.parse::<usize>().is_ok() {
                    datastack.push(StackElement::Word(
                        (w1.parse::<usize>().unwrap() + w2.parse::<usize>().unwrap()).to_string(),
                    ));
                    return Ok(Interpreter {
                        datastack,
                        callstack: self.callstack.clone(),
                        dictionary: self.dictionary.clone(),
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

    fn subtraction(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        if self.datastack.len() < 2 {
            return Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} not enough operands", "Error:".red().bold()),
            ));
        }
        if let StackElement::Word(w1) = datastack.pop().unwrap() {
            if let StackElement::Word(w2) = datastack.pop().unwrap() {
                if w1.parse::<usize>().is_ok() && w2.parse::<usize>().is_ok() {
                    datastack.push(StackElement::Word(
                        (w1.parse::<usize>().unwrap() - w2.parse::<usize>().unwrap()).to_string(),
                    ));
                    return Ok(Interpreter {
                        datastack,
                        callstack: self.callstack.clone(),
                        dictionary: self.dictionary.clone(),
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

    fn multiplication(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        if self.datastack.len() < 2 {
            return Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} not enough operands", "Error:".red().bold()),
            ));
        }
        if let StackElement::Word(w1) = datastack.pop().unwrap() {
            if let StackElement::Word(w2) = datastack.pop().unwrap() {
                if w1.parse::<usize>().is_ok() && w2.parse::<usize>().is_ok() {
                    datastack.push(StackElement::Word(
                        (w1.parse::<usize>().unwrap() * w2.parse::<usize>().unwrap()).to_string(),
                    ));
                    return Ok(Interpreter {
                        datastack,
                        callstack: self.callstack.clone(),
                        dictionary: self.dictionary.clone(),
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

    fn division(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        if self.datastack.len() < 2 {
            return Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} not enough operands", "Error:".red().bold()),
            ));
        }
        if let StackElement::Word(w1) = datastack.pop().unwrap() {
            if let StackElement::Word(w2) = datastack.pop().unwrap() {
                if w1.parse::<usize>().is_ok() && w2.parse::<usize>().is_ok() {
                    datastack.push(StackElement::Word(
                        (w1.parse::<usize>().unwrap() / w2.parse::<usize>().unwrap()).to_string(),
                    ));
                    return Ok(Interpreter {
                        datastack,
                        callstack: self.callstack.clone(),
                        dictionary: self.dictionary.clone(),
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

    fn modulo(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        if self.datastack.len() < 2 {
            return Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} not enough operands", "Error:".red().bold()),
            ));
        }
        if let StackElement::Word(w1) = datastack.pop().unwrap() {
            if let StackElement::Word(w2) = datastack.pop().unwrap() {
                if w1.parse::<usize>().is_ok() && w2.parse::<usize>().is_ok() {
                    datastack.push(StackElement::Word(
                        (w1.parse::<usize>().unwrap() % w2.parse::<usize>().unwrap()).to_string(),
                    ));
                    return Ok(Interpreter {
                        datastack,
                        callstack: self.callstack.clone(),
                        dictionary: self.dictionary.clone(),
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

    fn greater_than(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        if self.datastack.len() < 2 {
            return Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} not enough operands", "Error:".red().bold()),
            ));
        }
        if let StackElement::Word(w1) = datastack.pop().unwrap() {
            if let StackElement::Word(w2) = datastack.pop().unwrap() {
                if w1.parse::<usize>().is_ok() && w2.parse::<usize>().is_ok() {
                    datastack.push(StackElement::Word(
                        match w1.parse::<usize>().unwrap() > w2.parse::<usize>().unwrap() {
                            true => "t".to_string(),
                            false => "f".to_string(),
                        },
                    ));
                    return Ok(Interpreter {
                        datastack,
                        callstack: self.callstack.clone(),
                        dictionary: self.dictionary.clone(),
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

    fn less_than(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        if self.datastack.len() < 2 {
            return Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} not enough operands", "Error:".red().bold()),
            ));
        }
        if let StackElement::Word(w1) = datastack.pop().unwrap() {
            if let StackElement::Word(w2) = datastack.pop().unwrap() {
                if w1.parse::<usize>().is_ok() && w2.parse::<usize>().is_ok() {
                    datastack.push(StackElement::Word(
                        match w1.parse::<usize>().unwrap() < w2.parse::<usize>().unwrap() {
                            true => "t".to_string(),
                            false => "f".to_string(),
                        },
                    ));
                    return Ok(Interpreter {
                        datastack,
                        callstack: self.callstack.clone(),
                        dictionary: self.dictionary.clone(),
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

    fn equals(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        if self.datastack.len() < 2 {
            return Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} not enough operands", "Error:".red().bold()),
            ));
        }
        if let StackElement::Word(w1) = datastack.pop().unwrap() {
            if let StackElement::Word(w2) = datastack.pop().unwrap() {
                if w1.parse::<usize>().is_ok() && w2.parse::<usize>().is_ok() {
                    datastack.push(StackElement::Word(
                        match w1.parse::<usize>().unwrap() == w2.parse::<usize>().unwrap() {
                            true => "t".to_string(),
                            false => "f".to_string(),
                        },
                    ));
                    return Ok(Interpreter {
                        datastack,
                        callstack: self.callstack.clone(),
                        dictionary: self.dictionary.clone(),
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

    fn less_equals(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        if self.datastack.len() < 2 {
            return Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} not enough operands", "Error:".red().bold()),
            ));
        }
        if let StackElement::Word(w1) = datastack.pop().unwrap() {
            if let StackElement::Word(w2) = datastack.pop().unwrap() {
                if w1.parse::<usize>().is_ok() && w2.parse::<usize>().is_ok() {
                    datastack.push(StackElement::Word(
                        match w1.parse::<usize>().unwrap() <= w2.parse::<usize>().unwrap() {
                            true => "t".to_string(),
                            false => "f".to_string(),
                        },
                    ));
                    return Ok(Interpreter {
                        datastack,
                        callstack: self.callstack.clone(),
                        dictionary: self.dictionary.clone(),
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

    fn greater_equals(&self) -> Result<Interpreter, Error> {
        let mut datastack = self.datastack.clone();
        if self.datastack.len() < 2 {
            return Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} not enough operands", "Error:".red().bold()),
            ));
        }
        if let StackElement::Word(w1) = datastack.pop().unwrap() {
            if let StackElement::Word(w2) = datastack.pop().unwrap() {
                if w1.parse::<usize>().is_ok() && w2.parse::<usize>().is_ok() {
                    datastack.push(StackElement::Word(
                        match w1.parse::<usize>().unwrap() >= w2.parse::<usize>().unwrap() {
                            true => "t".to_string(),
                            false => "f".to_string(),
                        },
                    ));
                    return Ok(Interpreter {
                        datastack,
                        callstack: self.callstack.clone(),
                        dictionary: self.dictionary.clone(),
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

    fn comment(&self) -> Result<Interpreter, Error> {
        let mut callstack = self.callstack.clone();
        callstack.append(
            &mut [
                StackElement::SubStack(
                    [
                        StackElement::Word("dup".to_string()),
                        StackElement::Word("top".to_string()),
                        StackElement::Word("rot".to_string()),
                        StackElement::Word("swap".to_string()),
                        StackElement::Word("pop".to_string()),
                        StackElement::Word("continue".to_string()),
                    ]
                    .to_vec(),
                ),
                StackElement::Word("call/cc".to_string()),
            ]
            .to_vec(),
        );

        Ok(Interpreter {
            datastack: self.datastack.clone(),
            callstack,
            dictionary: self.dictionary.clone(),
        })
    }

    fn load(&self) -> Result<Interpreter, Error> {
        let mut callstack = self.callstack.clone();
        callstack.append(
            &mut [
                StackElement::Word("slurp".to_string()),
                StackElement::Word("uncomment".to_string()),
                StackElement::Word("tokenize".to_string()),
            ]
            .to_vec(),
        );

        Ok(Interpreter {
            datastack: self.datastack.clone(),
            callstack,
            dictionary: self.dictionary.clone(),
        })
    }

    fn run(&self) -> Result<Interpreter, Error> {
        let mut callstack = self.callstack.clone();
        callstack.append(
            &mut [
                StackElement::Word("load".to_string()),
                StackElement::Word("call".to_string()),
            ]
            .to_vec(),
        );

        Ok(Interpreter {
            datastack: self.datastack.clone(),
            callstack,
            dictionary: self.dictionary.clone(),
        })
    }

    fn start(&self) -> Result<Interpreter, Error> {
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

        Ok(Interpreter {
            datastack: self.datastack.clone(),
            callstack,
            dictionary: self.dictionary.clone(),
        })
    }
}
