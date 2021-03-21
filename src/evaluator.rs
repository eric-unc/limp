use pest::iterators::{Pairs, Pair};
use std::collections::HashMap;
use crate::Rule;
use std::process::exit;

#[derive(Debug)]
pub enum LimpValue {
	Integer(i64),
	Float(f64),
	Boolean(bool),
	Name(String),
	VoidValue,
	ErrorValue(String) // accepted an error description
}

impl PartialEq for LimpValue {
	fn eq(&self, other: &Self) -> bool {
		match self {
			Integer(i) =>
				match other {
					Integer(i2) => *i == *i2,
					Float(f) => *i as f64 == *f,
					_ => false
				}
			Float(f) =>
				match other {
					Integer(i) => *f == *i as f64,
					Float(f2) => *f == *f2,
					_ => false
				}
			Boolean(b) =>
				match other {
					Boolean(b2) => *b == *b2,
					_ => false
				}
			Name(n) =>
				match other {
					Name(n2) => *n == *n2,
					_ => false
				}
			VoidValue =>
				match other {
					VoidValue => true,
					_ => false
				}
			ErrorValue =>
				match other {
					ErrorValue => true,
					_ => false
				}
		}
	}

	fn ne(&self, other: &Self) -> bool {
		!self.eq(other)
	}
}

use crate::evaluator::LimpValue::*;

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

pub fn eval(tree: Pairs<Rule>){
	eval_with_env(tree, &Environment::new());
}

pub fn eval_with_env(tree: Pairs<Rule>, env: &Environment){
	eval_program(tree);
}

