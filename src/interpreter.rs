use std::{collections::{BTreeMap, HashMap}, io::{self, stdin, stdout, Error, Write}};

use colored::Colorize;

use crate::stack_element::StackElement;

type BuiltIn = Box<dyn FnMut(&mut Interpreter) -> Result<(), Error>>;

pub struct Interpreter<> {
    datastack: Vec<StackElement>,
    pub callstack: Vec<StackElement>,
    pub dictionary: HashMap<String, BuiltIn>
}

impl Interpreter {
    pub fn new() -> Self {
        let ret = Interpreter {
            datastack: Vec::new(),
            callstack: Vec::new(),
            dictionary: Self::init_dictionary(),
        };

        ret
    }

    fn init_dictionary() -> HashMap<String, BuiltIn> {
        let mut dict: HashMap<String, BuiltIn> = HashMap::new();

        dict.insert("swap".to_string(), Box::new(Self::swap));
        dict.insert("dup".to_string(), Box::new(Self::dup));
        dict.insert("drop".to_string(), Box::new(Self::drop));
        dict.insert("rot".to_string(), Box::new(Self::rot));
        dict.insert("type".to_string(), Box::new(Self::r#type));
        dict.insert("equal?".to_string(), Box::new(Self::equal));
        dict.insert("identical?".to_string(), Box::new(Self::identical));
        dict.insert("emptystack".to_string(), Box::new(Self::emptystack));
        dict.insert("push".to_string(), Box::new(Self::push));
        dict.insert("top".to_string(), Box::new(Self::top));
        dict.insert("pop".to_string(), Box::new(Self::pop));
        dict.insert("concat".to_string(), Box::new(Self::concat));
        dict.insert("reverse".to_string(), Box::new(Self::reverse));
        dict.insert("mapping".to_string(), Box::new(Self::mapping));
        dict.insert("unmap".to_string(), Box::new(Self::unmap));
        dict.insert("keys".to_string(), Box::new(Self::keys));
        dict.insert("assoc".to_string(), Box::new(Self::assoc));
        dict.insert("dissoc".to_string(), Box::new(Self::dissoc));
        dict.insert("get".to_string(), Box::new(Self::get));
        dict.insert("merge".to_string(), Box::new(Self::merge));
        dict.insert("word".to_string(), Box::new(Self::word));
        dict.insert("unword".to_string(), Box::new(Self::unword));
        dict.insert("char".to_string(), Box::new(Self::char));
        dict.insert("print".to_string(), Box::new(Self::print));
        dict.insert("flush".to_string(), Box::new(Self::flush));
        dict.insert("read-line".to_string(), Box::new(Self::read_line));
        dict.insert("slurp".to_string(), Box::new(Self::slurp));
        dict.insert("spit".to_string(), Box::new(Self::spit));
        dict.insert("spit-on".to_string(), Box::new(Self::spit_on));
        dict.insert("uncomment".to_string(), Box::new(Self::uncomment));
        dict.insert("tokenize".to_string(), Box::new(Self::tokenize));
        dict.insert("undocument".to_string(), Box::new(Self::undocument));
        dict.insert("current-time-millis".to_string(), Box::new(Self::current_time_millis));
        dict.insert("operating-system".to_string(), Box::new(Self::operating_system));
        dict.insert("call".to_string(), Box::new(Self::call));
        dict.insert("call/cc".to_string(), Box::new(Self::call_cc));
        dict.insert("continue".to_string(), Box::new(Self::r#continue));
        dict.insert("get-dict".to_string(), Box::new(Self::get_dict));
        dict.insert("set-dict".to_string(), Box::new(Self::set_dict));
        dict.insert("stepcc".to_string(), Box::new(Self::stepcc));
        dict.insert("apply".to_string(), Box::new(Self::apply));
        dict.insert("compose".to_string(), Box::new(Self::compose));
        dict.insert("func".to_string(), Box::new(Self::func));
        dict.insert("integer?".to_string(), Box::new(Self::integer));
        dict.insert("+".to_string(), Box::new(Self::addition));
        dict.insert("-".to_string(), Box::new(Self::subtraction));
        dict.insert("*".to_string(), Box::new(Self::multiplication));
        dict.insert("div".to_string(), Box::new(Self::division));
        dict.insert("mod".to_string(), Box::new(Self::modulo));
        dict.insert("<".to_string(), Box::new(Self::less_than));
        dict.insert(">".to_string(), Box::new(Self::greater_than));
        dict.insert("==".to_string(), Box::new(Self::equals));
        dict.insert("<=".to_string(), Box::new(Self::less_equals));
        dict.insert(">=".to_string(), Box::new(Self::greater_equals));
        dict.insert("\\".to_string(), Box::new(Self::comment));
        dict.insert("load".to_string(), Box::new(Self::load));
        dict.insert("run".to_string(), Box::new(Self::run));
        dict.insert("start".to_string(), Box::new(Self::start));


        dict
    }

    fn dup(&mut self) -> Result<(), Error>{
        if self.datastack.is_empty() {
            return Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} not enough operands", "Error:".red().bold()),
            ));
        }
        let a = self.datastack.pop().unwrap();
        self.datastack.push(a.clone());
        self.datastack.push(a);
        Ok(())
    }


    fn swap(&mut self) -> Result<(), Error> {
        if self.datastack.len() > 2 {
            let a = self.datastack.pop().unwrap();
            let b = self.datastack.pop().unwrap();
            self.datastack.push(a.clone());
            self.datastack.push(b.clone());
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} not enough operands", "Error:".red().bold()),
        ))
    }


    fn drop(&mut self) -> Result<(), Error>{
        if self.datastack.is_empty() {
            return Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} not enough operands", "Error:".red().bold()),
            ));
        }
        self.datastack.pop().unwrap();

        Ok(())
    }


    fn rot(&mut self) -> Result<(), Error> {
        if self.datastack.len() < 3 {
            return Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} not enough operands", "Error:".red().bold()),
            ));
        }

        let a = self.datastack.pop().unwrap();
        let b = self.datastack.pop().unwrap();
        let c = self.datastack.pop().unwrap();

        self.datastack.push(b);
        self.datastack.push(a);
        self.datastack.push(c);

        Ok(())
    }


    fn emptystack(&mut self) -> Result<(), Error> {
        self.datastack.push(StackElement::SubStack(Vec::new()));
        Ok(())
    }


    fn push(&mut self) -> Result<(), Error> {
        let e = self.datastack.pop().unwrap();

        if let StackElement::SubStack(ss) = self.datastack.pop().unwrap() {
            let mut new_substack = ss.clone();
            new_substack.push(e);
            self.datastack.push(StackElement::SubStack(new_substack));
            return Ok(());
        }
        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} I can only push to stacks", "Error:".red().bold()),
        ))
    }


    fn r#type(&mut self) -> Result<(), Error> {
        match self.datastack.pop().unwrap() {
            StackElement::SubStack(_) => self.datastack.push(StackElement::Word("stk".to_string())),
            StackElement::Word(_) => self.datastack.push(StackElement::Word("wrd".to_string())),
            StackElement::Keyword(_) => self.datastack.push(StackElement::Word("fct".to_string())),
            StackElement::Map(_) => self.datastack.push(StackElement::Word("map".to_string())),
        };

        Ok(())
    }

    fn equal(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn identical(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn pop(&mut self) -> Result<(), Error> {
        let substack = self.datastack.pop().unwrap();
        match substack {
            StackElement::SubStack(ss) => {
                let mut new_ss = ss.clone();
                new_ss.pop().unwrap();
                self.datastack.push(StackElement::SubStack(new_ss));
                Ok(())
            }
            _ => Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} I can only pop from stacks", "Error:".red().bold()),
            )),
        }
    }


    fn top(&mut self) -> Result<(), Error> {
        let substack = self.datastack.pop().unwrap();
        match substack {
            StackElement::SubStack(ss) => {
                let mut new_ss = ss.clone();
                let el = new_ss.pop().unwrap();
                self.datastack.push(el);
                Ok(())
            }
            _ => Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} I can only pop from stacks", "Error:".red().bold()),
            )),
        }
    }


    fn concat(&mut self) -> Result<(), Error> {
        if let StackElement::SubStack(mut ss1) = self.datastack.pop().unwrap() {
            if let StackElement::SubStack(mut ss2) = self.datastack.pop().unwrap() {
                ss2.append(&mut ss1);
                self.datastack.push(StackElement::SubStack(ss2))
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


    fn reverse(&mut self) -> Result<(), Error> {
        if let StackElement::SubStack(mut ss) = self.datastack.pop().unwrap() {
            ss.reverse();
            self.datastack.push(StackElement::SubStack(ss))
        } else {
            return Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} I can only reverse stacks", "Error:".red().bold()),
            ));
        }
        Ok(())
    }


    fn mapping(&mut self) -> Result<(), Error> {
        if let StackElement::SubStack(ss) = self.datastack.pop().unwrap() {
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

            self.datastack.push(StackElement::Map(map));
            return Ok(());
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need map to create mapping", "Error:".red().bold()),
        ))
    }


    fn unmap(&mut self) -> Result<(), Error> {
        if let StackElement::Map(map) = self.datastack.pop().unwrap() {
            let keys = map.keys().cloned().collect::<Vec<StackElement>>();
            let values = map.values().cloned().collect::<Vec<StackElement>>();
            let mut st = Vec::new();
            for i in 0..keys.len() {
                st.push(keys[i].clone());
                st.push(values[i].clone());
            }

            self.datastack.push(StackElement::SubStack(st));

            return Ok(());
        }
        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need map to unmap", "Error:".red().bold()),
        ))
    }


    fn keys(&mut self) -> Result<(), Error> {
        if let StackElement::Map(map) = self.datastack.pop().unwrap() {
            let keys = map.keys().cloned().collect::<Vec<StackElement>>();
            self.datastack.push(StackElement::SubStack(keys));

            return Ok(());
        }
        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need map to list keys", "Error:".red().bold()),
        ))
    }


    fn assoc(&mut self) -> Result<(), Error> {
        if let StackElement::Map(mut map) = self.datastack.pop().unwrap() {
            let key = self.datastack.pop().unwrap();
            let value = self.datastack.pop().unwrap();
            map.insert(key, value);

            self.datastack.push(StackElement::Map(map));

            return Ok(());
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need map to assoc value to map", "Error:".red().bold()),
        ))
    }


    fn dissoc(&mut self) -> Result<(), Error> {
        if let StackElement::Map(mut map) = self.datastack.pop().unwrap() {
            let key = self.datastack.pop().unwrap();
            map.remove(&key);

            self.datastack.push(StackElement::Map(map));

            return Ok(());
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "{} need map to dissoc value from map",
                "Error:".red().bold()
            ),
        ))
    }


    fn get(&mut self) -> Result<(), Error> {
        let key = self.datastack.pop().unwrap();
        if let StackElement::Map(m) = self.datastack.pop().unwrap() {
            let default = self.datastack.pop().unwrap();

            self.datastack.push(m.clone().get(&key).unwrap_or(&default).clone());

            return Ok(());
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need map to get value from map", "Error:".red().bold()),
        ))
    }


    fn merge(&mut self) -> Result<(), Error> {
        if let StackElement::Map(mut m1) = self.datastack.pop().unwrap() {
            if let StackElement::Map(m2) = self.datastack.pop().unwrap() {
                m2.iter().for_each(|(k, v)| {
                    m1.insert(k.clone(), v.clone()).unwrap();
                });

                self.datastack.push(StackElement::Map(m1));

                return Ok(());
            }
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need maps to merge maps", "Error:".red().bold()),
        ))
    }


    fn word(&mut self) -> Result<(), Error> {
        if let StackElement::SubStack(st) = self.datastack.pop().unwrap() {
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
                self.datastack.push(StackElement::Word(s));
                return Ok(());
            }
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need stack to use 'word'", "Error:".red().bold()),
        ))
    }


    fn unword(&mut self) -> Result<(), Error> {
        if let StackElement::Word(str) = self.datastack.pop().unwrap() {
            self.datastack.push(StackElement::SubStack(
                str.chars()
                    .map(|c| StackElement::Word(c.to_string()))
                    .collect(),
            ));

            return Ok(());
        }

        Err(Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} need word to use 'unword'", "Error:".red().bold()),
        ))
    }

    fn char(&mut self) -> Result<(), Error> {
        todo!()
    }


    fn print(&mut self) -> Result<(), Error> {
        print!("{}", self.datastack.pop().unwrap());

        Ok(())
    }


    fn flush(&mut self) -> Result<(), Error> {
        stdout().flush().unwrap();

        Ok(())
    }

    
    fn read_line(&mut self) -> Result<(), Error>{
        let mut inp = "".to_string();
        stdin().read_line(&mut inp).unwrap();

        self.datastack.push(StackElement::Word(inp));
        Ok(())
    }

    fn slurp(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn spit(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn spit_on(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn uncomment(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn tokenize(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn undocument(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn current_time_millis(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn operating_system(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn call(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn call_cc(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn r#continue(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn get_dict(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn set_dict(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn stepcc(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn apply(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn compose(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn func(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn integer(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn addition(&mut self) -> Result<(), Error> {
        if self.datastack.len() < 2 {
            return Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} not enough operands", "Error:".red().bold()),
            ));
        }
        if let StackElement::Word(w1) = self.datastack.pop().unwrap() {
            if let StackElement::Word(w2) = self.datastack.pop().unwrap() {
                if w1.parse::<usize>().is_ok() && w2.parse::<usize>().is_ok() {
                    self.datastack.push(StackElement::Word(
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

    fn subtraction(&mut self) -> Result<(), Error>{
        if self.datastack.len() < 2 {
            return Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} not enough operands", "Error:".red().bold()),
            ));
        }
        if let StackElement::Word(w1) = self.datastack.pop().unwrap() {
            if let StackElement::Word(w2) = self.datastack.pop().unwrap() {
                if w1.parse::<usize>().is_ok() && w2.parse::<usize>().is_ok() {
                    self.datastack.push(StackElement::Word(
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

    fn multiplication(&mut self) -> Result<(), Error> {
        if self.datastack.len() < 2 {
            return Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} not enough operands", "Error:".red().bold()),
            ));
        }
        if let StackElement::Word(w1) = self.datastack.pop().unwrap() {
            if let StackElement::Word(w2) = self.datastack.pop().unwrap() {
                if w1.parse::<usize>().is_ok() && w2.parse::<usize>().is_ok() {
                    self.datastack.push(StackElement::Word(
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

    fn division(&mut self) -> Result<(), Error> {
        if self.datastack.len() < 2 {
            return Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} not enough operands", "Error:".red().bold()),
            ));
        }
        if let StackElement::Word(w1) = self.datastack.pop().unwrap() {
            if let StackElement::Word(w2) = self.datastack.pop().unwrap() {
                if w1.parse::<usize>().is_ok() && w2.parse::<usize>().is_ok() {
                    self.datastack.push(StackElement::Word(
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

    fn modulo(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn greater_than(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn less_than(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn equals(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn less_equals(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn greater_equals(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn comment(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn load(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn run(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn start(&mut self) -> Result<(), Error> {
        todo!()
    }

}