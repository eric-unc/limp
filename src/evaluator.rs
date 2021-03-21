
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
	for pair in tree {

		for inner_pair in pair.into_inner() {
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
	let mut ret = Vec::new();

	for expr in exprs.into_inner() {
		ret.push(eval_expr(expr));
	}

	ret
}

// expr :: atom | invocation
fn eval_expr(expr: Pair<Rule>) -> LimpValue {
	let mut ret = LimpValue::ErrorValue; // TODO: clean this up

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
	let mut ret = LimpValue::ErrorValue;

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
	LimpValue::Float(float.as_span().as_str().parse::<f64>().unwrap())
}

fn eval_int(int: Pair<Rule>) -> LimpValue {
	LimpValue::Integer(int.as_span().as_str().parse::<u64>().unwrap())
}

fn eval_name(name: Pair<Rule>) -> LimpValue {
	LimpValue::Name(name.as_span().as_str().parse().unwrap())
}

fn eval_invocation(invocation: Pair<Rule>) -> LimpValue {
	LimpValue::ErrorValue // TODO
}
