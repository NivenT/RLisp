extern crate rand;

use parser::*;
use errors::*;
use types::*;

use errors::LispError::*;
use types::Function::*;
use types::Number::*;
use types::Datum::*;
use types::List::*;
use types::Atom::*;

use rand::Rng;

use term_painter::ToStyle;
use term_painter::Color::*;

use std::fmt;

pub fn add(args: Vec<Datum>) -> Result<Datum, LispError> {
	let mut sum = INTEGER(0);
	for item in args {
		match item {
			ATOM(NUMBER(n))	=> {sum = sum + n},
			_				=> return Err(INVALID_ARGUMENT_TYPE(item, "number"))
		}
	}
	Ok(ATOM(NUMBER(sum.simplify())))
}

pub fn sub(args: Vec<Datum>) -> Result<Datum, LispError> {
	if args.len() == 0 {
		Err(INVALID_NUMBER_OF_ARGS(0,1))
	} else {
		match args[0] {
			ATOM(NUMBER(n))	=> {
				if args.len() == 1 {
					return Ok(ATOM(NUMBER(-n)))
				}
				let res = add(args[1..].to_vec());
				if res.is_err() {
					return res;
				}
				match res.ok().unwrap() {
					ATOM(NUMBER(m))	=> Ok(ATOM(NUMBER((n-m).simplify()))),
					ref e @ _		=> Err(INVALID_ARGUMENT_TYPE(e.clone(), "number"))
				}
			},
			_				=> Err(INVALID_ARGUMENT_TYPE(args[0].clone(), "number"))
		}
	}
}

pub fn mul(args: Vec<Datum>) -> Result<Datum, LispError> {
	let mut prd = INTEGER(1);
	for item in args {
		match item {
			ATOM(NUMBER(n))	=> {prd = prd * n},
			_				=> return Err(INVALID_ARGUMENT_TYPE(item, "number"))
		}
	}
	Ok(ATOM(NUMBER(prd.simplify())))
}

pub fn div(args: Vec<Datum>) -> Result<Datum, LispError> {
	if args.len() == 0 {
		Err(INVALID_NUMBER_OF_ARGS(0,1))
	} else {
		match args[0] {
			ATOM(NUMBER(n))	=> {
				if args.len() == 1 {
					return Ok(ATOM(NUMBER(INTEGER(1)/n)))
				}
				let res = mul(args[1..].to_vec());
				if res.is_err() {
					return res;
				}
				match res.ok().unwrap() {
					ATOM(NUMBER(m)) if m.val()==0. => Err(DIVISION_BY_ZERO),
					ATOM(NUMBER(m))	=> Ok(ATOM(NUMBER((n/m).simplify()))),
					ref e @ _		=> Err(INVALID_ARGUMENT_TYPE(e.clone(), "number"))
				}
			},
			_				=> Err(INVALID_ARGUMENT_TYPE(args[0].clone(), "number"))
		}
	}
}

pub fn list(args: Vec<Datum>) -> Result<Datum, LispError> {
	Ok(LIST(List::from_vec(args)))
}

pub fn cons(args: Vec<Datum>) -> Result<Datum, LispError> {
	match args.len() {
		2 		=> Ok(LIST(CONS(Box::new(args[0].clone()),
								Box::new(args[1].clone())))),
		e @ _	=> Err(INVALID_NUMBER_OF_ARGS(e, 2))
	}
}

pub fn car(args: Vec<Datum>) -> Result<Datum, LispError> {
	match args.len() {
		1 => {
			match args[0] {
				LIST(ref l)	=> Ok(l.car()),
				_			=> Err(INVALID_ARGUMENT_TYPE(args[0].clone(), "list"))
			}
		},
		e @ _ => Err(INVALID_NUMBER_OF_ARGS(e, 1))
	}
}

pub fn cdr(args: Vec<Datum>) -> Result<Datum, LispError> {
	match args.len() {
		1 => {
			match args[0] {
				LIST(ref l)	=> Ok(l.cdr()),
				_			=> Err(INVALID_ARGUMENT_TYPE(args[0].clone(), "list"))
			}
		},
		e @ _ => Err(INVALID_NUMBER_OF_ARGS(e, 1))
	}	
}

