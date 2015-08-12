use errors::*;
use native::*;
use types::*;
use env::*;

use errors::LispError::*;
use types::Function::*;
use types::Special::*;
use types::Native::*;
use types::Datum::*;
use types::List::*;
use types::Atom::*;

pub fn eval(form: &Datum, env: &mut Env) -> Result<Datum, LispError> {
	match *form {
		LIST(ref l)	=> eval_list(l, env),
		ATOM(ref a) => eval_atom(a, env),
		_			=> Err(INVALID_FORM)
	}
}

fn eval_list(form: &List, env: &mut Env) -> Result<Datum, LispError> {
	match *form {
		CONS(ref car, ref cdr) 	=> {
			let f = eval(car, env);
			if f.is_err() {
				return f;
			} match f.ok().unwrap() {
				FUNCTION(func) 	=> {
					match *cdr.clone() {
						LIST(args) 	=> apply(func, args, env),
						_			=> Err(INVALID_ARG_LIST)
					}
				},
				_				=> Err(UNKNOWN_FUNCTION)
			} 
		},
		NIL						=> Ok(LIST(NIL))
	}
}

fn eval_atom(form: &Atom, env: &mut Env) -> Result<Datum, LispError> {
	match *form {
		SYMBOL(ref a) 	=> env.get(a),
		ref e @ _		=> Ok(ATOM(e.clone()))
	}
}

fn apply(func: Function, args: List, env: &mut Env) -> Result<Datum, LispError> {
	match func {
		SPECIAL(ref s)	=> apply_special(s, args, env),
		NATIVE(ref n)	=> apply_native(n, args, env),
		LAMBDA(ref l)	=> apply_lambda(l, args, env)
	}
}

fn apply_native(func: &Native, args: List, env: &mut Env) -> Result<Datum, LispError> {
	let mut items = args.get_items();
	for i in 0..items.len() {
		let res = eval(&items[i], env);
		if res.is_err() {
			return res;
		} else {
			items[i] = res.ok().unwrap();
		}
	}

	match *func {
		ADD			=> add(List::from_vec(items)),
		SUB			=> sub(List::from_vec(items)),
		MUL			=> mul(List::from_vec(items)),
		DIV			=> div(List::from_vec(items)),
		LIST_FUNC 	=> list(List::from_vec(items)),
		CONS_FUNC	=> cons(List::from_vec(items)),
		CAR 		=> car(List::from_vec(items)),
		CDR			=> cdr(List::from_vec(items)),
		NTH_CDR		=> nth_cdr(List::from_vec(items)),
		NTH			=> nth(List::from_vec(items)),
		GT 			=> greater_than(List::from_vec(items)),
		GE 			=> greater_equal(List::from_vec(items)),
		LT 			=> less_than(List::from_vec(items)),
		LE 			=> less_equal(List::from_vec(items)),
		MATH_EQ     => math_equal(List::from_vec(items)),
		MOD			=> lisp_mod(List::from_vec(items)),
		//_			=> Err(NOT_YET_IMPLEMENTED)
	}
}

fn apply_special(func: &Special, args: List, env: &mut Env) -> Result<Datum, LispError> {
	match *func {
		DEFINE		=> define(args, env),
		IF 			=> lisp_if(args, env),
		LAMBDA_FUNC => lambda(args),
		DEFUN 		=> defun(args, env),
		QUOTE 		=> quote(args),
		BACKQUOTE   => backquote(args, env),
		_			=> Err(NOT_YET_IMPLEMENTED)
	}
}

fn apply_lambda(func: &Lambda, args: List, env: &mut Env) -> Result<Datum, LispError> {
	let mut params = args.get_items();

	if params.len() < func.args.len() {
		return Err(NOT_ENOUGH_ARGUMENTS);
	} else if params.len() > func.args.len() {
		return Err(TOO_MANY_ARGUMENTS);
	}

	for i in 0..params.len() {
		let res = eval(&params[i], env);
		if res.is_err() {
			return res;
		} else {
			params[i] = res.ok().unwrap();
		}
	}

	env.push();
	for param_arg in params.into_iter().zip(&func.args) {
		env.set(param_arg.1.clone(), param_arg.0);
	}
	let res = eval(&func.body, env);
	env.pop();

	return res;
}

