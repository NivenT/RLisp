extern crate time;
extern crate rand;
extern crate term_painter;

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

use errors::LispError::*;

use term_painter::ToStyle;
use term_painter::Color::*;

use rand::Rng;
use std::io;
use std::io::prelude::*;

fn levenshtein(s1: &String, s2: &String, sofar: usize, cap: usize) -> usize {
	if s1 == s2 {
		return sofar;
	} else if s1.len() == 0 || s2.len() == 0 {
		return sofar+s1.len()+s2.len();
	} else if sofar >= cap {
		cap
	} else {
		let cost = if s1.chars().next() == s2.chars().next() {0} else {1};
		vec![levenshtein(&s1[1..].to_string(), &s2, 1+sofar, cap),
		   	 levenshtein(&s1, &s2[1..].to_string(), 1+sofar, cap),
		   	 levenshtein(&s1[1..].to_string(), &s2[1..].to_string(),
		   	 			 sofar+cost,cap)]
			.into_iter().min().unwrap()
	}
}

fn matched_parentheses(s: &String) -> Option<bool> {
	let mut stack: Vec<char> = vec![];
	let mut in_string = false;
	for c in s.chars() {
		if (c=='(' || c=='[') && !in_string {
			stack.push(c)
		} else if c==')' && !in_string {
			if stack.pop().unwrap_or(' ') != '(' {
				return None
			}
		} else if c==']' && !in_string {
			if stack.pop().unwrap_or(' ') != '[' {
				return None
			}
		} else if c=='"' {
			in_string = !in_string
		}
	}
	Some(stack.is_empty())
}

fn main() {
	let mut input = String::new();
	let mut result: Result<Datum, LispError>;
	let mut env = Env::new();

	eval(&parse(&mut tokenize(&"(load \"std.rlisp\")".to_string())),
		 &mut env).ok();
	env.push();
	loop {
		input.clear();
		result = Ok(Datum::LIST(List::NIL));

		print!("{}", BrightCyan.paint("RLisp>> ")); 
		io::stdout().flush().ok().expect("Could not flush stdout");
		io::stdin().read_line(&mut input).ok().expect("Failed to read line");
		loop {
			match matched_parentheses(&input) {
				Some(finished) 	=> {
					if !finished {
						print!("\t"); io::stdout().flush().ok().expect("Could not flush stdout");

						let mut next_line = String::new();
						io::stdin().read_line(&mut next_line).ok()
								   .expect("Failed to read line");
						input = format!("{}\n\t{}", input, next_line);
					} else if input.trim() == "".to_string() {
						result = Err(NO_INPUT); break
					} else {break}
				},
				None			=> {result = Err(MISMATCHED_BRACKETS); break}
			}
		}
		if result != Err(MISMATCHED_BRACKETS) &&
		   result != Err(NO_INPUT) {
			result = eval(&parse(&mut tokenize(&input)), &mut env)
		} match result {
			Ok(ref a) 	=> {println!("{}", BrightYellow.paint(a.clone()));env.set("%%%".to_string(), a.clone());},
			Err(ref a)	=> println!("{}", Blue.paint(a.message()))
		} if let Err(UNBOUND_VARIABLE(name)) = result.clone() {
			let mut min = ("".to_string(), 99999);
			for (key, _) in env.join() {
				let score = levenshtein(&key, &name, 0, 3);
				if score < min.1 || (score == min.1 && rand::thread_rng().gen_range(0,2) == 1) {
					min = (key, score);
				}
			}
			if min.1 <= 2 {
				println!("{}{}{}?", Blue.paint("Did you mean '"), Green.paint(min.0), Blue.paint("'"));
			} /*else {
				println!("Did you mean '(QUOTE {})'?", name);
			}*/
		}
		println!("");
		
		/*
		println!("Development output:");
		println!("	tokens: {:?}\n", tokenize(&input));
		println!("	parsed expression: {}\n", parse(&mut tokenize(&input)));
		println!("	debug result: {:?}\n", result);
		println!("\n");
		*/
	}
}
