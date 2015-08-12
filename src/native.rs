use errors::*;
use types::*;

use errors::LispError::*;
use types::Number::*;
use types::Datum::*;
use types::List::*;
use types::Atom::*;

pub fn add(args: List) -> Result<Datum, LispError> {
	let mut sum = INTEGER(0);
	for item in args.get_items() {
		match item {
			ATOM(NUMBER(n))	=> {sum = sum + n},
			_				=> return Err(INVALID_ARGUMENT_TYPE)
		}
	}
	Ok(ATOM(NUMBER(sum.simplify())))
}

pub fn sub(args: List) -> Result<Datum, LispError> {
	let nums = args.get_items();
	match nums[0] {
		ATOM(NUMBER(n))	=> {
			if nums.len() == 1 {
				return Ok(ATOM(NUMBER(-n)))
			}
			let res = add(List::from_vec(tail(nums)));
			if res.is_err() {
				return res;
			}
			match res.ok().unwrap() {
				ATOM(NUMBER(m))	=> Ok(ATOM(NUMBER((n-m).simplify()))),
				_				=> Err(INVALID_ARGUMENT_TYPE)
			}
		},
		_				=> Err(INVALID_ARGUMENT_TYPE)
	}
}

pub fn mul(args: List) -> Result<Datum, LispError> {
	let mut prd = INTEGER(1);
	for item in args.get_items() {
		match item {
			ATOM(NUMBER(n))	=> {prd = prd * n},
			_				=> return Err(INVALID_ARGUMENT_TYPE)
		}
	}
	Ok(ATOM(NUMBER(prd.simplify())))
}

pub fn div(args: List) -> Result<Datum, LispError> {
	let nums = args.get_items();
	match nums[0] {
		ATOM(NUMBER(n))	=> {
			if nums.len() == 1 {
				return Ok(ATOM(NUMBER(INTEGER(1)/n)))
			}
			let res = mul(List::from_vec(tail(nums)));
			if res.is_err() {
				return res;
			}
			match res.ok().unwrap() {
				ATOM(NUMBER(m))	=> Ok(ATOM(NUMBER((n/m).simplify()))),
				_				=> Err(INVALID_ARGUMENT_TYPE)
			}
		},
		_				=> Err(INVALID_ARGUMENT_TYPE)
	}
}

pub fn list(args: List) -> Result<Datum, LispError> {
	Ok(LIST(args))
}

pub fn cons(args: List) -> Result<Datum, LispError> {
	let lst = args.get_items();
	match lst.len() {
		0|1 => Err(NOT_ENOUGH_ARGUMENTS),
		2 	=> Ok(LIST(CONS(Box::new(lst[0].clone()), Box::new(lst[1].clone())))),
		_	=> Err(TOO_MANY_ARGUMENTS)
	}
}

pub fn car(args: List) -> Result<Datum, LispError> {
	let lst = args.get_items();
	match lst.len() {
		0 => Err(NOT_ENOUGH_ARGUMENTS),
		1 => {
			match lst[0] {
				LIST(ref l)	=> Ok(l.car()),
				_			=> Err(INVALID_ARGUMENT_TYPE)
			}
		},
		_ => Err(TOO_MANY_ARGUMENTS)
	}
}

pub fn cdr(args: List) -> Result<Datum, LispError> {
	let lst = args.get_items();
	match lst.len() {
		0 => Err(NOT_ENOUGH_ARGUMENTS),
		1 => {
			match lst[0] {
				LIST(ref l)	=> Ok(l.cdr()),
				_			=> Err(INVALID_ARGUMENT_TYPE)
			}
		},
		_ => Err(TOO_MANY_ARGUMENTS)
	}	
}

pub fn nth(args: List) -> Result<Datum, LispError> {
	let lst = args.get_items();
	match lst.len() {
		0 | 1 => Err(NOT_ENOUGH_ARGUMENTS),
		2 => {
			if let ATOM(NUMBER(INTEGER(n))) = lst[0] {
				let n = n as usize;
				match lst[1] {
					LIST(ref l)	=> return if l.get_items().len() > n {
						Ok(l.get_items()[n].clone())
					} else {Ok(LIST(NIL))},
					_		=> return Err(INVALID_ARGUMENT_TYPE)
				}
			}
			Err(INVALID_ARGUMENT_TYPE)
		},
		_ => Err(TOO_MANY_ARGUMENTS)
	}
}