// program ::= expr_list
fn eval_program(tree: Pairs<Rule>){
	for pair in tree {

		for inner_pair in pair.into_inner() {
			match inner_pair.as_rule() {
				Rule::expr_list => {
					eval_expr_list(inner_pair);
				},
				Rule::EOI => {},
				_ => {
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

// expr :: atom | if_form | invocation
fn eval_expr(expr: Pair<Rule>) -> LimpValue {
	for inner_pair in expr.into_inner() {
		match inner_pair.as_rule() {
			Rule::atom => return eval_atom(inner_pair),
			Rule::if_form => return eval_if_form(inner_pair),
			Rule::invocation => return eval_invocation(inner_pair),
			_ => unreachable!()
		}
	}

	unreachable!()
}
// atom ::= float | int | bool | name
fn eval_atom(atom: Pair<Rule>) -> LimpValue {
	for inner_pair in atom.into_inner() {
		match inner_pair.as_rule() {
			Rule::float => return eval_float(inner_pair),
			Rule::int => return eval_int(inner_pair),
			Rule::boolean => return eval_boolean(inner_pair),
			Rule::name => return eval_name(inner_pair),
			_ => unreachable!()
		}
	}

	unreachable!();
}

fn eval_float(float: Pair<Rule>) -> LimpValue {
	match float.as_span().as_str().parse::<f64>() {
		Ok(value) => { return Float(value) }
		Err(err) => { return ErrorValue(err.to_string()) }
	}
}

fn eval_int(int: Pair<Rule>) -> LimpValue {
	match int.as_span().as_str().parse::<i64>() {
		Ok(value) => { return Integer(value) }
		Err(err) => { return ErrorValue(err.to_string()) }
	}
}

fn eval_boolean(boolean: Pair<Rule>) -> LimpValue {
	match boolean.as_span().as_str() {
		"true" => Boolean(true),
		"false" => Boolean(false),
		_ => panic!()
	}
}

fn eval_name(name: Pair<Rule>) -> LimpValue {
	match name.as_span().as_str().parse() {
		Ok(value) => { return Name(value) }
		Err(err) => { return ErrorValue(err.to_string()) }
	}
}

// if_form ::= (if expr expr expr)
fn eval_if_form(if_form: Pair<Rule>) -> LimpValue {
	let mut iter = if_form.into_inner();

	let cond = eval_expr(iter.next().unwrap());

	let if_true = iter.next().unwrap();
	let else_if = iter.next().unwrap();

	match cond {
		Boolean(b) => {
			if b {
				eval_expr(if_true)
			} else {
				eval_expr(else_if)
			}
		}
		_ => ErrorValue
	}
}

// invocation ::= ( expr_list )
fn eval_invocation(invocation: Pair<Rule>) -> LimpValue {
	for inner_pair in invocation.into_inner() {
		let rators_and_rands = eval_expr_list(inner_pair);

		let rator = &rators_and_rands[0];
		let rands = &rators_and_rands[1..rators_and_rands.len()];

		match rator {
			Name(n) => {
				match n.as_str() {
					"+" => {
						if rands.len() < 2 {
							return ErrorValue("Rator `+` expects at least 2 rands!".to_string());
						}

						let mut ret_val = 0.0;

						for rand in rands {
							match rand {
								LimpValue::Integer(i) => { ret_val += *i as f64; }
								LimpValue::Float(f) => { ret_val += *f; }
								// TODO: implement bindings
								_ => { return ErrorValue(format!("Bad type of {:?} for +!", rand)) }
							}
						}

						return f_to_i_if_possible(ret_val);
					},
					"-" => {
						if rands.len() < 2 {
							return ErrorValue("Rator `-` expects at least 2 rands!".to_string());
						}

						let mut ret_val = 0.0;
						let mut ret_init = false;

						for rand in rands {
							match rand {
								Integer(i) => {
									if !ret_init {
										ret_val = *i as f64;
										ret_init = true;
									} else {
										ret_val -= *i as f64;
									}
								},
								Float(f) => {
									if !ret_init {
										ret_val = *f;
										ret_init = true;
									} else {
										ret_val -= *f;
									}
								}
								// TODO: implement bindings
								_ => { return ErrorValue(format!("Bad type of {:?} for -!", rand))}
							}
						}

						return f_to_i_if_possible(ret_val);
					},
					"*" => {
						if rands.len() < 2 {
							return ErrorValue("Rator `*` expects at least 2 rands!".to_string());
						}

						let mut ret_val = 1.0;

						for rand in rands {
							match rand {
								Integer(i) => { ret_val *= *i as f64; }
								Float(f) => { ret_val *= *f; }
								// TODO: implement bindings
								_ => { return ErrorValue(format!("Bad type of {:?} for *!", rand))}
							}
						}

						return f_to_i_if_possible(ret_val);
					},
					"/" => {
						if rands.len() < 2 {
							return ErrorValue("Rator `/` expects at least 2 rands!".to_string());
						}

						let mut ret_val = 0.0;
						let mut ret_init = false;

						for rand in rands {
							match rand {
								Integer(i) => {
									if !ret_init {
										ret_val = *i as f64;
										ret_init = true;
									} else {
										ret_val /= *i as f64;
									}
								},
								Float(f) => {
									if !ret_init {
										ret_val = *f;
										ret_init = true;
									} else {
										ret_val /= *f;
									}
								}
								// TODO: implement bindings
								_ => { return ErrorValue(format!("Bad type of {:?} for /!", rand))}
							}
						}

						return f_to_i_if_possible(ret_val);
					},
					"and" => {
						if rands.len() < 2 {
							panic!("Rator `and` expects at least 2 rands!");
						}

						for rand in rands {
							match rand {
								Boolean(b) => {
									if !*b {
										return Boolean(false);
									}
								}
								// TODO: implement bindings
								_ => { panic!("Bad type of {:?} for and!", rand)}
							}
						}

						return Boolean(true);
					},
					"or" => {
						if rands.len() < 2 {
							panic!("Rator `or` expects at least 2 rands!");
						}

						for rand in rands {
							match rand {
								Boolean(b) => {
									if *b {
										return Boolean(true);
									}
								}
								// TODO: implement bindings
								_ => { panic!("Bad type of {:?} for or!", rand)}
							}
						}

						return Boolean(false);
					},
					"xor" => {
						if rands.len() < 2 {
							panic!("Rator `xor` expects at least 2 rands!");
						}

						// Wikipedia: "[xor] may be considered to be an n-ary operator which is true if and only if an odd number of arguments are true"
						let mut trues = 0;

						for rand in rands {
							match rand {
								Boolean(b) => {
									if *b {
										trues += 1;
									}
								}
								// TODO: implement bindings
								_ => { panic!("Bad type of {:?} for xor!", rand)}
							}
						}

						return Boolean(if trues % 2 == 0 { false } else { true });
					},
					"not" => {
						if rands.len() != 1 {
							panic!("Rator `not` only supports a single rand!");
						}

						for rand in rands {
							match rand {
								Boolean(b) => { return Boolean(!*b); }
								// TODO: implement bindings
								_ => { panic!("Bad type of {:?} for not!", rand)}
							}
						}

						unreachable!();
					},
					"==" => {
						if rands.len() != 2 {
							panic!("Rator `==` only supports 2 rands!");
						}

						return Boolean(rands[0] == rands[1]);
					},
					"!=" => {
						if rands.len() != 2 {
							panic!("Rator `!=` only supports 2 rands!");
						}

						return Boolean(rands[0] != rands[1]);
					},
					"print" => {
						if rands.len() < 1 {
							return ErrorValue("Rator `print` expects at least 1 rand!".to_string());
						}

						for rand in rands {
							match rand {
								Integer(i) => { println!("{}", i) }
								Float(f) => { println!("{}", f) }
								Boolean(b) => { println!("{}", b) }
								// TODO: implement bindings

								ErrorValue(s) => { println!("{}", s) }
								_ => { return ErrorValue(format!("Bad type of {:?} for print!", rand))}
							}
						}

						return VoidValue;
					},
					"exit" => {
						match rands.len() {
							0 => exit(0),
							1 => {
								match rands[0] {
									Integer(i) => { exit(i as i32) }
									Float(f) => { exit(f as i32) }
									// TODO: implement bindings
									_ => { return ErrorValue(format!("Bad type of {:?} for exit!", rands[0])) }
								}
							},
							_ => return ErrorValue("Rator `exit` expects at least 1 rand!".to_string())
						}
					},
					_ => { return ErrorValue(format!("Unexpected rator {:?}!", n.as_str())) }
				}
			}
			_ => { return ErrorValue(format!("Unexpected rator {:?}!", &rators_and_rands[0])) }
		}
	}

	unreachable!()
}

fn f_to_i_if_possible(float: f64) -> LimpValue {
	let int = float as i64;

	if float != int as f64 {
		Float(float)
	} else {
		Integer(int)
	}
}
