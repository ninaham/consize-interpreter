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

use crate::stack_element::{map_to_dict, BuiltIn, Funct, StackElement};

#[derive(Clone)]
pub struct Interpreter {
    pub datastack: Vec<StackElement>,
    pub callstack: Vec<StackElement>,
    pub dictionary: BTreeMap<String, Rc<Funct>>,
}

impl Interpreter {
    pub fn new(
        datastack: Vec<StackElement>,
        callstack: Vec<StackElement>,
        dictionary: BTreeMap<String, Rc<Funct>>,
    ) -> Self {
        Self {
            datastack,
            callstack,
            dictionary,
        }
    }

    fn pop_or_err(vec: &mut Vec<StackElement>, str: &str) -> Result<StackElement, Error> {
        vec.pop().ok_or(Self::error(str))
    }

    pub fn error(str: &str) -> Error {
        Error::new(
            io::ErrorKind::InvalidData,
            format!("{} {str}", "Error:".red().bold()),
        )
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

    pub fn dup(mut self) -> Result<Self, Error> {
        let a = Self::pop_or_err(&mut self.datastack, "not enough operands")?;
        self.datastack.push(a.clone());
        self.datastack.push(a);
        Ok(self)
    }

    pub fn swap(mut self) -> Result<Self, Error> {
        let a = Self::pop_or_err(&mut self.datastack, "not enough operands")?;
        let b = Self::pop_or_err(&mut self.datastack, "not enough operands")?;
        self.datastack.push(a);
        self.datastack.push(b);

        Ok(self)
    }

    pub fn drop(mut self) -> Result<Self, Error> {
        Self::pop_or_err(&mut self.datastack, "not enough operands")?;

        Ok(self)
    }

    pub fn rot(mut self) -> Result<Self, Error> {
        let a = Self::pop_or_err(&mut self.datastack, "not enough operands")?;
        let b = Self::pop_or_err(&mut self.datastack, "not enough operands")?;
        let c = Self::pop_or_err(&mut self.datastack, "not enough operands")?;

        self.datastack.push(b);
        self.datastack.push(a);
        self.datastack.push(c);

        Ok(self)
    }

    pub fn emptystack(mut self) -> Result<Self, Error> {
        self.datastack.push(StackElement::SubStack(Vec::new()));
        Ok(self)
    }

    pub fn push(mut self) -> Result<Self, Error> {
        let e = Self::pop_or_err(&mut self.datastack, "not enough operands")?;

        if let StackElement::SubStack(mut ss) =
            Self::pop_or_err(&mut self.datastack, "not enough operands")?
        {
            ss.push(e);
            self.datastack.push(StackElement::SubStack(ss));
            return Ok(self);
        }
        Err(Self::error("I can only push to stacks"))
    }

    pub fn r#type(mut self) -> Result<Self, Error> {
        match Self::pop_or_err(&mut self.datastack, "not enough operands")? {
            StackElement::SubStack(_) => self.datastack.push(StackElement::Word("stk".to_string())),
            StackElement::Word(_) => self.datastack.push(StackElement::Word("wrd".to_string())),
            StackElement::Map(_) => self.datastack.push(StackElement::Word("map".to_string())),
            StackElement::Nil => self.datastack.push(StackElement::Word("nil".to_string())),
            StackElement::Fun(_) => self.datastack.push(StackElement::Word("fct".to_string())),
        };

        Ok(self)
    }

    pub fn equal(mut self) -> Result<Self, Error> {
        let a = Self::pop_or_err(&mut self.datastack, "not enough operands")?;
        let b = Self::pop_or_err(&mut self.datastack, "not enough operands")?;

        if a == b {
            self.datastack.push(StackElement::Word("t".to_string()));
        } else {
            self.datastack.push(StackElement::Word("f".to_string()));
        }

        Ok(self)
    }

    pub fn identical(self) -> Result<Self, Error> {
        unimplemented!()
    }

    pub fn pop(mut self) -> Result<Self, Error> {
        let substack = Self::pop_or_err(&mut self.datastack, "not enough operands")?;

        match substack {
            StackElement::SubStack(mut ss) => {
                ss.pop().unwrap_or(StackElement::Nil);
                self.datastack.push(StackElement::SubStack(ss));
                Ok(self)
            }
            _ => Err(Self::error("I can only pop from stacks")),
        }
    }