pub fn nth_cdr(args: List) -> Result<Datum, LispError> {
	let lst = args.get_items();
	match lst.len() {
		0 | 1 => Err(NOT_ENOUGH_ARGUMENTS),
		2 => {
			if let ATOM(NUMBER(INTEGER(n))) = lst[0] {
				let n = n as usize;
				match lst[1] {
					LIST(ref l)	=> return if l.get_items().len() > n {
						Ok(LIST(List::from_vec(
							l.get_items().into_iter().skip(n).collect())))
					} else {Ok(LIST(NIL))},
					_		=> return Err(INVALID_ARGUMENT_TYPE)
				}
			}
			Err(INVALID_ARGUMENT_TYPE)
		},
		_ => Err(TOO_MANY_ARGUMENTS)
	}
}

pub fn greater_than(args: List) -> Result<Datum, LispError> {
	let nums = args.get_items();
	for i in 1..nums.len() {
		if let ATOM(NUMBER(a)) = nums[i-1] {
			if let ATOM(NUMBER(b)) = nums[i] {
				if a <= b {
					return Ok(LIST(NIL));
				}
			} else {
				return Err(INVALID_ARGUMENT_TYPE);
			}
		} else {
			return Err(INVALID_ARGUMENT_TYPE);
		}
	}
	Ok(ATOM(T))
}

pub fn greater_equal(args: List) -> Result<Datum, LispError> {
	let nums = args.get_items();
	for i in 1..nums.len() {
		if let ATOM(NUMBER(a)) = nums[i-1] {
			if let ATOM(NUMBER(b)) = nums[i] {
				if a < b {
					return Ok(LIST(NIL));
				}
			} else {
				return Err(INVALID_ARGUMENT_TYPE);
			}
		} else {
			return Err(INVALID_ARGUMENT_TYPE);
		}
	}
	Ok(ATOM(T))
}

pub fn less_than(args: List) -> Result<Datum, LispError> {
	let nums = args.get_items();
	for i in 1..nums.len() {
		if let ATOM(NUMBER(a)) = nums[i-1] {
			if let ATOM(NUMBER(b)) = nums[i] {
				if a >= b {
					return Ok(LIST(NIL));
				}
			} else {
				return Err(INVALID_ARGUMENT_TYPE);
			}
		} else {
			return Err(INVALID_ARGUMENT_TYPE);
		}
	}
	Ok(ATOM(T))
}

pub fn less_equal(args: List) -> Result<Datum, LispError> {
	let nums = args.get_items();
	for i in 1..nums.len() {
		if let ATOM(NUMBER(a)) = nums[i-1] {
			if let ATOM(NUMBER(b)) = nums[i] {
				if a > b {
					return Ok(LIST(NIL));
				}
			} else {
				return Err(INVALID_ARGUMENT_TYPE);
			}
		} else {
			return Err(INVALID_ARGUMENT_TYPE);
		}
	}
	Ok(ATOM(T))
}

pub fn math_equal(args: List) -> Result<Datum, LispError> {
	let nums = args.get_items();
	for i in 1..nums.len() {
		if let ATOM(NUMBER(a)) = nums[i-1] {
			if let ATOM(NUMBER(b)) = nums[i] {
				if a != b {
					return Ok(LIST(NIL));
				}
			} else {
				return Err(INVALID_ARGUMENT_TYPE);
			}
		} else {
			return Err(INVALID_ARGUMENT_TYPE);
		}
	}
	Ok(ATOM(T))
}

pub fn lisp_mod(args: List) -> Result<Datum, LispError> {
	let nums = args.get_items();
	if nums.len() < 2 {
		return Err(NOT_ENOUGH_ARGUMENTS);
	} else if nums.len() > 2 {
		return Err(TOO_MANY_ARGUMENTS);
	}

	if let ATOM(NUMBER(a)) = nums[0] {
		if let ATOM(NUMBER(b)) = nums[1] {
			return Ok(ATOM(NUMBER(
					a - b*INTEGER((a/b).val().floor() as i64))));
		} else {
			return Err(INVALID_ARGUMENT_TYPE);
		}
	} else {
		return Err(INVALID_ARGUMENT_TYPE);
	}
}