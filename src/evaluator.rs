
use pest::iterators::{Pairs, Pair};
use std::collections::HashMap;
use crate::Rule;
use std::ptr::null;
use std::fmt;

#[derive(Debug)]
pub enum LimpValue {
	Integer(u64),
	Float(f64),
	Name(String),
	ErrorValue
}

/*impl fmt::Debug for LimpValue {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("LimpValue").field(
		match self {
			LimpValue::Integer(_) => "Integer",
			LimpValue::Float(_) => "Float",
			LimpValue::Name(_) => "Name",
			Error => {}
		}, "5").finish()
		//f.debug_struct("LimpValue").finish()
	}
}*/

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
	println!("{:?}", tree);

	for pair in tree {
		//println!("{:?}", pair);

		for inner_pair in pair.into_inner() {
			//println!("{:?}", inner_pair);

			match inner_pair.as_rule() {
				Rule::expr_list => {
					eval_expr_list(inner_pair);
				},
				Rule::EOI => {},
				_ => {
					println!("{:?}", inner_pair.as_rule());
					unreachable!();
				}
			}
		}
	}
}

// expr_list ::= expr+
fn eval_expr_list(exprs: Pair<Rule>) -> Vec<LimpValue> {
	println!("expr list time!!!");
	println!("{:?}", exprs);
	let mut ret = Vec::new();

	for expr in exprs.into_inner() {
		ret.push(eval_expr(expr));
	}

	ret
}

// expr :: atom | invocation
fn eval_expr(expr: Pair<Rule>) -> LimpValue {
	let mut ret = LimpValue::ErrorValue; // TODO: clean this up
	println!("expr time!!!");
	println!("{:?}", expr);

	for inner_pair in expr.into_inner() {
		match inner_pair.as_rule() {
			Rule::atom => ret = eval_atom(inner_pair),
			Rule::invocation => ret = eval_invocation(inner_pair),
			_ => unreachable!()
		}
	}

	ret
}
// atom ::= float | int | name
fn eval_atom(atom: Pair<Rule>) -> LimpValue {
	println!("atom time!!!");
	println!("{:?}", atom);

	let mut ret = LimpValue::ErrorValue;

	/*for pair in atom {
		match pair.as_rule() {
			Rule::float => ret = eval_float(pair.into_inner()),
			Rule::int => ret = eval_int(pair.into_inner()),
			Rule::name => ret = eval_name(pair.into_inner()),
			_ => unreachable!()
		}
	}*/

	for inner_pair in atom.into_inner() {
		match inner_pair.as_rule() {
			Rule::float => ret = eval_float(inner_pair),
			Rule::int => ret = eval_int(inner_pair),
			Rule::name => ret = eval_name(inner_pair),
			_ => unreachable!()
		}
	}

	println!("{:?}", ret);

	ret
}

fn eval_float(float: Pair<Rule>) -> LimpValue {
	LimpValue::ErrorValue // TODO
}

fn eval_int(int: Pair<Rule>) -> LimpValue {
	println!("int time!!!");
	println!("{:?}", int);
	LimpValue::ErrorValue // TODO
}

fn eval_name(name: Pair<Rule>) -> LimpValue {
	LimpValue::ErrorValue // TODO
}

fn eval_invocation(invocation: Pair<Rule>) -> LimpValue {
	LimpValue::ErrorValue // TODO
}
