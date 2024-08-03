use colored::Colorize;
use core::panic;
use cpu_time::ProcessTime;
use interpreter::Interpreter;
use stack_element::{print_stack, BuiltIn, Funct, StackElement};
use std::{collections::BTreeMap, env, ops::Deref, rc::Rc};

pub mod interpreter;
pub mod stack_element;

fn main() {
    let args = env::args();
    let int = Interpreter {
        datastack: vec![StackElement::Word(
            "\\ prelude-plain.txt run".to_string(), //args.skip(1).collect::<Vec<String>>().join(" "),
        )],
        callstack: Vec::new(),
        dictionary: Rc::new(Interpreter::init_dictionary()),
    };

    let mut int2 = call(int);
    let mut new_dict = int2.dictionary.deref().clone();
    new_dict.insert(
        "stepcc".to_string(),
        Rc::new(Funct::BuiltIn(Rc::new(Interpreter::new_stepcc))),
    );
    new_dict.insert(
        "call".to_string(),
        Rc::new(Funct::BuiltIn(Rc::new(Interpreter::call_after_preproccess))),
    );
    int2.dictionary = Rc::new(new_dict);

    //println!("asdfghjklöä");

    //println!("{:?}", int2.dictionary.get("["));

    let mut int3 = optimise_dict(int2);

    //println!("{:?}", int3.dictionary.get("["));

    let start = ProcessTime::now();

    int3.datastack = vec![StackElement::Word(
        //"say-hi".to_string(),
        args.skip(1).collect::<Vec<String>>().join(" "),
    )];

    let int4 = call(int3);

    let end = start.elapsed();

    println!(
        "{} {} Took {:?}",
        "Consize returns:".yellow().bold(),
        print_stack(&int4.datastack, false, false),
        end
    )
}

fn call(int: Interpreter) -> Interpreter {
    let mut int1 = int.uncomment().tokenize().get_dict().func();

    int1.datastack.push(StackElement::SubStack(Vec::new()));
    int1.swap().apply()
}

pub fn optimise_dict(mut int: Interpreter) -> Interpreter {
    let dictionary = int.dictionary.to_owned();
    let mut new_dict = BTreeMap::new();

    dictionary.iter().for_each(|(n, l)| {
        new_dict.insert(
            n.clone(),
            match l.deref() {
                Funct::BuiltIn(bi) => Rc::new(Funct::BuiltIn(bi.clone())),
                Funct::SelfDefined(sd) => match sd {
                    StackElement::SubStack(ss) => {
                        let step_1 = preprocess(n.clone(), ss, &dictionary);
                        let step_2 = replace_with_fun(&step_1, &dictionary);
                        let step_3 = map_to_functions(&step_2);
                        //let step_4 = compose_functions(&step_3);
                        //Rc::new(Funct::BuiltIn(step_4))
                        Rc::new(Funct::SelfDefined(StackElement::SubStack(step_3)))
                    }
                    StackElement::Word(w) => Rc::new(Funct::SelfDefined(StackElement::SubStack(
                        preprocess(n.clone(), &[StackElement::Word(w.clone())], &dictionary),
                    ))),
                    _ => unimplemented!(),
                },
            },
        );
    });

    int.dictionary = Rc::new(new_dict);

    int
}

pub fn preprocess(
    word: String,
    words: &[StackElement],
    dictionary: &Rc<BTreeMap<String, Rc<Funct>>>,
) -> Vec<StackElement> {
    let new_words: Vec<StackElement> = words
        .iter()
        .enumerate()
        .flat_map(|(i, se)| match se.clone() {
            StackElement::Word(w) => {
                if w != word
                    || (i + 2 < words.len()
                        && words[i + 1] == StackElement::Word("\\".to_string())
                        && words[i + 2] != StackElement::Word("\\".to_string()))
                    || (i + 2 == words.len()
                        && words[i + 1] == StackElement::Word("\\".to_string()))
                {
                    if (i + 2 < words.len()
                        && words[i + 1] == StackElement::Word("\\".to_string())
                        && words[i + 2] != StackElement::Word("\\".to_string()))
                        || (i + 2 == words.len()
                            && words[i + 1] == StackElement::Word("\\".to_string()))
                    {
                        //println!("word: {}, words: {:?}", word, words);
                        return vec![StackElement::Fun(Rc::new(Funct::BuiltIn(pull_to_ds(
                            se.clone(),
                        ))))];
                    }
                    match dictionary.get(&w) {
                        Some(fun) => match fun.deref() {
                            Funct::BuiltIn(_) => vec![se.to_owned()],
                            Funct::SelfDefined(sd) => match sd {
                                StackElement::SubStack(sd) => preprocess(w.clone(), sd, dictionary),
                                _ => unimplemented!(),
                            },
                        },
                        None => vec![se.to_owned()],
                    }
                } else {
                    vec![se.to_owned()]
                }
            }
            StackElement::SubStack(ss) => vec![StackElement::SubStack(preprocess(
                word.to_owned(),
                &ss,
                dictionary,
            ))],
            StackElement::Map(m) => {
                vec![StackElement::Map(
                    m.into_iter()
                        .map(|(k, v)| {
                            (
                                k,
                                match v {
                                    StackElement::SubStack(ss) => StackElement::SubStack(
                                        preprocess(word.clone(), &ss, dictionary),
                                    ),
                                    StackElement::Word(w) => StackElement::SubStack(preprocess(
                                        word.clone(),
                                        &[StackElement::Word(w)],
                                        dictionary,
                                    )),
                                    _ => todo!(),
                                },
                            )
                        })
                        .collect(),
                )]
            }
            _ => vec![se.to_owned()],
        })
        .collect();
    new_words
        .into_iter()
        .filter(|i| i != &StackElement::Word("\\".to_string()))
        .collect()
}

