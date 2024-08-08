use clap::{arg, Command};
use colored::Colorize;
use core::panic;
use cpu_time::ProcessTime;
use interpreter::Interpreter;
use preprocessor::{call_fn_step_1, call_fn_step_2, call_fn_step_3, call_fn_step_4, optimise_dict};
use stack_element::{print_stack, Funct, StackElement};
use std::{ops::Deref, rc::Rc};

pub mod interpreter;
pub mod preprocessor;
pub mod stack_element;

fn main() {
    let cli = load_program_data().get_matches();
    let end;

    let int = Interpreter {
        datastack: vec![StackElement::Word("\\ prelude-plain.txt run".to_string())],
        callstack: Vec::new(),
        dictionary: Rc::new(Interpreter::init_dictionary()),
    };

    let code: String = cli
        .get_one::<String>("code")
        .expect("Code has to be provided")
        .clone();
    let mut int2 = call(int, 0);

    let int3 = match cli
        .get_one::<String>("level")
        .unwrap_or(&"0".to_string())
        .parse::<u8>()
        .expect(format!("{} level needs to be numeric", "Error:".bold().red()).as_str())
    {
        0 => {
            int2.datastack = vec![StackElement::Word(code)];
            let start = ProcessTime::now();
            let ret = call(int2, 0);
            end = Some(start.elapsed());

            ret
        }
        i if i > 0_u8 && i < 4_u8 => {
            let mut int = optimise_dict(int2, i);
            int.datastack = vec![StackElement::Word(code)];

            let start = ProcessTime::now();

            let ret = call(int, i);
            end = Some(start.elapsed());

            ret
        }
        4 => {
            let mut new_dict = int2.dictionary.deref().clone();
            new_dict.insert(
                "call".to_string(),
                Rc::new(Funct::BuiltIn(Rc::new(
                    Interpreter::call_after_preproccess_step_4,
                ))),
            );
            new_dict.insert(
                "\\".to_string(),
                Rc::new(Funct::BuiltIn(Rc::new(
                    Interpreter::comment_after_preprocess,
                ))),
            );
            new_dict.insert(
                "run".to_string(),
                Rc::new(Funct::BuiltIn(Rc::new(
                    Interpreter::run_after_preprocess_step_4,
                ))),
            );
            new_dict.insert(
                "call/cc".to_string(),
                Rc::new(Funct::BuiltIn(Rc::new(
                    Interpreter::call_cc_after_preprocess_step_4,
                ))),
            );
            new_dict.insert(
                "func".to_string(),
                Rc::new(Funct::BuiltIn(Rc::new(
                    Interpreter::func_after_preprocess_step_4,
                ))),
            );
            int2.dictionary = Rc::new(new_dict);

            let mut int = optimise_dict(int2, 4);
            int.datastack = vec![StackElement::Word(code)];
            let start = ProcessTime::now();
            let ret = call(int, 4);
            end = Some(start.elapsed());

            ret
        }
        _ => panic!("level has to be between 0 and 4"),
    };

    println!(
        "{} {} Took {:?}",
        "Consize returns:".yellow().bold(),
        print_stack(&int3.datastack, false, false),
        end.unwrap()
    )
}

fn call(int: Interpreter, level: u8) -> Interpreter {
    let mut int1 = int.uncomment().tokenize();
    let new_datastack = match level {
        0 => int1.datastack,
        1 => call_fn_step_1("asdfghj".to_string(), &int1.datastack, &int1.dictionary),
        2 => call_fn_step_2("asdfghj".to_string(), &int1.datastack, &int1.dictionary),
        3 => call_fn_step_3("asdfghj".to_string(), &int1.datastack, &int1.dictionary),
        4 => match int1.datastack.pop().unwrap() {
            StackElement::SubStack(ss) => vec![StackElement::Fun(Rc::new(Funct::BuiltIn(
                call_fn_step_4("asdfghj".to_string(), &ss, &int1.dictionary),
            )))],
            _ => panic!("passiert nicht"),
        },

        _ => panic!("invalid level"),
    };

    int1.datastack = new_datastack;
    if level < 4 {
        let mut int2 = int1.get_dict().func();
        int2.datastack.push(StackElement::SubStack(Vec::new()));
        int2.swap().apply()
    } else {
        int1.datastack.push(StackElement::SubStack(Vec::new()));
        int1.swap().apply()
    }
}

fn load_program_data() -> Command {
    Command::new("Consize Rust")
        .version("0.1.0")
        .about("This is a Rust implementation of the consize programming language, incorporating a few performance enhancements. Some work better, some worse.")
        .args([arg!(code: <code> "Consize code to execute, has to be in double quotes. The prelude has been preloaded"), 
               arg!(level: -l --level <lvl> "Optimization level. \n\t0: Default. Without any optimizations. Just vanilla consize. \n\t1: All prelude functions have been expanded to only contain primitives. \n\t2: All primitive functions are replaced by rust functions. \n\t3: All remaining words and quotations are replaced by functions. \n\t4: Consize is now executed as one big function composition. \nLevel 3 and 4 are not working correctly, refer to the documentation for more details."),
            ])
}
