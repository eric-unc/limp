extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::env;
use std::io::{self, BufRead, Write};

#[derive(Parser)]
#[grammar = "limp.pest"]
pub struct LimpParser;

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
	while true {
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
	// TODO
	println!("{}", line);
}
