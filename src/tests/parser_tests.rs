use crate::LimpParser;
use crate::Rule;

use crate::pest;

use pest::*;

//// Core

#[test]
fn parse_atom(){
	parses_to! {
		parser: LimpParser,
		input: "print",
		rule: Rule::atom,
		tokens: [
			atom(0, 5, [
				name(0, 5)
			])
		]
	}

	parses_to! {
		parser: LimpParser,
		input: "-1",
		rule: Rule::atom,
		tokens: [
			atom(0, 2, [
				int(0, 2)
			])
		]
	}
}

#[test]
fn parse_invocation(){
	parses_to! {
		parser: LimpParser,
		input: "(+ 5 5)",
		rule: Rule::invocation,
		tokens: [
			invocation(0, 7, [
				expr_list(1, 6, [
					expr(1, 2, [
						atom(1, 2, [
							name(1, 2),
						])]
					),
					expr(3, 4, [
						atom(3, 4, [
							int(3, 4),
						])]
					),
					expr(5, 6, [
						atom(5, 6, [
							int(5, 6),
						])]
					)
				])
			])
		]
	}
}

#[test]
fn parse_if_form(){
	parses_to! {
		parser: LimpParser,
		input: "(if true 1 0)",
		rule: Rule::if_form,
		tokens: [
			if_form(0, 13, [
				expr(4, 8, [
					atom(4, 8, [
						boolean(4, 8)
					])]
				),
				expr(9, 10, [
					atom(9, 10, [
						int(9, 10)
					])]
				),
				expr(11, 12, [
					atom(11, 12, [
						int(11, 12)
					])]
				)
			])
		]
	}
}

//// Atoms

#[test]
fn parse_int(){
	parses_to! {
		parser: LimpParser,
		input: "5",
		rule: Rule::int,
		tokens: [
			int(0, 1)
		]
	}

	parses_to! {
		parser: LimpParser,
		input: "100",
		rule: Rule::int,
		tokens: [
			int(0, 3)
		]
	}

	parses_to! {
		parser: LimpParser,
		input: "-1",
		rule: Rule::int,
		tokens: [
			int(0, 2)
		]
	}
}

#[test]
fn parse_float(){
	parses_to! {
		parser: LimpParser,
		input: "1.0",
		rule: Rule::float,
		tokens: [
			float(0, 3)
		]
	}

	parses_to! {
		parser: LimpParser,
		input: "-3.14",
		rule: Rule::float,
		tokens: [
			float(0, 5)
		]
	}
}

#[test]
fn parse_boolean(){
	parses_to! {
		parser: LimpParser,
		input: "true",
		rule: Rule::boolean,
		tokens: [
			boolean(0, 4)
		]
	}

	parses_to! {
		parser: LimpParser,
		input: "false",
		rule: Rule::boolean,
		tokens: [
			boolean(0, 5)
		]
	}
}

#[test]
fn parse_name(){
	parses_to! {
		parser: LimpParser,
		input: "-",
		rule: Rule::name,
		tokens: [
			name(0, 1)
		]
	}

	parses_to! {
		parser: LimpParser,
		input: "print",
		rule: Rule::name,
		tokens: [
			name(0, 5)
		]
	}
}