    pub fn top(mut self) -> Result<Self, Error> {
        let substack = Self::pop_or_err(&mut self.datastack, "not enough operands")?;
        match substack {
            StackElement::SubStack(mut ss) => {
                let el = ss.pop().unwrap_or(StackElement::Nil);
                self.datastack.push(el);
                Ok(self)
            }
            _ => Err(Self::error("I can only pop from stacks")),
        }
    }

    pub fn concat(mut self) -> Result<Self, Error> {
        if let StackElement::SubStack(mut ss1) =
            Self::pop_or_err(&mut self.datastack, "not enough operands")?
        {
            if let StackElement::SubStack(mut ss2) =
                Self::pop_or_err(&mut self.datastack, "not enough operands")?
            {
                ss1.append(&mut ss2);
                self.datastack.push(StackElement::SubStack(ss1))
            } else {
                return Err(Self::error("I can only concat stacks"));
            }
        } else {
            return Err(Self::error("I can only concat stacks"));
        }

        Ok(self)
    }

    pub fn reverse(mut self) -> Result<Self, Error> {
        if let StackElement::SubStack(ss) =
            Self::pop_or_err(&mut self.datastack, "not enough operands")?
        {
            let ss_rev = ss.into_iter().rev().collect();
            self.datastack.push(StackElement::SubStack(ss_rev))
        } else {
            return Err(Self::error("I can only reverse stacks"));
        }
        Ok(self)
    }

    pub fn mapping(mut self) -> Result<Self, Error> {
        if let StackElement::SubStack(ss) =
            Self::pop_or_err(&mut self.datastack, "not enough operands")?
        {
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
                return Err(Self::error("not enough values for every key"));
            }

            let mut map = Vec::new();

            for i in 0..keys.len() {
                map.push((keys[i].to_owned(), values[i].to_owned()));
            }

            self.datastack.push(StackElement::Map(map));
            return Ok(self);
        }

