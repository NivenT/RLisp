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
		   nums[1].parse::<i64>().is_ok() {
		   	NUMBER(RATIONAL(nums[0].parse::<i64>().unwrap(),
		   					nums[1].parse::<i64>().unwrap()).simplify())
		} else {SYMBOL(tkn)}
	} else if tkn.to_uppercase() == "T" {
		T
	} else {
		SYMBOL(tkn.to_uppercase())
	}
}

pub fn tokenize(str: &String) -> Vec<Atom> {
	tokenize_helper(&mut str.clone(), &mut vec![], &mut String::new())
}

pub fn tokenize_helper(str: &mut String, tkns: &mut Vec<Atom>, curr: &mut String) -> Vec<Atom> {
	match str.pop() {
		Some('"') if curr.ends_with('"') => {
			curr.insert(0, '"');
			tkns.insert(0, atomize(curr.clone()));
			tokenize_helper(str, tkns, &mut "".to_string())
		},
		Some('"') => tokenize_helper(str, tkns, &mut "\"".to_string()),
		Some(c) if curr.ends_with('"') => {
			curr.insert(0, c);
			tokenize_helper(str, tkns, curr)
		}
		Some(c) if vec!['(', ')', '\'', '`', ',', '[', ']'].contains(&c)	=> {
			if !curr.is_empty() {
				tkns.insert(0, atomize(curr.clone()));
			}
			tkns.insert(0, atomize(c.to_string()));
			tokenize_helper(str, tkns, &mut "".to_string())
		},
		Some(w) if w.is_whitespace() => {
			if !curr.is_empty() {
				tkns.insert(0, atomize(curr.clone()));
			}
			tokenize_helper(str, tkns, &mut "".to_string())
		},
		Some(c)	=> {
			curr.insert(0, c);
			tokenize_helper(str, tkns, curr)
		},
		None => {
			if !curr.is_empty() {
				tkns.insert(0, atomize(curr.clone()));
			}
			tkns.clone()
		}
	}
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