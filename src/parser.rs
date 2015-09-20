use types::*;

use types::Number::*;
use types::Datum::*;
use types::Atom::*;
use types::List::*;

fn atomize(tkn: String) -> Atom {
	if tkn.parse::<i64>().is_ok() {
		NUMBER(INTEGER(tkn.parse::<i64>().unwrap()))
	} else if tkn.parse::<f64>().is_ok() {
		NUMBER(REAL(tkn.parse::<f64>().unwrap()))
	} else if tkn.starts_with("\"") && tkn.ends_with("\"") {
		unsafe {
			STRING(tkn.slice_unchecked(1,tkn.len()-1).to_string())
		}
	} else if tkn.find('/') != None {
		let nums: Vec<String> = tkn.split('/').map(|s| s.to_string())
								   .collect();
		if nums[0].parse::<i64>().is_ok() && 
		   nums[1].parse::<i64>().is_ok() && 
		   nums.len() == 2 {
		   	NUMBER(RATIONAL(nums[0].parse::<i64>().unwrap(),
		   					nums[1].parse::<i64>().unwrap()).simplify())
		} else {SYMBOL(tkn)}
	} else if tkn.to_uppercase() == "T" {
		T
	} else {
		SYMBOL(tkn.to_uppercase())
	}
}

pub fn tokenize(s: &String) -> Vec<Atom> {
	let mut curr = String::new();
	let mut tkns: Vec<Atom> = Vec::new();
	for character in s.chars() {
		match character {
			'"' if curr.starts_with('"') => {
				curr.push('"');
				tkns.push(atomize(curr.clone()));
				curr = String::from("");
			}
			'"' => {
				curr = String::from("\"");
			}
			c if curr.starts_with('"') => {
				curr.push(c);
			}
			c if vec!['(', ')', '\'', '`', ',', '[', ']'].contains(&c) => {
				if !curr.is_empty() {
					tkns.push(atomize(curr.clone()))
				}
				tkns.push(atomize(c.to_string()));
				curr = String::from("");
			}
			w if w.is_whitespace() => {
				if !curr.is_empty() {
					tkns.push(atomize(curr.clone()))
				}
				curr = String::from("")
			}
			c => {
				curr.push(c);
			}
		}
	}
	if !curr.is_empty() {
		tkns.push(atomize(curr.clone()))
	}
	tkns.clone()
}

pub fn parse(tkns: &mut Vec<Atom>) -> Datum {
	match tkns.remove(0) {
		SYMBOL(s)	=> {
			if s=="(" || s=="[" {
				let mut lst: Vec<Datum> = vec![];
				while match tkns[0] 
					{SYMBOL(ref s) => s!=")" && s!="]", _ => true} {
						lst.push(parse(tkns))
					}
				tkns.remove(0); //get rid of "("
				LIST(List::from_vec(lst))
			} else if s=="'" {
				LIST(CONS(
					Box::new(ATOM(SYMBOL("QUOTE".to_string()))),
					Box::new(LIST(CONS(
						Box::new(parse(tkns)),
						Box::new(LIST(NIL)))))))
			} else if s=="`" {
				LIST(CONS(
					Box::new(ATOM(SYMBOL("BACKQUOTE".to_string()))),
					Box::new(LIST(CONS(
						Box::new(parse(tkns)),
						Box::new(LIST(NIL)))))))
			} else if s=="," {
				LIST(CONS(
					Box::new(ATOM(SYMBOL("COMMA".to_string()))),
					Box::new(LIST(CONS(
						Box::new(parse(tkns)),
						Box::new(LIST(NIL)))))))
			} else if s=="NIL" {
				LIST(NIL)
			} else {
				ATOM(SYMBOL(s))
			}
		},
		e @ _ 		=> {
			ATOM(e)
		}
	}
}