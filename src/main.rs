extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "limp.pest"]
pub struct LimpParser;

fn main(){
	let p = LimpParser::parse(Rule::number, "5");
	println!("{:?}", p);
}
