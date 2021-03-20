extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::env;

#[derive(Parser)]
#[grammar = "limp.pest"]
pub struct LimpParser;

fn main(){
	//let p = LimpParser::parse(Rule::program, "5");
	//println!("{:?}", p);

	let args: Vec<String> = env::args().collect();

	if args.len() >= 2 {
		load_and_interpret(&args[1]);
	} else {
		repl();
	}
}

fn load_and_interpret(file_name: &String){
	// TODO
}

fn repl(){
	// TODO
}

