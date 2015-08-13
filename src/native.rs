use errors::*;
use types::*;

use errors::LispError::*;
use types::Number::*;
use types::Datum::*;
use types::List::*;
use types::Atom::*;

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
	match args[0] {
		ATOM(NUMBER(n))	=> {
			if args.len() == 1 {
				return Ok(ATOM(NUMBER(-n)))
			}
			let res = add(tail(args));
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
	match args[0] {
		ATOM(NUMBER(n))	=> {
			if args.len() == 1 {
				return Ok(ATOM(NUMBER(INTEGER(1)/n)))
			}
			let res = mul(tail(args));
			if res.is_err() {
				return res;
			}
			match res.ok().unwrap() {
				ATOM(NUMBER(m))	=> Ok(ATOM(NUMBER((n/m).simplify()))),
				ref e @ _		=> Err(INVALID_ARGUMENT_TYPE(e.clone(), "number"))
			}
		},
		_				=> Err(INVALID_ARGUMENT_TYPE(args[0].clone(), "number"))
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
				if a != b {
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
		return Err(INVALID_NUMBER_OF_ARGS(args.len(), 2));
	} 

	if let ATOM(NUMBER(a)) = args[0] {
		if let ATOM(NUMBER(b)) = args[1] {
			return Ok(ATOM(NUMBER(
					a - b*INTEGER((a/b).val().floor() as i64))));
		} else {
			return Err(INVALID_ARGUMENT_TYPE(args[1].clone(), "number"));
		}
	} else {
		return Err(INVALID_ARGUMENT_TYPE(args[0].clone(), "number"));
	}
}