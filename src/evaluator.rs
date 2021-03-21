
use pest::iterators::Pairs;
use std::collections::HashMap;
use crate::Rule;
use std::ptr::null;

pub enum LimpValue {
	Integer(u64),
	Float(f64),
	Error
}

type Scope = HashMap<String, LimpValue>;
type Bindings = Vec<Scope>;

pub struct Environment {
	bindings: Bindings
}

impl Environment {
	pub fn new() -> Self {
		Self {
			bindings: vec![Scope::new()]
		}
	}

	pub fn add_scope(&mut self){
		self.bindings.push(Scope::new());
	}

	pub fn close_scope(&mut self){
		self.bindings.pop();
	}

	pub fn add_binding(&mut self, name: String, val: LimpValue){
		// TODO: lol this doesn't work
		//self.bindings.last().unwrap().insert(name, val);
	}
}

// program ::= expr_list
pub fn evaluate(tree: Pairs<Rule>){
	//println!("{:?}", tree);
	//eval_expr_list(tree);

	// TODO: I don't know how to go into this without a for loop
	for pair in tree {
		eval_expr_list(pair.into_inner());
	}
}

// expr_list ::= expr+
fn eval_expr_list(exprs: Pairs<Rule>) -> Vec<LimpValue> {
	//println!("{:?}", tree);
	let mut ret = Vec::new();

	for expr in exprs {
		ret.push(eval_expr(expr.into_inner()));
	}

	ret
}

// expr :: atom | invocation
fn eval_expr(expr: Pairs<Rule>) -> LimpValue {
	let mut ret = LimpValue::Error; // TODO: clean this up

	for pair in expr {
		for inner_pair in pair.into_inner() {
			match inner_pair.as_rule() {
				Rule::atom => ret = eval_atom(inner_pair.into_inner()),
				Rule::invocation => ret = eval_invocation(inner_pair.into_inner()),
				_ => unreachable!()
			}
		}
	}

	ret
}

fn eval_atom(atom: Pairs<Rule>) -> LimpValue {
	println!("{:?}", atom);
	LimpValue::Float(5.5)
}

fn eval_invocation(invocation: Pairs<Rule>) -> LimpValue {
	LimpValue::Float(5.5) // todo
}