fn define(args: List, env: &mut Env) -> Result<Datum, LispError> {
	let lst = args.get_items();
	match lst.len() {
		0 | 1 => Err(NOT_ENOUGH_ARGUMENTS),
		2 => {
			if let ATOM(SYMBOL(sym)) = lst[0].clone() {
				let res = eval(&lst[1], env);
				if res.is_err() {
					return res;
				} else {
					match env.get(&sym) {
						Ok(FUNCTION(SPECIAL(_))) |
						Ok(FUNCTION(NATIVE(_))) => return Err(INVALID_ARGUMENT_TYPE),
						_						=> return Ok(env.set(sym, res.ok().unwrap()))
					}
				}
			}
			Err(INVALID_ARGUMENT_TYPE)
		},
		_ => Err(TOO_MANY_ARGUMENTS)
	}
}

fn is_true(cond: Datum) -> bool {
	match cond {
		LIST(NIL)	=> false,
		_			=> true
	}
}

fn lisp_if(args: List, env: &mut Env) -> Result<Datum, LispError> {
	let lst = args.get_items();
	match lst.len() {
		0 | 1 	=> Err(NOT_ENOUGH_ARGUMENTS),
		2 | 3 	=> {
			let res = eval(&lst[0], env);
			if res.is_err() {
				res
			} else if is_true(res.ok().unwrap()) {
				eval(&lst[1], env)
			} else if lst.len() == 3 {
				eval(&lst[2], env)
			} else {
				Ok(LIST(NIL))
			}
		},
		_	 	=> Err(TOO_MANY_ARGUMENTS)
	}
}

fn lambda(args: List) -> Result<Datum, LispError> {
	let lst = args.get_items();
	if lst.len() < 2 {
		return Err(NOT_ENOUGH_ARGUMENTS);
	} else if lst.len() > 2 {
		return Err(TOO_MANY_ARGUMENTS);
	}

	if let LIST(params) = lst[0].clone() {
		let mut args: Vec<String> = vec![];
		let params = params.get_items();

		for param in params {
			if let ATOM(SYMBOL(name)) = param {
				args.push(name);
			} else {
				return Err(INVALID_ARGUMENT_TYPE);
			}
		}

		let func = Lambda{args: args, body: Box::new(lst[1].clone())};
		return Ok(FUNCTION(LAMBDA(func)));
	} else {
		return Err(INVALID_ARGUMENT_TYPE);
	}
}

fn defun(args: List, env: &mut Env) -> Result<Datum, LispError> {
	let lst = args.get_items();
	if lst.len() < 3 {
		return Err(NOT_ENOUGH_ARGUMENTS);
	} else if lst.len() > 3 {
		return Err(TOO_MANY_ARGUMENTS);
	}

	let lam = lambda(List::from_vec(vec!(lst[1].clone(), lst[2].clone())));
	if lam.is_err() {
		return lam;
	} else if let ATOM(SYMBOL(name)) = lst[0].clone() {
		match env.get(&name) {
			Ok(FUNCTION(SPECIAL(_))) |
			Ok(FUNCTION(NATIVE(_))) => return Err(INVALID_ARGUMENT_TYPE),
			_						=> return Ok(env.set(name, lam.ok().unwrap()))
		}
	} else {
		return Err(INVALID_ARGUMENT_TYPE)
	}
}

fn quote(args: List) -> Result<Datum, LispError> {
	let lst = args.get_items();
	if lst.len() < 1 {
		Err(NOT_ENOUGH_ARGUMENTS)
	} else if lst.len() > 1 {
		Err(TOO_MANY_ARGUMENTS)
	} else {
		Ok(lst[0].clone())
	}
}

fn backquote(args: List, env: &mut Env) -> Result<Datum, LispError> {
	let lst = args.get_items();
	if lst.len() < 1 {
		Err(NOT_ENOUGH_ARGUMENTS)
	} else if lst.len() > 1 {
		Err(TOO_MANY_ARGUMENTS)
	} else {
		backquote_helper(&lst[0], env)
	}
}

fn backquote_helper(arg: &Datum, env: &mut Env) -> Result<Datum, LispError> {
	match *arg {
		ref e @ ATOM(_) | ref e @ FUNCTION(_) 	=> Ok(e.clone()),
		LIST(ref lst)							=> {
			if lst.car() == ATOM(SYMBOL("COMMA".to_string())) {
				eval(&lst.get_items()[1], env) //assumes list is of form (COMMA item)
			} else {
				let mut items = lst.get_items();
				for i in 0..items.len() {
					let res = eval(&items[i], env);
					if res.is_err() {
						return res;
					} else {
						items[i] = res.ok().unwrap();
					}
				}

				Ok(LIST(List::from_vec(items)))
			}
		}
	}
}