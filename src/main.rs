extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::env;
use std::io::{self, Write};

#[derive(Parser)]
#[grammar = "limp.pest"]
pub struct LimpParser;

#[cfg(test)]
mod tests;

fn main() {
	//let p = LimpParser::parse(Rule::program, "5");
	//println!("{:?}", p);

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
	loop {
		print!("> ");
		io::stdout().flush().unwrap();

		let mut line = String::new();
		io::stdin().read_line(&mut line).unwrap();
		line = line.strip_suffix("\n").unwrap().to_string();
		if line.is_empty() {
			return;
		}

		eval_line(&line);
		io::stdout().flush().unwrap();
	}
}

fn eval_line(line: &String) {
	let pairs = LimpParser::parse(Rule::expr, line).unwrap_or_else(|e| panic!("{}", e));

	for pair in pairs {
		for inner_pair in pair.into_inner() {
			match inner_pair.as_rule() {
				Rule::atom => eval_atom(inner_pair.as_str()),
        Rule::invocation => eval_invocation(inner_pair.as_str()),
        _ => unreachable!()
			};
		}
	}
}

fn eval_atom(atom: &str) {
	println!("{}", atom);
}

fn eval_invocation(invocation: &str) {
	// TODO
	println!("Invocation: {}", invocation)
}
