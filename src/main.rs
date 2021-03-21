extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::{env, fs};
use std::io::{self, Write};
use crate::evaluator::{Environment, eval_with_env, eval};

#[derive(Parser)]
#[grammar = "limp.pest"]
pub struct LimpParser;

#[cfg(test)]
mod tests;

mod evaluator;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() >= 2 {
		load_and_interpret(&args[1]);
	} else {
		repl();
	}
}

fn load_and_interpret(file_name: &String) {
	let script = fs::read_to_string(file_name);

	match script {
		Ok(s) => { eval(LimpParser::parse(Rule::program, &s).unwrap()); }
		Err(e) => { panic!("{:?}", e)}
	}
}

fn repl() {
	let mut env = Environment::new();

	loop {
		print!(">>> ");
		io::stdout().flush().unwrap();

		let mut line = String::new();
		io::stdin().read_line(&mut line).unwrap();
		line = line.trim().to_string();

		if line.is_empty() {
			continue;
		}

		let parse_tree = LimpParser::parse(Rule::program, &line);

		match parse_tree {
			Ok(tree) => {
				eval_with_env(tree, &mut env);
			},
			Err(e) => {
				println!("{}", e)
			}
		}

		io::stdout().flush().unwrap();
	}
}
