use colored::Colorize;
use cpu_time::ProcessTime;
use interpreter::Interpreter;
use stack_element::{print_stack, Funct, StackElement};
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

    let mut int3 = call(int);

    //let mut int3 = optimise_dict(int2);

    let start = ProcessTime::now();

    int3.datastack = vec![StackElement::Word(
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
        //println!("{}", n);
        new_dict.insert(
            n.clone(),
            match l.deref() {
                Funct::BuiltIn(bi) => Rc::new(Funct::BuiltIn(bi.clone())),
                Funct::SelfDefined(sd) => match sd {
                    StackElement::SubStack(ss) => {
                        let step_1 = preprocess(n.clone(), ss, &dictionary);
                        let step_2 = replace_with_fun(&step_1, &dictionary);
                        Rc::new(Funct::SelfDefined(StackElement::SubStack(step_2)))
                    }
                    StackElement::Word(w) => Rc::new(Funct::SelfDefined(StackElement::SubStack(
                        preprocess(n.clone(), &vec![StackElement::Word(w.clone())], &dictionary),
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
    words: &Vec<StackElement>,
    dictionary: &Rc<BTreeMap<String, Rc<Funct>>>,
) -> Vec<StackElement> {
    let new_ds: Vec<StackElement> = words
        .iter()
        .enumerate()
        .flat_map(|(i, se)| match se.clone() {
            StackElement::Word(w) => {
                let ret = if w != word
                    && (i == words.len() - 1
                        || words[i + 1] != StackElement::Word("\\".to_string()))
                {
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
                };
                ret
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
                                        &vec![StackElement::Word(w)],
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

    new_ds
}

fn replace_with_fun(
    words: &[StackElement],
    dictionary: &Rc<BTreeMap<String, Rc<Funct>>>,
) -> Vec<StackElement> {
    words
        .iter()
        .enumerate()
        .map(|(i, se)| match se {
            StackElement::Word(ref w) => {
                if i == words.len() - 1 || words[i + 1] != StackElement::Word("\\".to_string()) {
                    match dictionary.get(w) {
                        Some(f) => match f.deref() {
                            Funct::BuiltIn(bi) => {
                                StackElement::Fun(Rc::new(Funct::BuiltIn(bi.clone())))
                            }
                            Funct::SelfDefined(_) => se.to_owned(),
                        },
                        None => se.to_owned(),
                    }
                } else {
                    se.to_owned()
                }
            }
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
