use crate::LimpParser;
use crate::Rule;
use crate::evaluator::eval;

use crate::pest::Parser;

// TODO: expand

#[test]
fn eval_add(){
	let tree = LimpParser::parse(Rule::program, "(+ 5 9)").unwrap();
	eval(tree);
}