pub fn nth(args: Vec<Datum>) -> Result<Datum, LispError> {
	match args.len() {
		2 => {
			if let ATOM(NUMBER(INTEGER(n))) = args[0] {
				let n = n as usize;
				match args[1] {
					LIST(ref l)	=> return if l.get_items().len() > n {
						Ok(l.get_items()[n].clone())
					} else {Ok(LIST(NIL))},
					_		=> return Err(INVALID_ARGUMENT_TYPE(args[1].clone(), "list"))
				}
			}
			Err(INVALID_ARGUMENT_TYPE(args[0].clone(), "integer"))
		},
		e @ _ => Err(INVALID_NUMBER_OF_ARGS(e, 2))
	}
}

pub fn nth_cdr(args: Vec<Datum>) -> Result<Datum, LispError> {
	match args.len() {
		2 => {
			if let ATOM(NUMBER(INTEGER(n))) = args[0] {
				let n = n as usize;
				match args[1] {
					LIST(ref l)	=> return if l.get_items().len() > n {
						Ok(LIST(List::from_vec(
							l.get_items().into_iter().skip(n).collect())))
					} else {Ok(LIST(NIL))},
					_		=> return Err(INVALID_ARGUMENT_TYPE(args[1].clone(), "list"))
				}
			}
			Err(INVALID_ARGUMENT_TYPE(args[0].clone(), "integer"))
		},
		e @ _ => Err(INVALID_NUMBER_OF_ARGS(e, 2))
	}
}

pub fn greater_than(args: Vec<Datum>) -> Result<Datum, LispError> {
	for i in 1..args.len() {
		if let ATOM(NUMBER(a)) = args[i-1] {
			if let ATOM(NUMBER(b)) = args[i] {
				if a <= b {
					return Ok(LIST(NIL));
				}
			} else {
				return Err(INVALID_ARGUMENT_TYPE(args[i].clone(), "number"));
			}
		} else {
			return Err(INVALID_ARGUMENT_TYPE(args[i-1].clone(), "number"));
		}
	}
	Ok(ATOM(T))
}

pub fn greater_equal(args: Vec<Datum>) -> Result<Datum, LispError> {
	for i in 1..args.len() {
		if let ATOM(NUMBER(a)) = args[i-1] {
			if let ATOM(NUMBER(b)) = args[i] {
				if a < b {
					return Ok(LIST(NIL));
				}
			} else {
				return Err(INVALID_ARGUMENT_TYPE(args[i].clone(), "number"));
			}
		} else {
			return Err(INVALID_ARGUMENT_TYPE(args[i-1].clone(), "number"));
		}
	}
	Ok(ATOM(T))
}

pub fn less_than(args: Vec<Datum>) -> Result<Datum, LispError> {
	for i in 1..args.len() {
		if let ATOM(NUMBER(a)) = args[i-1] {
			if let ATOM(NUMBER(b)) = args[i] {
				if a >= b {
					return Ok(LIST(NIL));
				}
			} else {
				return Err(INVALID_ARGUMENT_TYPE(args[i].clone(), "number"));
			}
		} else {
			return Err(INVALID_ARGUMENT_TYPE(args[i-1].clone(), "number"));
		}
	}
	Ok(ATOM(T))
}

pub fn less_equal(args: Vec<Datum>) -> Result<Datum, LispError> {
	for i in 1..args.len() {
		if let ATOM(NUMBER(a)) = args[i-1] {
			if let ATOM(NUMBER(b)) = args[i] {
				if a > b {
					return Ok(LIST(NIL));
				}
			} else {
				return Err(INVALID_ARGUMENT_TYPE(args[i].clone(), "number"));
			}
		} else {
			return Err(INVALID_ARGUMENT_TYPE(args[i-1].clone(), "number"));
		}
	}
	Ok(ATOM(T))
}

pub fn math_equal(args: Vec<Datum>) -> Result<Datum, LispError> {
	for i in 1..args.len() {
		if let ATOM(NUMBER(a)) = args[i-1] {
			if let ATOM(NUMBER(b)) = args[i] {
				if a.simplify() != b.simplify() && a.val() != b.val() {
					return Ok(LIST(NIL));
				}
			} else {
				return Err(INVALID_ARGUMENT_TYPE(args[i].clone(), "number"));
			}
		} else {
			return Err(INVALID_ARGUMENT_TYPE(args[i-1].clone(), "number"));
		}
	}
	Ok(ATOM(T))
}

