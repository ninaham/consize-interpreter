use clap::{arg, Command};
use colored::Colorize;
use cpu_time::ProcessTime;
use interpreter::Interpreter;
use preprocessor::optimise_dict;
use stack_element::{print_stack, Funct, StackElement};
use std::{ops::Deref, rc::Rc};

pub mod interpreter;
pub mod preprocessor;
pub mod stack_element;

fn main() {
    let cli = load_program_data().get_matches();

    let int = Interpreter {
        datastack: vec![StackElement::Word(
            "\\ prelude-plain.txt run".to_string(),
        )],
        callstack: Vec::new(),
        dictionary: Rc::new(Interpreter::init_dictionary()),
    };

    let code: String = cli
        .get_one::<String>("code")
        .expect("Code has to be provided")
        .clone();
    let mut int2 = call(int);

    let mut int3 = match cli.get_one::<String>("level").unwrap_or(&"0".to_string()).parse::<u8>().expect(format!("{}", "level needs to be numeric".red()).as_str()) {
        0 => int2,
        i if i > 0_u8 && i < 4_u8 => optimise_dict(int2, i),
        4 => {
            let mut new_dict = int2.dictionary.deref().clone();
            new_dict.insert(
                "call".to_string(),
                Rc::new(Funct::BuiltIn(Rc::new(
                    Interpreter::call_after_preproccess_step_4,
                ))),
            );
            int2.dictionary = Rc::new(new_dict);
            optimise_dict(int2, 4)
        }
        _ => panic!("level has to be between 0 and 4"),
    };

    let start = ProcessTime::now();

    int3.datastack = vec![StackElement::Word(code)];

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

fn load_program_data() -> Command {
    Command::new("Consize Rust")
        .version("0.1.0")
        .about("This is a Rust implementation of the consize programming language, incorporating a few performance enhancements. Some work better, some worse.")
        .args([arg!(code: <code> "Consize code to execute, has to be in double quotes. The prelude has been preloaded"), 
               arg!(level: -l --level <lvl> "Optimization level. \t\n0: Without any optimizations. Just vanilla consize. \t\n1: All prelude functions have been expanded to only contain primitives. \t\n2: All primitive functions are replaced by rust functions. \t\n3: All remaining words and quotations are replaced by functions. \t\n4: Consize is now executed as one big function composition. \nLevel 3 and 4 are not working correctly, refer to the documentation for more details."),
            ])
}
