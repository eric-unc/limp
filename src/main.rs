extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::env;
use std::io::{self, Write};
use std::process::exit;
use crate::evaluator::{Environment, eval_with_env};

#[derive(Parser)]
#[grammar = "limp.pest"]
pub struct LimpParser;

#[cfg(test)]
mod tests;

mod evaluator;

fn main() {
	//let p = LimpParser::parse(Rule::program, "(print (+ 1 2) (- 1 2) (* 3 4) (/ 8 2))");
	//println!("{:?}", p);
	//evaluator::evaluate(p.unwrap());
	//exit(0); // temp

	let args: Vec<String> = env::args().collect();

	if args.len() >= 2 {
		load_and_interpret(&args[1]);
	} else {
		repl();
	}
}

fn load_and_interpret(file_name: &String) {
	// TODO
}

fn repl() {
	let env = Environment::new();

	loop {
		print!(">>> ");
		io::stdout().flush().unwrap();

		let mut line = String::new();
		io::stdin().read_line(&mut line).unwrap();
		line = line.trim().to_string();

		if line.is_empty() {
			return;
		}

		let parse_tree = LimpParser::parse(Rule::program, &line);

		match parse_tree {
			Ok(tree) => {
				eval_with_env(tree, &env);
			},
			Err(e) => {
				println!("{}", e)
			}
		}

		io::stdout().flush().unwrap();
	}
}