pub fn lisp_mod(args: Vec<Datum>) -> Result<Datum, LispError> {
	if args.len() != 2 {
		Err(INVALID_NUMBER_OF_ARGS(args.len(), 2))
	} else if let ATOM(NUMBER(a)) = args[0] {
		if let ATOM(NUMBER(b)) = args[1] {
			if b.val() == 0. {
				Err(DIVISION_BY_ZERO)
			} else {
				Ok(ATOM(NUMBER(
					a - b*INTEGER((a/b).val().floor() as i64))))
			}
		} else {
			Err(INVALID_ARGUMENT_TYPE(args[1].clone(), "number"))
		}
	} else {
		Err(INVALID_ARGUMENT_TYPE(args[0].clone(), "number"))
	}
}

pub fn powi(args: Vec<Datum>) -> Result<Datum, LispError> {
	if args.len() != 2 {
		Err(INVALID_NUMBER_OF_ARGS(args.len(), 2))
	} else if let ATOM(NUMBER(a)) = args[0] {
		if let ATOM(NUMBER(INTEGER(b))) = args[1] {
			Ok(ATOM(NUMBER(REAL(
				a.val().powi(b as i32)).simplify())))
		} else {
			Err(INVALID_ARGUMENT_TYPE(args[1].clone(), "integer"))
		}
	} else {
		Err(INVALID_ARGUMENT_TYPE(args[0].clone(), "number"))
	}
}

pub fn powr(args: Vec<Datum>) -> Result<Datum, LispError> {
	if args.len() != 2 {
		Err(INVALID_NUMBER_OF_ARGS(args.len(), 2))
	} else if let ATOM(NUMBER(a)) = args[0] {
		if let ATOM(NUMBER(b)) = args[1] {
			Ok(ATOM(NUMBER(REAL(
				a.val().powf(b.val())).simplify())))
		} else {
			Err(INVALID_ARGUMENT_TYPE(args[1].clone(), "number"))
		}
	} else {
		Err(INVALID_ARGUMENT_TYPE(args[0].clone(), "number"))
	}
}

pub fn floor(args: Vec<Datum>) -> Result<Datum, LispError> {
	if args.len() != 1 {
		Err(INVALID_NUMBER_OF_ARGS(args.len(), 1))
	} else if let ATOM(NUMBER(a)) = args[0] {
		Ok(ATOM(NUMBER(REAL(a.val().floor()).simplify())))
	} else {
		Err(INVALID_ARGUMENT_TYPE(args[0].clone(), "number"))
	}
}

pub fn ceil(args: Vec<Datum>) -> Result<Datum, LispError> {
	if args.len() != 1 {
		Err(INVALID_NUMBER_OF_ARGS(args.len(), 1))
	} else if let ATOM(NUMBER(a)) = args[0] {
		Ok(ATOM(NUMBER(REAL(a.val().ceil()).simplify())))
	} else {
		Err(INVALID_ARGUMENT_TYPE(args[0].clone(), "number"))
	}
}

pub fn type_lisp(args: Vec<Datum>) -> Result<Datum, LispError> {
	if args.len() != 1 {
		return Err(INVALID_NUMBER_OF_ARGS(args.len(), 1));
	}

	match args[0] {
		ATOM(SYMBOL(_)) 			=> Ok(ATOM(SYMBOL("SYMBOL".to_string()))),
		ATOM(STRING(_)) 			=> Ok(ATOM(SYMBOL("STRING".to_string()))),
		ATOM(NUMBER(RATIONAL(..)))	=> Ok(ATOM(SYMBOL("RATIONAL".to_string()))),
		ATOM(NUMBER(INTEGER(_)))	=> Ok(ATOM(SYMBOL("INTEGER".to_string()))),
		ATOM(NUMBER(REAL(_)))		=> Ok(ATOM(SYMBOL("REAL".to_string()))),
		ATOM(T)						=> Ok(ATOM(SYMBOL("BOOLEAN".to_string()))),
		LIST(CONS(..))				=> Ok(ATOM(SYMBOL("CONS".to_string()))),
		LIST(NIL)					=> Ok(ATOM(SYMBOL("NULL".to_string()))),
		FUNCTION(SPECIAL(_))		=> Ok(ATOM(SYMBOL("SPECIAL FUNCTION".to_string()))),
		FUNCTION(NATIVE(_))			=> Ok(ATOM(SYMBOL("NATIVE FUNCTION".to_string()))),
		FUNCTION(LAMBDA(_))			=> Ok(ATOM(SYMBOL("LAMBDA EXPRESSION".to_string()))),
		FUNCTION(MACRO(_))			=> Ok(ATOM(SYMBOL("MACRO".to_string())))
	}
}

