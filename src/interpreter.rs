use std::{
    collections::BTreeMap,
    env,
    fs::{self, OpenOptions},
    io::{stdin, stdout, Error, Write},
    ops::Deref,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    preprocessor::call_fn_step_4,
    stack_element::{map_to_dict, BuiltIn, Funct, StackElement},
};

#[derive(Clone)]
pub struct Interpreter {
    pub datastack: Vec<StackElement>,
    pub callstack: Vec<StackElement>,
    pub dictionary: Rc<BTreeMap<String, Rc<Funct>>>,
}

impl Interpreter {
    pub fn new(
        datastack: Vec<StackElement>,
        callstack: Vec<StackElement>,
        dictionary: Rc<BTreeMap<String, Rc<Funct>>>,
    ) -> Self {
        Self {
            datastack,
            callstack,
            dictionary,
        }
    }

    pub fn insert(dictionary: &mut BTreeMap<String, Rc<Funct>>, str: &str, f: BuiltIn) {
        dictionary.insert(str.to_string(), Rc::new(Funct::BuiltIn(f)));
    }

    pub fn init_dictionary() -> BTreeMap<String, Rc<Funct>> {
        let mut dict: BTreeMap<String, Rc<Funct>> = BTreeMap::new();

        Self::insert(&mut dict, "swap", Rc::new(Self::swap));
        Self::insert(&mut dict, "dup", Rc::new(Self::dup));
        Self::insert(&mut dict, "drop", Rc::new(Self::drop));
        Self::insert(&mut dict, "rot", Rc::new(Self::rot));
        Self::insert(&mut dict, "type", Rc::new(Self::r#type));
        Self::insert(&mut dict, "equal?", Rc::new(Self::equal));
        Self::insert(&mut dict, "identical?", Rc::new(Self::identical));
        Self::insert(&mut dict, "emptystack", Rc::new(Self::emptystack));
        Self::insert(&mut dict, "push", Rc::new(Self::push));
        Self::insert(&mut dict, "top", Rc::new(Self::top));
        Self::insert(&mut dict, "pop", Rc::new(Self::pop));
        Self::insert(&mut dict, "concat", Rc::new(Self::concat));
        Self::insert(&mut dict, "reverse", Rc::new(Self::reverse));
        Self::insert(&mut dict, "mapping", Rc::new(Self::mapping));
        Self::insert(&mut dict, "unmap", Rc::new(Self::unmap));
        Self::insert(&mut dict, "keys", Rc::new(Self::keys));
        Self::insert(&mut dict, "assoc", Rc::new(Self::assoc));
        Self::insert(&mut dict, "dissoc", Rc::new(Self::dissoc));
        Self::insert(&mut dict, "get", Rc::new(Self::get));
        Self::insert(&mut dict, "merge", Rc::new(Self::merge));
        Self::insert(&mut dict, "word", Rc::new(Self::word));
        Self::insert(&mut dict, "unword", Rc::new(Self::unword));
        Self::insert(&mut dict, "char", Rc::new(Self::char));
        Self::insert(&mut dict, "print", Rc::new(Self::print));
        Self::insert(&mut dict, "flush", Rc::new(Self::flush));
        Self::insert(&mut dict, "read-line", Rc::new(Self::read_line));
        Self::insert(&mut dict, "slurp", Rc::new(Self::slurp));
        Self::insert(&mut dict, "spit", Rc::new(Self::spit));
        Self::insert(&mut dict, "spit-on", Rc::new(Self::spit_on));
        Self::insert(&mut dict, "uncomment", Rc::new(Self::uncomment));
        Self::insert(&mut dict, "tokenize", Rc::new(Self::tokenize));
        Self::insert(&mut dict, "undocument", Rc::new(Self::undocument));
        Self::insert(&mut dict, "current-time-millis", Rc::new(Self::ctm));
        Self::insert(&mut dict, "operating-system", Rc::new(Self::os));
        Self::insert(&mut dict, "call", Rc::new(Self::call));
        Self::insert(&mut dict, "call/cc", Rc::new(Self::call_cc));
        Self::insert(&mut dict, "continue", Rc::new(Self::r#continue));
        Self::insert(&mut dict, "get-dict", Rc::new(Self::get_dict));
        Self::insert(&mut dict, "set-dict", Rc::new(Self::set_dict));
        Self::insert(&mut dict, "stepcc", Rc::new(Self::stepcc));
        Self::insert(&mut dict, "apply", Rc::new(Self::apply));
        Self::insert(&mut dict, "compose", Rc::new(Self::compose));
        Self::insert(&mut dict, "func", Rc::new(Self::func));
        Self::insert(&mut dict, "integer?", Rc::new(Self::integer));
        Self::insert(&mut dict, "+", Rc::new(Self::addition));
        Self::insert(&mut dict, "-", Rc::new(Self::subtraction));
        Self::insert(&mut dict, "*", Rc::new(Self::multiplication));
        Self::insert(&mut dict, "div", Rc::new(Self::division));
        Self::insert(&mut dict, "mod", Rc::new(Self::modulo));
        Self::insert(&mut dict, "<", Rc::new(Self::less_than));
        Self::insert(&mut dict, ">", Rc::new(Self::greater_than));
        Self::insert(&mut dict, "==", Rc::new(Self::equals));
        Self::insert(&mut dict, "<=", Rc::new(Self::less_equals));
        Self::insert(&mut dict, ">=", Rc::new(Self::greater_equals));
        Self::insert(&mut dict, "\\", Rc::new(Self::comment));
        Self::insert(&mut dict, "load", Rc::new(Self::load));
        Self::insert(&mut dict, "run", Rc::new(Self::run));
        Self::insert(&mut dict, "start", Rc::new(Self::start));

        dict
    }

    pub fn dup(mut self) -> Self {
        let a = self.datastack.pop().unwrap();
        self.datastack.push(a.clone());
        self.datastack.push(a);

        self
    }

    pub fn swap(mut self) -> Self {
        let a = self.datastack.pop().unwrap();
        let b = self.datastack.pop().unwrap();
        self.datastack.push(a);
        self.datastack.push(b);

        self
    }

    pub fn drop(mut self) -> Self {
        self.datastack.pop().unwrap();

        self
    }

    pub fn rot(mut self) -> Self {
        let a = self.datastack.pop().unwrap();
        let b = self.datastack.pop().unwrap();
        let c = self.datastack.pop().unwrap();

        self.datastack.push(b);
        self.datastack.push(a);
        self.datastack.push(c);

        self
    }

    pub fn emptystack(mut self) -> Self {
        self.datastack.push(StackElement::SubStack(Vec::new()));
        self
    }

    pub fn push(mut self) -> Self {
        let e = self.datastack.pop().unwrap();
        let x = self.datastack.pop().unwrap();

        if let StackElement::SubStack(mut ss) = x {
            ss.push(e);
            self.datastack.push(StackElement::SubStack(ss));
            return self;
        }
        panic!("I can only push to stacks, received {:?}", x)
    }

    pub fn r#type(mut self) -> Self {
        match self.datastack.pop().unwrap() {
            StackElement::SubStack(_) => self.datastack.push(StackElement::Word("stk".to_string())),
            StackElement::Word(_) => self.datastack.push(StackElement::Word("wrd".to_string())),
            StackElement::Map(_) => self.datastack.push(StackElement::Word("map".to_string())),
            StackElement::Nil => self.datastack.push(StackElement::Word("nil".to_string())),
            StackElement::Fun(_) => self.datastack.push(StackElement::Word("fct".to_string())),
        };

        self
    }

    pub fn equal(mut self) -> Self {
        let a = self.datastack.pop().unwrap();
        let b = self.datastack.pop().unwrap();

        if a == b {
            self.datastack.push(StackElement::Word("t".to_string()));
        } else {
            self.datastack.push(StackElement::Word("f".to_string()));
        }

        self
    }

    pub fn identical(self) -> Self {
        unimplemented!()
    }

    pub fn pop(mut self) -> Self {
        let substack = self.datastack.pop().unwrap();

        match substack {
            StackElement::SubStack(mut ss) => {
                ss.pop().unwrap_or(StackElement::Nil);
                self.datastack.push(StackElement::SubStack(ss));
                self
            }
            _ => panic!("I can only pop from stacks, received {:?}", substack),
        }
    }

    pub fn top(mut self) -> Self {
        let substack = self.datastack.pop().unwrap();
        match substack {
            StackElement::SubStack(mut ss) => {
                let el = ss.pop().unwrap_or(StackElement::Nil);
                self.datastack.push(el);
                self
            }
            _ => panic!("I can only pop from stacks"),
        }
    }

    pub fn concat(mut self) -> Self {
        if let StackElement::SubStack(mut ss1) = self.datastack.pop().unwrap() {
            if let StackElement::SubStack(mut ss2) = self.datastack.pop().unwrap() {
                ss1.append(&mut ss2);
                self.datastack.push(StackElement::SubStack(ss1))
            } else {
                panic!("I can only concat stacks");
            }
        } else {
            panic!("I can only concat stacks");
        }

        self
    }

    pub fn reverse(mut self) -> Self {
        if let StackElement::SubStack(ss) = self.datastack.pop().unwrap() {
            let ss_rev = ss.into_iter().rev().collect();
            self.datastack.push(StackElement::SubStack(ss_rev))
        } else {
            panic!("I can only reverse stacks");
        }
        self
    }

    pub fn mapping(mut self) -> Self {
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
                panic!("not enough values for every key");
            }

            let mut map = Vec::new();

            for i in 0..keys.len() {
                map.push((keys[i].to_owned(), values[i].to_owned()));
            }

            self.datastack.push(StackElement::Map(map));
            return self;
        }

        panic!("need map to create mapping")
    }

    pub fn unmap(mut self) -> Self {
        if let StackElement::Map(map) = self.datastack.pop().unwrap() {
            let keys: Vec<StackElement> = map.iter().map(|(i, _)| i).cloned().collect();
            let values: Vec<StackElement> = map.into_iter().map(|(_, i)| i).collect();
            let mut st = Vec::new();
            for i in 0..keys.len() {
                st.push(values[i].to_owned());
                st.push(keys[i].to_owned());
            }

            self.datastack.push(StackElement::SubStack(st));

            return self;
        }
        panic!("need map to unmap")
    }

    pub fn keys(mut self) -> Self {
        if let StackElement::Map(map) = self.datastack.pop().unwrap() {
            let keys = map
                .into_iter()
                .map(|(i, _)| i)
                .collect::<Vec<StackElement>>();
            self.datastack.push(StackElement::SubStack(keys));

            return self;
        }
        panic!("need map to list keys")
    }

    pub fn assoc(mut self) -> Self {
        if let StackElement::Map(mut map) = self.datastack.pop().unwrap() {
            let key = self.datastack.pop().unwrap();
            let value = self.datastack.pop().unwrap();

            if !map.iter().any(|(i, _)| *i == key) {
                map.push((key, value));
            }

            self.datastack.push(StackElement::Map(map));

            return self;
        }

        panic!("need map to assoc value to map")
    }

    pub fn dissoc(mut self) -> Self {
        if let StackElement::Map(mut map) = self.datastack.pop().unwrap() {
            let key = self.datastack.pop().unwrap();
            map.retain(|(i, _)| *i != key);

            self.datastack.push(StackElement::Map(map));

            return self;
        }

        panic!("need map to dissoc value from map")
    }

    pub fn get(mut self) -> Self {
        let default = self.datastack.pop().unwrap();
        if let StackElement::Map(m) = self.datastack.pop().unwrap() {
            let key = self.datastack.pop().unwrap();

            self.datastack
                .push(match m.iter().find(|(i, _)| *i == key) {
                    Some((_, j)) => match j {
                        StackElement::Fun(func) => match func.deref() {
                            Funct::BuiltIn(_) => j.to_owned(),
                            Funct::SelfDefined(st) => st.to_owned(),
                        },
                        _ => j.to_owned(),
                    },
                    None => default,
                });

            return self;
        }

        panic!("need map to get value from map")
    }

    pub fn merge(mut self) -> Self {
        if let StackElement::Map(mut m1) = self.datastack.pop().unwrap() {
            if let StackElement::Map(m2) = self.datastack.pop().unwrap() {
                m2.into_iter().for_each(|(k, v)| {
                    if !m1.iter().any(|(i, _)| *i == k) {
                        m1.push((k, v));
                    }
                });

                self.datastack.push(StackElement::Map(m1));

                return self;
            }
        }

        panic!("need maps to merge maps")
    }

    pub fn word(mut self) -> Self {
        if let StackElement::SubStack(st) = self.datastack.pop().unwrap() {
            if let Ok(s) = st
                .iter()
                .map(|e| {
                    if let StackElement::Word(str) = e {
                        Ok(str.as_str())
                    } else {
                        panic!("stack may only contain words, found {}", e)
                    }
                })
                .rev()
                .collect::<Result<String, Error>>()
            {
                self.datastack.push(StackElement::Word(s));
                return self;
            }
        }

        panic!("need stack to use 'word'")
    }

    pub fn unword(mut self) -> Self {
        if let StackElement::Word(str) = self.datastack.pop().unwrap() {
            self.datastack.push(StackElement::SubStack(
                str.chars()
                    .map(|c| StackElement::Word(c.to_string()))
                    .rev()
                    .collect(),
            ));

            return self;
        }

        panic!("need word to use 'unword'")
    }

    pub fn char(mut self) -> Self {
        if let StackElement::Word(w) = self.datastack.pop().unwrap() {
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
                        char::from_u32(u32::from_str_radix(&ss, 16).unwrap())
                            .unwrap()
                            .to_string()
                    } else {
                        panic!("invalid utf string");
                    }
                }
            }));

            return self;
        }
        panic!("need word to parse to utf")
    }

    pub fn print(mut self) -> Self {
        print!("{}", self.datastack.pop().unwrap());
        //self.datastack.pop().unwrap();

        self
    }

    pub fn flush(self) -> Self {
        stdout().flush().unwrap();

        self
    }

    pub fn read_line(mut self) -> Self {
        let mut inp = "".to_string();
        stdin().read_line(&mut inp).unwrap();

        self.datastack.push(StackElement::Word(inp));
        self
    }

    pub fn slurp(mut self) -> Self {
        if let StackElement::Word(src) = self.datastack.pop().unwrap() {
            self.datastack
                .push(StackElement::Word(fs::read_to_string(src).unwrap()));
            return self;
        }

        panic!("need word to read from file")
    }

    pub fn spit(mut self) -> Self {
        if let StackElement::Word(data) = self.datastack.pop().unwrap() {
            if let StackElement::Word(file) = self.datastack.pop().unwrap() {
                fs::write(file, data).unwrap();
                return self;
            }
        }

        panic!("could not write file")
    }

    pub fn spit_on(mut self) -> Self {
        if let StackElement::Word(data) = self.datastack.pop().unwrap() {
            if let StackElement::Word(path) = self.datastack.pop().unwrap() {
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(path)
                    .unwrap();

                file.write_all(data.as_bytes()).unwrap();
                return self;
            }
        }

        panic!("could not write file")
    }

    pub fn uncomment(mut self) -> Self {
        let debug = self.datastack[self.datastack.len() - 1].clone();
        if let StackElement::Word(wrd) = self.datastack.pop().unwrap() {
            self.datastack.push(StackElement::Word(
                wrd.lines()
                    .map(|l| l.split('%').next().unwrap())
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join(" "),
            ));

            return self;
        }

        panic!("I can only uncomment words, found {:?}", debug)
    }

    pub fn tokenize(mut self) -> Self {
        if let StackElement::Word(w) = self.datastack.pop().unwrap() {
            self.datastack.push(StackElement::SubStack(
                w.split_whitespace()
                    .map(|s| StackElement::Word(s.to_string()))
                    .rev()
                    .collect(),
            ));
            return self;
        }

        panic!("need word to tokenize")
    }

    pub fn undocument(self) -> Self {
        unimplemented!()
    }

    pub fn ctm(mut self) -> Self {
        self.datastack.push(StackElement::Word(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis()
                .to_string(),
        ));

        self
    }

    pub fn os(mut self) -> Self {
        self.datastack
            .push(StackElement::Word(env::consts::OS.to_string()));

        self
    }

    pub fn call(mut self) -> Self {
        match self.datastack.pop().unwrap() {
            StackElement::SubStack(mut st) => self.callstack.append(&mut st),
            StackElement::Word(w) => self.callstack.push(StackElement::Word(w)),
            StackElement::Fun(f) => match f.deref() {
                Funct::BuiltIn(bi) => return bi(self),
                Funct::SelfDefined(_) => unimplemented!(),
            },
            _ => unimplemented!(),
        }

        self
    }

    pub fn call_cc(mut self) -> Self {
        let top = self.datastack.pop().unwrap();
        let mut new_callstack = Vec::new();
        match top {
            StackElement::SubStack(mut ss) => new_callstack.append(&mut ss),
            el => new_callstack.push(el),
        }

        self.datastack = vec![
            StackElement::SubStack(self.datastack),
            StackElement::SubStack(self.callstack),
        ];

        Self::new(self.datastack, new_callstack, self.dictionary)
    }

    pub fn call_cc_after_preprocess_step_4(mut self) -> Self {
        let top = self.datastack.pop().unwrap();

        self.datastack = vec![
            StackElement::SubStack(self.datastack),
            StackElement::SubStack(self.callstack),
        ];

        match top {
            StackElement::SubStack(ss) => {
                call_fn_step_4("asdfg".to_string(), &ss, &self.dictionary)(Self::new(
                    self.datastack,
                    Vec::new(),
                    self.dictionary,
                ))
            }

            _ => unimplemented!(),
        }
    }

    pub fn r#continue(mut self) -> Self {
        if let StackElement::SubStack(new_callstack) = self.datastack.pop().unwrap() {
            if let StackElement::SubStack(new_datastack) = self.datastack.pop().unwrap() {
                return Self::new(new_datastack, new_callstack, self.dictionary);
            }
        }

        panic!("need quotation for continue")
    }

    pub fn get_dict(mut self) -> Self {
        let mut map = Vec::new();
        self.dictionary.clone().iter().for_each(|(k, v)| {
            let key = StackElement::Word(k.to_owned());
            let value = StackElement::Fun(v.to_owned());
            if !map.iter().map(|(i, _)| i).any(|se| key == *se) {
                map.push((key, value));
            }
        });

        self.datastack.push(StackElement::Map(map));

        self
    }

    pub fn set_dict(mut self) -> Self {
        if let StackElement::Map(dict) = self.datastack.pop().unwrap() {
            let dict = map_to_dict(&dict).unwrap();
            return Self::new(self.datastack, self.callstack, Rc::new(dict));
        }

        panic!("need map for set-dict")
    }

    pub fn stepcc(mut self) -> Self {
        let e = self.callstack.pop().unwrap();

        match e {
            StackElement::SubStack(ss) => self.datastack.push(StackElement::SubStack(ss)),
            StackElement::Word(w) => {
                match self.dictionary.clone().get(&w) {
                    Some(fun) => match fun.deref() {
                        Funct::BuiltIn(fct) => {
                            return fct(self);
                        }
                        Funct::SelfDefined(stack) => {
                            if let StackElement::SubStack(mut ss) = stack.to_owned() {
                                self.callstack.append(&mut ss);
                            }
                        }
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
                    return bi(self);
                }
                Funct::SelfDefined(sd) => {
                    if let StackElement::SubStack(mut ss) = sd.to_owned() {
                        self.callstack.append(&mut ss)
                    }
                }
            },
        };

        self
    }

    pub fn apply(mut self) -> Self {
        if let StackElement::Fun(fun) = self.datastack.pop().unwrap() {
            if let StackElement::SubStack(stack) = self.datastack.pop().unwrap() {
                let int = match fun.deref() {
                    Funct::BuiltIn(bi) => bi(Self::new(
                        stack,
                        self.callstack.clone(),
                        self.dictionary.clone(),
                    )),
                    Funct::SelfDefined(_) => unimplemented!(),
                };
                self.datastack.push(StackElement::SubStack(int.datastack));
                self.dictionary = int.dictionary;
            }
        }

        self
    }

    pub fn compose(self) -> Self {
        unimplemented!()
    }

    pub fn func(mut self) -> Self {
        if let StackElement::Map(_dict) = self.datastack.pop().unwrap() {
            if let StackElement::SubStack(qt) = self.datastack.pop().unwrap() {
                fn runcc(inner: Interpreter) -> Interpreter {
                    let mut int = inner;
                    while !int.callstack.is_empty() {
                        int = int.stepcc();
                    }

                    int
                }

                let f = move |interpreter: Interpreter| {
                    let qt = qt.to_owned();
                    runcc(Self::new(interpreter.datastack, qt, interpreter.dictionary))
                };

                self.datastack
                    .push(StackElement::Fun(Rc::new(Funct::BuiltIn(Rc::new(f)))));
            }
        }

        self
    }

    pub fn func_after_preprocess_step_4(mut self) -> Self {
        if let StackElement::Map(_dict) = self.datastack.pop().unwrap() {
            if let StackElement::SubStack(qt) = self.datastack.pop().unwrap() {
                let f = call_fn_step_4("asdfgh".to_string(), &qt, &self.dictionary);

                self.datastack
                    .push(StackElement::Fun(Rc::new(Funct::BuiltIn(f))));
            }
        }

        self
    }

    pub fn integer(mut self) -> Self {
        let e = self.datastack.pop().unwrap();
        self.datastack.push(match e {
            StackElement::Word(w) => match w.parse::<usize>() {
                Ok(_) => StackElement::Word("t".to_string()),
                Err(_) => StackElement::Word("f".to_string()),
            },
            _ => StackElement::Word("f".to_string()),
        });

        self
    }

    pub fn addition(self) -> Self {
        self.binary("+")
    }

    pub fn subtraction(self) -> Self {
        self.binary("-")
    }

    pub fn multiplication(self) -> Self {
        self.binary("*")
    }

    pub fn division(self) -> Self {
        self.binary("div")
    }

    pub fn modulo(self) -> Self {
        self.binary("mod")
    }

    pub fn greater_than(self) -> Self {
        self.binary(">")
    }

    pub fn less_than(self) -> Self {
        self.binary("<")
    }

    pub fn equals(self) -> Self {
        self.binary("==")
    }

    pub fn less_equals(self) -> Self {
        self.binary("<=")
    }

    pub fn greater_equals(self) -> Self {
        self.binary(">=")
    }

    fn binary(mut self, op: &str) -> Self {
        if let StackElement::Word(w1) = self.datastack.pop().unwrap() {
            if let StackElement::Word(w2) = self.datastack.pop().unwrap() {
                let x: isize = w1.parse().unwrap();
                let y: isize = w2.parse().unwrap();
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
                return self;
            }
        }
        panic!("need integers for binary operations")
    }

    pub fn comment(mut self) -> Self {
        let next = self.callstack.pop().unwrap();
        self.datastack.push(next);

        self
    }

    pub fn comment_after_preprocess(self) -> Self {
        panic!("das machen wir hier nicht mehr")
    }

    pub fn call_after_preproccess_step_4(mut self) -> Self {
        match self.datastack.pop().unwrap() {
            StackElement::SubStack(st) => {
                call_fn_step_4("asdfgh".to_string(), st.as_slice(), &self.dictionary)(self)
            }
            StackElement::Fun(f) => match f.deref() {
                Funct::BuiltIn(bi) => bi(self),
                _ => unimplemented!(),
            },
            StackElement::Word(f) => call_fn_step_4(
                "qwert".to_string(),
                &[StackElement::Word(f)],
                &self.dictionary,
            )(self),
            x => panic!("{:?}", x),
        }
    }

    pub fn load(self) -> Self {
        self.slurp().uncomment().tokenize()
    }

    pub fn run(self) -> Self {
        self.load().call()
    }

    pub fn run_after_preprocess_step_4(self) -> Self {
        self.load().call_after_preproccess_step_4()
    }

    pub fn start(self) -> Self {
        self.apply()
            .swap()
            .emptystack()
            .func()
            .get_dict()
            .tokenize()
            .uncomment()
            .slurp()
    }
}
