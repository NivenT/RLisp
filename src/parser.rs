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
	} else if tkn == "T".to_string() {
		T
	} else {
		SYMBOL(tkn)
	}
}

fn str_replace(s: &String, reps: Vec<(&str, &str)>) -> String {
	let mut ret = s.clone();
	for r in reps {
		ret = ret.replace(r.0, r.1)
	}
	ret
}

pub fn tokenize(str: &String) -> Vec<Atom> {
	str_replace(&str.to_uppercase(), vec![("(", " ( "), (")", " ) "),
						  				  ("'", " ' "), ("`", " ` "),
						  				  ("[", " [ "), ("]", " ] ")])
	   .split_whitespace().map(|s| atomize(s.to_string()))
	   .collect()
}

pub fn parse(tkns: &mut Vec<Atom>) -> Datum {
	match tkns.pop().unwrap_or(SYMBOL("NIL".to_string())) {
		SYMBOL(s)	=> {
			if s==")" || s=="]" {
				let mut lst = NIL;
				while match *tkns.last().unwrap() 
					{SYMBOL(ref s) => s!="(" && s!="[", _ => true} {
						lst = CONS(Box::new(parse(tkns)),
								   Box::new(LIST(lst)))
					}
				tkns.pop(); //get rid of "("
				LIST(lst)
			} else {
				ATOM(SYMBOL(s))
			}
		},
		e @ _ 		=> {
			ATOM(e)
		}
	}
}