pub fn is_atom(args: Vec<Datum>) -> Result<Datum, LispError> {
	if args.len() != 1 {
		Err(INVALID_NUMBER_OF_ARGS(args.len(), 1))
	} else if let ATOM(_) = args[0] {
		Ok(ATOM(T))
	} else if let LIST(NIL) = args[0] {
		Ok(ATOM(T))
	} else {
		Ok(LIST(NIL))
	}
}

pub fn is_list(args: Vec<Datum>) -> Result<Datum, LispError> {
	if args.len() != 1 {
		Err(INVALID_NUMBER_OF_ARGS(args.len(), 1))
	} else if let LIST(_) = args[0] {
		Ok(ATOM(T))
	} else {
		Ok(LIST(NIL))
	}
}

pub fn is_cons(args: Vec<Datum>) -> Result<Datum, LispError> {
	if args.len() != 1 {
		Err(INVALID_NUMBER_OF_ARGS(args.len(), 1))
	} else if let LIST(CONS(..)) = args[0] {
		Ok(ATOM(T))
	} else {
		Ok(LIST(NIL))
	}
}

pub fn is_symbol(args: Vec<Datum>) -> Result<Datum, LispError> {
	if args.len() != 1 {
		Err(INVALID_NUMBER_OF_ARGS(args.len(), 1))
	} else if let ATOM(SYMBOL(_)) = args[0] {
		Ok(ATOM(T))
	} else if let LIST(NIL) = args[0] {
		Ok(ATOM(T))
	} else {
		Ok(LIST(NIL))
	}
}

pub fn equal(args: Vec<Datum>) -> Result<Datum, LispError> {
	if args.len() != 2 {
		Err(INVALID_NUMBER_OF_ARGS(args.len(), 2))
	} else if args[0] == args[1] {
		Ok(ATOM(T))
	} else if math_equal(args) == Ok(ATOM(T)) {
		Ok(ATOM(T))
	} else {
		Ok(LIST(NIL))
	}
}

pub fn write_to_string(args: Vec<Datum>) -> Result<Datum, LispError> {
	if args.len() != 1 {
		Err(INVALID_NUMBER_OF_ARGS(args.len(), 1))
	} else {
		Ok(ATOM(STRING(format!("{}", args[0]))))
	}
}

pub fn read_from_string(args: Vec<Datum>) -> Result<Datum, LispError> {
	if args.len() != 1 {
		Err(INVALID_NUMBER_OF_ARGS(args.len(), 1))
	} else if let ATOM(STRING(ref s)) = args[0] {
		Ok(parse(&mut tokenize(s)))
	} else {
		Err(INVALID_ARGUMENT_TYPE(args[0].clone(), "string"))
	}
}

pub fn string_concat(args: Vec<Datum>) -> Result<Datum, LispError> {
	let mut ret = String::new();
	for arg in args {
		if let ATOM(STRING(s)) = arg {
			ret.push_str(s.as_ref());
		} else {
			return Err(INVALID_ARGUMENT_TYPE(arg, "string"));
		}
	}
	Ok(ATOM(STRING(ret)))
}

pub fn not(args: Vec<Datum>) -> Result<Datum, LispError> {
	if args.len() != 1 {
		Err(INVALID_NUMBER_OF_ARGS(args.len(), 1))
	} else if args[0] == LIST(NIL) {
		Ok(ATOM(T))
	} else {
		Ok(LIST(NIL))
	}
}

pub fn print(args: Vec<Datum>) -> Result<Datum, LispError> {
	if args.len() != 1 {
		Err(INVALID_NUMBER_OF_ARGS(args.len(), 1))
	} else {
		println!("{}", Red.paint(args[0].clone()));
		Ok(args[0].clone())
	}
}

pub fn most(args: Vec<Datum>) -> Result<Datum, LispError> {
	if args.len() != 1 {
		Err(INVALID_NUMBER_OF_ARGS(args.len(), 1))
	} else if let LIST(NIL) = args[0].clone() {
		Ok(LIST(NIL))
	} else if let LIST(lst) = args[0].clone() {
		let mut v = lst.get_items();
		v.pop();
		Ok(LIST(List::from_vec(v)))
	} else {
		Err(INVALID_ARGUMENT_TYPE(args[0].clone(), "list"))
	}
}