        Err(Self::error("need map to create mapping"))
    }

    pub fn unmap(mut self) -> Result<Self, Error> {
        if let StackElement::Map(map) =
            Self::pop_or_err(&mut self.datastack, "not enough operands")?
        {
            let keys: Vec<StackElement> = map.iter().map(|(i, _)| i).cloned().collect();
            let values: Vec<StackElement> = map.into_iter().map(|(_, i)| i).collect();
            let mut st = Vec::new();
            for i in 0..keys.len() {
                st.push(values[i].to_owned());
                st.push(keys[i].to_owned());
            }

            self.datastack.push(StackElement::SubStack(st));

            return Ok(self);
        }
        Err(Self::error("need map to unmap"))
    }

    pub fn keys(mut self) -> Result<Self, Error> {
        if let StackElement::Map(map) =
            Self::pop_or_err(&mut self.datastack, "not enough operands")?
        {
            let keys = map
                .into_iter()
                .map(|(i, _)| i)
                .collect::<Vec<StackElement>>();
            self.datastack.push(StackElement::SubStack(keys));

            return Ok(self);
        }
        Err(Self::error("need map to list keys"))
    }

    pub fn assoc(mut self) -> Result<Self, Error> {
        if let StackElement::Map(mut map) =
            Self::pop_or_err(&mut self.datastack, "not enough operands")?
        {
            let key = Self::pop_or_err(&mut self.datastack, "not enough operands")?;
            let value = Self::pop_or_err(&mut self.datastack, "not enough operands")?;

            if !map.iter().any(|(i, _)| *i == key) {
                map.push((key, value));
            }

            self.datastack.push(StackElement::Map(map));

            return Ok(self);
        }

        Err(Self::error("need map to assoc value to map"))
    }

    pub fn dissoc(mut self) -> Result<Self, Error> {
        if let StackElement::Map(mut map) =
            Self::pop_or_err(&mut self.datastack, "not enough operands")?
        {
            let key = Self::pop_or_err(&mut self.datastack, "not enough operands")?;
            map.retain(|(i, _)| *i != key);

            self.datastack.push(StackElement::Map(map));

            return Ok(self);
        }

        Err(Self::error("need map to dissoc value from map"))
    }

    pub fn get(mut self) -> Result<Self, Error> {
        let default = Self::pop_or_err(&mut self.datastack, "not enough operands")?;
        if let StackElement::Map(m) = Self::pop_or_err(&mut self.datastack, "not enough operands")?
        {
            let key = Self::pop_or_err(&mut self.datastack, "not enough operands")?;

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

            return Ok(self);
        }

        Err(Self::error("need map to get value from map"))
    }

    pub fn merge(mut self) -> Result<Self, Error> {
        if let StackElement::Map(mut m1) =
            Self::pop_or_err(&mut self.datastack, "not enough operands")?
        {
            if let StackElement::Map(m2) =
                Self::pop_or_err(&mut self.datastack, "not enough operands")?
            {
                m2.into_iter().for_each(|(k, v)| {
                    if !m1.iter().any(|(i, _)| *i == k) {
                        m1.push((k, v));
                    }
                });

                self.datastack.push(StackElement::Map(m1));

                return Ok(self);
            }
        }

        Err(Self::error("need maps to merge maps"))
    }

    pub fn word(mut self) -> Result<Self, Error> {
        if let StackElement::SubStack(st) =
            Self::pop_or_err(&mut self.datastack, "not enough operands")?
        {
            if let Ok(s) = st
                .iter()
                .map(|e| {
                    if let StackElement::Word(str) = e {
                        Ok(str.as_str())
                    } else {
                        Err(Self::error("stack may only contain words"))
                    }
                })
                .rev()
                .collect::<Result<String, Error>>()
            {
                self.datastack.push(StackElement::Word(s));
                return Ok(self);
            }
        }

        Err(Self::error("need stack to use 'word'"))
    }

    pub fn unword(mut self) -> Result<Self, Error> {
        if let StackElement::Word(str) =
            Self::pop_or_err(&mut self.datastack, "not enough operands")?
        {
            self.datastack.push(StackElement::SubStack(
                str.chars()
                    .map(|c| StackElement::Word(c.to_string()))
                    .rev()
                    .collect(),
            ));

            return Ok(self);
        }

        Err(Self::error("need word to use 'unword'"))
    }

    pub fn char(mut self) -> Result<Self, Error> {
        if let StackElement::Word(w) = Self::pop_or_err(&mut self.datastack, "not enough operands")?
        {
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
                        char::from_u32(
                            u32::from_str_radix(&ss, 16)
                                .map_err(|_| Self::error("invalid utf string"))?,
                        )
                        .ok_or(Self::error("invalid utf string"))?
                        .to_string()
                    } else {
                        return Err(Self::error("invalid utf string"));
                    }
                }
            }));

            return Ok(self);
        }
        Err(Self::error("need word to parse to utf"))
    }

    pub fn print(mut self) -> Result<Self, Error> {
        print!(
            "{}",
            Self::pop_or_err(&mut self.datastack, "not enough operands")?
        );

        Ok(self)
    }

    pub fn flush(self) -> Result<Self, Error> {
        stdout().flush().unwrap();

        Ok(self)
    }

    pub fn read_line(mut self) -> Result<Self, Error> {
        let mut inp = "".to_string();
        stdin().read_line(&mut inp).unwrap();

        self.datastack.push(StackElement::Word(inp));
        Ok(self)
    }

    pub fn slurp(mut self) -> Result<Self, Error> {
        if let StackElement::Word(src) =
            Self::pop_or_err(&mut self.datastack, "not enough operands")?
        {
            self.datastack
                .push(StackElement::Word(fs::read_to_string(src)?));
            return Ok(self);
        }

        Err(Self::error("need word to read from file"))
    }

    pub fn spit(mut self) -> Result<Self, Error> {
        if let StackElement::Word(data) =
            Self::pop_or_err(&mut self.datastack, "not enough operands")?
        {
            if let StackElement::Word(file) =
                Self::pop_or_err(&mut self.datastack, "not enough operands")?
            {
                fs::write(file, data)?;
                return Ok(self);
            }
        }

        Err(Self::error("could not write file"))
    }

    pub fn spit_on(mut self) -> Result<Self, Error> {
        if let StackElement::Word(data) =
            Self::pop_or_err(&mut self.datastack, "not enough operands")?
        {
            if let StackElement::Word(path) =
                Self::pop_or_err(&mut self.datastack, "not enough operands")?
            {
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(path)
                    .unwrap();

                file.write_all(data.as_bytes()).unwrap();
                return Ok(self);
            }
        }

        Err(Self::error("could not write file"))
    }

    pub fn uncomment(mut self) -> Result<Self, Error> {
        if let StackElement::Word(wrd) =
            Self::pop_or_err(&mut self.datastack, "not enough operands")?
        {
            self.datastack.push(StackElement::Word(
                wrd.lines()
                    .map(|l| l.split('%').next().unwrap())
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join(" "),
            ));

            return Ok(self);
        }

        Err(Self::error("I can only uncomment words"))
    }

    pub fn tokenize(mut self) -> Result<Self, Error> {
        if let StackElement::Word(w) = Self::pop_or_err(&mut self.datastack, "not enough operands")?
        {
            self.datastack.push(StackElement::SubStack(
                w.split_whitespace()
                    .map(|s| StackElement::Word(s.to_string()))
                    .rev()
                    .collect(),
            ));

            return Ok(self);
        }

        Err(Self::error("need word to tokenize"))
    }

    pub fn undocument(self) -> Result<Self, Error> {
        unimplemented!()
    }

    pub fn ctm(mut self) -> Result<Self, Error> {
        self.datastack.push(StackElement::Word(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis()
                .to_string(),
        ));

        Ok(self)
    }

    pub fn os(mut self) -> Result<Self, Error> {
        self.datastack
            .push(StackElement::Word(env::consts::OS.to_string()));

        Ok(self)
    }

    pub fn call(mut self) -> Result<Self, Error> {
        match Self::pop_or_err(&mut self.datastack, "not enough operands")? {
            StackElement::SubStack(mut st) => self.callstack.append(&mut st),
            StackElement::Word(w) => self.callstack.push(StackElement::Word(w)),
            _ => unimplemented!(),
        }

        Ok(self)
    }

    pub fn call_cc(mut self) -> Result<Self, Error> {
        let top = Self::pop_or_err(&mut self.datastack, "not enough operands")?;
        let mut new_callstack = Vec::new();
        match top {
            StackElement::SubStack(mut ss) => new_callstack.append(&mut ss),
            el => new_callstack.push(el),
        }

        self.datastack = vec![
            StackElement::SubStack(self.datastack),
            StackElement::SubStack(self.callstack),
        ];

        Ok(Self::new(self.datastack, new_callstack, self.dictionary))
    }

    pub fn r#continue(mut self) -> Result<Self, Error> {
        if let StackElement::SubStack(new_callstack) =
            Self::pop_or_err(&mut self.datastack, "not enough operands")?
        {
            if let StackElement::SubStack(new_datastack) =
                Self::pop_or_err(&mut self.datastack, "not enough operands")?
            {
                return Ok(Self::new(new_datastack, new_callstack, self.dictionary));
            }
        }

        Err(Self::error("need quotation for continue"))
    }

    pub fn get_dict(mut self) -> Result<Self, Error> {
        let mut map = Vec::new();
        self.dictionary.clone().into_iter().for_each(|(k, v)| {
            let key = StackElement::Word(k);
            let value = StackElement::Fun(v);
            if !map.iter().map(|(i, _)| i).any(|se| key == *se) {
                map.push((key, value));
            }
        });

        self.datastack.push(StackElement::Map(map));

        Ok(self)
    }

    pub fn set_dict(mut self) -> Result<Self, Error> {
        if let StackElement::Map(dict) =
            Self::pop_or_err(&mut self.datastack, "not enough operands")?
        {
            let dict = map_to_dict(&dict)?;
            return Ok(Self::new(self.datastack, self.callstack, dict));
        }

        Err(Self::error("need map for set-dict"))
    }

    pub fn stepcc(mut self) -> Result<Self, Error> {
        let e = Self::pop_or_err(&mut self.callstack, "not enough operands")?;

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

        Ok(self)
    }

    pub fn apply(mut self) -> Result<Self, Error> {
        if let StackElement::Fun(fun) =
            Self::pop_or_err(&mut self.datastack, "not enough operands")?
        {
            if let StackElement::SubStack(stack) =
                Self::pop_or_err(&mut self.datastack, "not enough operands")?
            {
                self.datastack
                    .push(StackElement::SubStack(match fun.deref() {
                        Funct::BuiltIn(bi) => {
                            bi(Self::new(
                                stack,
                                self.callstack.clone(),
                                self.dictionary.clone(),
                            ))?
                            .datastack
                        }
                        Funct::SelfDefined(_) => unimplemented!(),
                    }))
            }
        }

        Ok(self)
    }

    pub fn compose(self) -> Result<Self, Error> {
        unimplemented!()
    }

    pub fn func(mut self) -> Result<Self, Error> {
        if let StackElement::Map(dict) =
            Self::pop_or_err(&mut self.datastack, "not enough operands")?
        {
            let dict = map_to_dict(&dict)?;
            if let StackElement::SubStack(qt) = self.datastack.pop().unwrap() {
                fn runcc(inner: Interpreter) -> Result<Vec<StackElement>, Error> {
                    let mut int = inner;
                    while !int.callstack.is_empty() {
                        int = int.stepcc()?;
                    }

                    Ok(int.datastack)
                }

                let f = move |interpreter: Interpreter| {
                    let qt = qt.to_owned();
                    Ok(Self::new(
                        runcc(Self::new(interpreter.datastack, qt, dict.to_owned()))?,
                        Vec::new(),
                        BTreeMap::new(),
                    ))
                };

                self.datastack
                    .push(StackElement::Fun(Rc::new(Funct::BuiltIn(Rc::new(f)))));
            }
        }

        Ok(self)
    }

    pub fn integer(mut self) -> Result<Self, Error> {
        let e = Self::pop_or_err(&mut self.datastack, "not enough operands")?;
        self.datastack.push(match e {
            StackElement::Word(w) => match w.parse::<usize>() {
                Ok(_) => StackElement::Word("t".to_string()),
                Err(_) => StackElement::Word("f".to_string()),
            },
            _ => StackElement::Word("f".to_string()),
        });

        Ok(self)
    }

    pub fn addition(self) -> Result<Self, Error> {
        self.binary("+")
    }

    pub fn subtraction(self) -> Result<Self, Error> {
        self.binary("-")
    }

    pub fn multiplication(self) -> Result<Self, Error> {
        self.binary("*")
    }

    pub fn division(self) -> Result<Self, Error> {
        self.binary("div")
    }

    pub fn modulo(self) -> Result<Self, Error> {
        self.binary("mod")
    }

    pub fn greater_than(self) -> Result<Self, Error> {
        self.binary(">")
    }

    pub fn less_than(self) -> Result<Self, Error> {
        self.binary("<")
    }

    pub fn equals(self) -> Result<Self, Error> {
        self.binary("==")
    }

    pub fn less_equals(self) -> Result<Self, Error> {
        self.binary("<=")
    }

    pub fn greater_equals(self) -> Result<Self, Error> {
        self.binary(">=")
    }

    fn binary(mut self, op: &str) -> Result<Self, Error> {
        if let StackElement::Word(w1) =
            Self::pop_or_err(&mut self.datastack, "not enough operands")?
        {
            if let StackElement::Word(w2) =
                Self::pop_or_err(&mut self.datastack, "not enough operands")?
            {
                let x: isize = w1
                    .parse()
                    .map_err(|_| Self::error(format!("{} is not an integer", w1,).as_str()))?;
                let y: isize = w2
                    .parse()
                    .map_err(|_| Self::error(format!("{} is not an integer", w1,).as_str()))?;
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
                return Ok(self);
            }
        }
        Err(Self::error("need integers for binary operations"))
    }

    pub fn comment(mut self) -> Result<Self, Error> {
        self.datastack
            .push(StackElement::Fun(Rc::new(Funct::SelfDefined(
                StackElement::SubStack(vec![
                    StackElement::Word("continue".to_string()),
                    StackElement::Word("pop".to_string()),
                    StackElement::Word("swap".to_string()),
                    StackElement::Word("push".to_string()),
                    StackElement::Word("swap".to_string()),
                    StackElement::Word("rot".to_string()),
                    StackElement::Word("top".to_string()),
                    StackElement::Word("dup".to_string()),
                ]),
            ))));
        self.callstack
            .push(StackElement::Word("call/cc".to_string()));

        Ok(self)
    }

    pub fn load(mut self) -> Result<Self, Error> {
        self.callstack.append(&mut vec![
            StackElement::Word("tokenize".to_string()),
            StackElement::Word("uncomment".to_string()),
            StackElement::Word("slurp".to_string()),
        ]);

        Ok(self)
    }

    pub fn run(mut self) -> Result<Self, Error> {
        self.callstack.append(&mut vec![
            StackElement::Word("call".to_string()),
            StackElement::Word("load".to_string()),
        ]);

        Ok(self)
    }

    pub fn start(mut self) -> Result<Self, Error> {
        self.callstack.append(&mut vec![
            StackElement::Word("slurp".to_string()),
            StackElement::Word("uncomment".to_string()),
            StackElement::Word("tokenize".to_string()),
            StackElement::Word("get-dict".to_string()),
            StackElement::Word("func".to_string()),
            StackElement::Word("emptystack".to_string()),
            StackElement::Word("swap".to_string()),
            StackElement::Word("apply".to_string()),
        ]);

        Ok(self)
    }
}