fn replace_with_fun(
    words: &[StackElement],
    dictionary: &Rc<BTreeMap<String, Rc<Funct>>>,
) -> Vec<StackElement> {
    words
        .iter()
        .enumerate()
        .map(|(_, se)| match se {
            StackElement::Word(ref w) => match dictionary.get(w) {
                Some(f) => match f.deref() {
                    Funct::BuiltIn(bi) => StackElement::Fun(Rc::new(Funct::BuiltIn(bi.clone()))),
                    Funct::SelfDefined(_) => se.to_owned(),
                },
                None => se.to_owned(),
            },
            StackElement::SubStack(ss) => StackElement::SubStack(replace_with_fun(ss, dictionary)),
            StackElement::Map(m) => StackElement::Map(
                m.iter()
                    .map(|(k, v)| {
                        (
                            k.clone(),
                            match v {
                                StackElement::SubStack(ss) => {
                                    StackElement::SubStack(replace_with_fun(ss, dictionary))
                                }
                                _ => unimplemented!(),
                            },
                        )
                    })
                    .collect(),
            ),
            _ => se.to_owned(),
        })
        .collect()
}

fn map_to_functions(words: &[StackElement]) -> Vec<StackElement> {
    words
        .iter()
        .map(|se| match se {
            StackElement::Word(w) => {
                StackElement::Fun(Rc::new(Funct::BuiltIn(wrap_word(w.clone()))))
            }
            StackElement::Fun(f) => match f.deref() {
                Funct::BuiltIn(_) => se.to_owned(),
                Funct::SelfDefined(_) => todo!(),
            },
            StackElement::SubStack(ss) => StackElement::Fun(Rc::new(Funct::BuiltIn(pull_to_ds(
                StackElement::SubStack(map_to_functions(ss.as_slice())),
            )))),
            StackElement::Map(m) => {
                StackElement::Fun(Rc::new(Funct::BuiltIn(pull_to_ds(StackElement::Map(
                    m.iter()
                        .map(|(k, v)| {
                            (
                                k.clone(),
                                match v {
                                    StackElement::SubStack(ss) => {
                                        StackElement::SubStack(map_to_functions(ss.as_slice()))
                                    }
                                    _ => unimplemented!(),
                                },
                            )
                        })
                        .collect(),
                )))))
            }
            StackElement::Nil => {
                StackElement::Fun(Rc::new(Funct::BuiltIn(pull_to_ds(StackElement::Nil))))
            }
        })
        .collect()
}

fn wrap_word(word: String) -> BuiltIn {
    Rc::new(
        move |mut int: Interpreter| match int.dictionary.clone().get(&word) {
            Some(fun) => match fun.deref() {
                Funct::BuiltIn(fct) => fct(int),
                Funct::SelfDefined(stack) => {
                    if let StackElement::SubStack(ss) = stack.to_owned() {
                        int.callstack.append(&mut map_to_functions(&ss));
                        int
                    } else {
                        unimplemented!()
                    }
                }
            },
            None => {
                int.datastack.push(StackElement::Word(word.clone()));
                int.callstack
                    .push(StackElement::Fun(Rc::new(Funct::BuiltIn(wrap_word(
                        "read-word".to_string(),
                    )))));
                int
            }
        },
    )
}

fn pull_to_ds(se: StackElement) -> BuiltIn {
    Rc::new(move |mut int: Interpreter| {
        //println!("{}", se);
        int.datastack.push(se.clone());
        int
    })
}

fn compose_functions(words: &[StackElement]) -> BuiltIn {
    if words.is_empty() {
        return Rc::new(move |int: Interpreter| int);
    }
    words
        .iter()
        .map(|se| match se {
            StackElement::Fun(f) => match f.deref() {
                Funct::BuiltIn(bi) => bi.clone(),
                Funct::SelfDefined(_) => panic!("auch SelfDefined sollte es hier nicht mehr geben"),
            },
            _ => panic!("gibts hier nicht"),
        })
        .reduce(|a, b| compose_two(a, b))
        .unwrap()
}

fn compose_two(a: BuiltIn, b: BuiltIn) -> BuiltIn {
    Rc::new(move |i| a(b(i)))
}
pub fn call_fn(
    words: &[StackElement],
    dictionary: &Rc<BTreeMap<String, Rc<Funct>>>,
) -> Vec<StackElement> {
    map_to_functions(
        replace_with_fun(
            preprocess("asdfghjkl".to_string(), words, dictionary).as_slice(),
            dictionary,
        )
        .as_slice(),
    )
}