pub fn rand_int(args: Vec<Datum>) -> Result<Datum, LispError> {
	if args.len() == 1 {
		if let ATOM(NUMBER(INTEGER(n))) = args[0].clone() {
			if n > 0 {
				Ok(ATOM(NUMBER(INTEGER(rand::thread_rng().gen_range(0,n)))))
			} else {
				Err(INVALID_ARGUMENT_TYPE(args[0].clone(), "positive integer"))
			}
		} else {
			Err(INVALID_ARGUMENT_TYPE(args[0].clone(), "positive integer"))
		}
	} else if args.len() == 2 {
		if let ATOM(NUMBER(INTEGER(m))) = args[0].clone() {
			if let ATOM(NUMBER(INTEGER(n))) = args[1].clone() {
				if n > m {
					Ok(ATOM(NUMBER(INTEGER(rand::thread_rng().gen_range(m,n)))))
				} else {
					Ok(ATOM(NUMBER(INTEGER(rand::thread_rng().gen_range(n,m)))))
				}
			} else {
				Err(INVALID_ARGUMENT_TYPE(args[1].clone(), "integer"))
			}
		} else {
			Err(INVALID_ARGUMENT_TYPE(args[0].clone(), "integer"))
		}
	} else {
		Err(INVALID_NUMBER_OF_ARGS(args.len(), 1))
	}
}

pub fn rand_bool(args: Vec<Datum>) -> Result<Datum, LispError> {
	if args.len() != 0 {
		Err(INVALID_NUMBER_OF_ARGS(args.len(), 0))
	} else if rand::random() {
		Ok(LIST(NIL))
	} else {
		Ok(ATOM(T))
	}
}

pub fn rand_real(args: Vec<Datum>) -> Result<Datum, LispError> {
	if args.len() == 0 {
		Ok(ATOM(NUMBER(REAL(rand::thread_rng().gen_range(0f64,1f64)))))
	} else if args.len() == 1 {
		if let ATOM(NUMBER(n)) = args[0].clone() {
			if n.val() > 0f64 {
				Ok(ATOM(NUMBER(REAL(
					rand::thread_rng().gen_range(0f64,n.val())).simplify())))
			} else {
				Err(INVALID_ARGUMENT_TYPE(args[0].clone(), "positive number"))
			}
		} else {
			Err(INVALID_ARGUMENT_TYPE(args[0].clone(), "positive number"))
		}
	} else if args.len() == 2 {
		if let ATOM(NUMBER(m)) = args[0].clone() {
			if let ATOM(NUMBER(n)) = args[1].clone() {
				if n > m {
					Ok(ATOM(NUMBER(REAL(
						rand::thread_rng().gen_range(m.val(),n.val())).simplify())))
				} else {
					Ok(ATOM(NUMBER(REAL(
						rand::thread_rng().gen_range(n.val(),m.val())).simplify())))
				}
			} else {
				Err(INVALID_ARGUMENT_TYPE(args[1].clone(), "number"))
			}
		} else {
			Err(INVALID_ARGUMENT_TYPE(args[0].clone(), "number"))
		}
	} else {
		Err(INVALID_NUMBER_OF_ARGS(args.len(), 1))
	}
}

fn format_vec<T>(s: &str, v: Vec<T>) -> String where T: fmt::Binary + Clone {
	if let Some(pos) = s.find("{}") {
		format!("{}{:b}{}", &s[..pos], v[0], format_vec(&s[pos+2..], v[1..].to_vec()))
	} else {
		s.to_string()
	}
}

pub fn format(args: Vec<Datum>) -> Result<Datum, LispError> {
	if args.len() < 1 {
		Err(INVALID_NUMBER_OF_ARGS(args.len(), 1))
	} else if let ATOM(STRING(s)) = args[0].clone() {
		let count = s.split("{}").count();
		if count == args.len() {
			let format_args = args[1..].to_vec();
			Ok(ATOM(STRING(format_vec(&s, format_args))))
		} else {
			Err(INVALID_NUMBER_OF_FORMAT_PARAMS(args.len()-1, count-1))
		}
	} else {
		Err(INVALID_ARGUMENT_TYPE(args[0].clone(), "string"))
	}
}