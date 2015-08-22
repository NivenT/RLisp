extern crate time;

mod native;
mod parser;
mod errors;
mod types;
mod eval;
mod env;

use parser::*;
use errors::*;
use types::*;
use eval::*;
use env::*;

use std::io;
use std::io::prelude::*;
use time::PreciseTime;

fn matched_parentheses(s: &String) -> Option<bool> {
	let mut stack: Vec<char> = vec![];
	for c in s.chars() {
		if c=='(' || c=='[' {
			stack.push(c)
		} else if c==')' {
			if stack.pop().unwrap_or(' ') != '(' {
				return None
			}
		} else if c==']' {
			if stack.pop().unwrap_or(' ') != '[' {
				return None
			}
		}
	}
	Some(stack.is_empty())
}

fn main() {
	let mut input = String::new();
	let mut result: Result<Datum, LispError>;
	let mut env = Env::new();

	eval(&parse(&mut tokenize(&"(load \"std.rlisp\")".to_string())), &mut env);
	loop {
		input.clear();
		result =  Ok(Datum::LIST(List::NIL));

		print!("RLisp>> "); io::stdout().flush().ok().expect("Could not flush stdout");
		io::stdin().read_line(&mut input).ok()
				   .expect("Failed to read line");
		loop {
			match matched_parentheses(&input) {
				Some(finished) 	=> {
					if !finished {
						print!("\t"); io::stdout().flush().ok().expect("Could not flush stdout");

						let mut next_line = String::new();
						io::stdin().read_line(&mut next_line).ok()
								   .expect("Failed to read line");
						input = format!("{}\t\n{}", input, next_line);
					} else if input == "\r\n".to_string() {
						result = Err(LispError::NO_INPUT); break
					} else {break}
				},
				None			=> {result = Err(LispError::MISMATCHED_BRACKETS); break}
			}
		}
		if result != Err(LispError::MISMATCHED_BRACKETS) &&
		   result != Err(LispError::NO_INPUT) {
			result = eval(&parse(&mut tokenize(&input)), &mut env)
		} match result {
			Ok(ref a) 	=> println!("{}\n", *a),
			Err(ref a)	=> println!("{}\n", a.message())
		}
		env.reset();
		/*
		println!("Development output:");
		println!("	tokens: {:?}\n", tokenize(&input));
		println!("	parsed expression: {}\n", parse(&mut tokenize(&input)));
		println!("	debug result: {:?}\n", result);
		println!("\n");
		*/
	}
}
